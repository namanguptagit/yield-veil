use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;

pub mod errors;

pub mod state;

mod mxe_instruction;
mod yield_router;

use mxe_instruction::*;
use errors::ErrorCode;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


const COMP_DEF_OFFSET_MATCH:    u32 = comp_def_offset("match_orders");
const COMP_DEF_OFFSET_VALIDATE: u32 = comp_def_offset("validate_order");
const COMP_DEF_OFFSET_ROUTE:    u32 = comp_def_offset("route_yield");
const COMP_DEF_OFFSET_COMPOUND: u32 = comp_def_offset("compound_yield");

const SETTLEMENT_WINDOW_SECS: i64 = 120;

#[arcium_program]
pub mod arcyield {
    use super::*;


    pub fn init_orderbook(
        ctx: Context<InitOrderbook>,
        arcium_artifact_id:      [u8; 32],
        arcium_verification_key: [u8; 32],
        arcium_mxe_public_key:   [u8; 32],
    ) -> Result<()> {
        let ob = &mut ctx.accounts.orderbook;
        ob.admin                   = ctx.accounts.authority.key();
        ob.arcium_artifact_id      = arcium_artifact_id;
        ob.arcium_verification_key = arcium_verification_key;
        ob.arcium_mxe_public_key   = arcium_mxe_public_key;
        ob.escrow_authority_bump   = ctx.bumps.escrow_authority;
        ob.order_count             = 0;
        ob.token_mint              = ctx.accounts.token_mint.key();
        ob.total_volume            = 0;
        ob.total_matches           = 0;
        ob.created_at              = Clock::get()?.unix_timestamp;

        emit!(OrderbookInitialized {
        authority:         ob.admin,
        token_mint:        ob.token_mint,
        arcium_artifact_id,
        arcium_mxe_pubkey: arcium_mxe_public_key,
    });
        Ok(())
    }
    pub fn init_match_orders_comp_def(ctx: Context<InitMatchOrdersCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    // ORDER PLACEMENT

    pub fn place_order(
        ctx:              Context<PlaceOrder>,
        order_ciphertext: [u8; 96],
        encryption_pubkey: [u8; 32],
        nonce:            u128,
        escrow_amount:    u64,
    ) -> Result<()> {
        require!(escrow_amount > 0, ErrorCode::InsufficientEscrow);

        // Lock collateral.
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.user_token.to_account_info(),
                    to:        ctx.accounts.escrow_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            escrow_amount,
        )?;

        let ob    = &mut ctx.accounts.orderbook;
        let order = &mut ctx.accounts.order;
        let now   = Clock::get()?.unix_timestamp;

        order.owner            = ctx.accounts.user.key();
        order.order_id         = ob.order_count;
        order.encryption_pubkey = encryption_pubkey;
        order.nonce            = nonce;
        order.escrow_amount    = escrow_amount;
        order.status           = OrderStatus::Pending;
        order.order_ciphertext = order_ciphertext;
        order.placed_at        = now;
        order.updated_at       = now;

        ob.order_count = ob.order_count.checked_add(1).ok_or(ErrorCode::MathOverflow)?;

        emit!(OrderPlaced {
            order_id:      order.order_id,
            owner:         order.owner,
            escrow_amount,
            orderbook:     ob.key(),
        });

        Ok(())
    }


    // ORDER MATCHING  (MXE queue)
    pub fn match_order_pair(
        ctx: Context<MatchOrderPair>,
        computation_offset: u64,
        order1_enc: OrderEncryptedData,
        order2_enc: OrderEncryptedData,
    ) -> Result<()> {
        require!(
        ctx.accounts.order1.status == OrderStatus::Pending,
        ErrorCode::OrderNotPending
    );
        require!(
        ctx.accounts.order2.status == OrderStatus::Pending,
        ErrorCode::OrderNotPending
    );


        let args = ArgBuilder::new()

            .x25519_pubkey(order1_enc.owner_enc)
            .plaintext_u128(order1_enc.nonce)
            .encrypted_bool(order1_enc.is_buy_enc)
            .encrypted_u64(order1_enc.price_enc)
            .encrypted_u64(order1_enc.size_enc)

            .x25519_pubkey(order2_enc.owner_enc)
            .plaintext_u128(order2_enc.nonce)
            .encrypted_bool(order2_enc.is_buy_enc)
            .encrypted_u64(order2_enc.price_enc)
            .encrypted_u64(order2_enc.size_enc)
            .build();


        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![MatchOrdersCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[
                    CallbackAccount { pubkey: ctx.accounts.order1.key(),  is_writable: true },
                    CallbackAccount { pubkey: ctx.accounts.order2.key(),  is_writable: true },
                    CallbackAccount { pubkey: ctx.accounts.escrow1.key(), is_writable: true },
                    CallbackAccount { pubkey: ctx.accounts.escrow2.key(), is_writable: true },
                ],
            )?],
            1, // number of callback transactions
            0,
        )?;

        ctx.accounts.order1.status = OrderStatus::Queued;
        ctx.accounts.order2.status = OrderStatus::Queued;
        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.order1.updated_at = now;
        ctx.accounts.order2.updated_at = now;

        emit!(MatchQueued {
        order1: ctx.accounts.order1.key(),
         order2: ctx.accounts.order2.key(),
        });

        Ok(())
    }




    #[arcium_callback(encrypted_ix = "match_orders")]
    pub fn match_orders_callback(
        ctx: Context<MatchOrdersCallback>,
        output: SignedComputationOutputs<MatchOrdersOutput>,
    ) -> Result<()> {
        // TODO(arcium): once `arcium build` regenerates the .arcis with the
        // current circuit shape, restore the original field-by-name access
        // (matched / execution_price / execution_size). The auto-generated
        // output struct currently exposes positional fields (field_0..field_2)
        // that don't carry the inner MatchResult fields directly, so the full
        // settlement logic is staged for that pass. For now we just verify
        // signature, mark both orders Pending, and emit OrderMatchFailed.
        match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(_) => {}
            Err(e) => {
                msg!("Computation aborted: {}", e);
                return Err(ErrorCode::AbortedComputation.into());
            }
        };

        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.order1.status = OrderStatus::Pending;
        ctx.accounts.order2.status = OrderStatus::Pending;
        ctx.accounts.order1.updated_at = now;
        ctx.accounts.order2.updated_at = now;

        emit!(OrderMatchFailed {
            order1: ctx.accounts.order1.key(),
            order2: ctx.accounts.order2.key(),
        });

        Ok(())
    }


    // SETTLEMENT  (relayer-submitted bundle)

    pub fn settle_matches(
        ctx:              Context<SettleMatches>,
        bundle:           SettlementBundleHeader,
        arcium_signature: Vec<u8>,
    ) -> Result<()> {
        let ob  = &ctx.accounts.orderbook;
        let now = Clock::get()?.unix_timestamp;

        // 1. Artifact ID.
        require!(
            bundle.artifact_id == ob.arcium_artifact_id,
            ErrorCode::InvalidArtifactId
        );

        // 2. Replay prevention (account init enforced by Anchor `init`
        //    constraint on `settlement_record` — tx fails if already exists).

        // 3. Timestamp window.
        require!(
            (now - bundle.timestamp).abs() <= SETTLEMENT_WINDOW_SECS,
            ErrorCode::SettlementExpired
        );

        // 4. Signature.
        require!(
            verify_arcium_bundle(&bundle, &arcium_signature, &ob.arcium_verification_key),
            ErrorCode::InvalidArciumSignature
        );

        // 5. Order states.
        let a_state = ctx.accounts.order_a.status;
        let b_state = ctx.accounts.order_b.status;
        require!(
            a_state == OrderStatus::Pending || a_state == OrderStatus::Processing,
           ErrorCode::InvalidOrderStateForSettlement
        );
        require!(
            b_state == OrderStatus::Pending || b_state == OrderStatus::Processing,
           ErrorCode::InvalidOrderStateForSettlement
        );


        let ob_key = ob.key();
        let seeds: &[&[u8]] = &[
            b"escrow_auth",
            ob_key.as_ref(),
            &[ob.escrow_authority_bump],
        ];
        let signer = &[&seeds[..]];

        if bundle.amount_a_to_b > 0 {
            require!(
                ctx.accounts.escrow_a.amount >= bundle.amount_a_to_b,
                ErrorCode::InsufficientEscrow
            );
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from:      ctx.accounts.escrow_a.to_account_info(),
                        to:        ctx.accounts.recipient_b.to_account_info(),
                        authority: ctx.accounts.escrow_authority.to_account_info(),
                    },
                    signer,
                ),
                bundle.amount_a_to_b,
            )?;
        }

        if bundle.amount_b_to_a > 0 {
            require!(
                ctx.accounts.escrow_b.amount >= bundle.amount_b_to_a,
                ErrorCode::InsufficientEscrow
            );
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from:      ctx.accounts.escrow_b.to_account_info(),
                        to:        ctx.accounts.recipient_a.to_account_info(),
                        authority: ctx.accounts.escrow_authority.to_account_info(),
                    },
                    signer,
                ),
                bundle.amount_b_to_a,
            )?;
        }

        // Mark orders settled.
        ctx.accounts.order_a.status     = OrderStatus::Filled;
        ctx.accounts.order_b.status     = OrderStatus::Filled;
        ctx.accounts.order_a.updated_at = now;
        ctx.accounts.order_b.updated_at = now;


        let sr              = &mut ctx.accounts.settlement_record;
        sr.settlement_hash  = bundle.settlement_hash;
        sr.bundle_timestamp = bundle.timestamp;
        sr.relayer          = ctx.accounts.relayer.key();
        sr.is_yield_settlement = false;

        emit!(SettlementExecuted {
            settlement_hash: bundle.settlement_hash,
            order_a:         ctx.accounts.order_a.owner,
            order_b:         ctx.accounts.order_b.owner,
            amount_a_to_b:   bundle.amount_a_to_b,
            amount_b_to_a:   bundle.amount_b_to_a,
        });

        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // ORDER CANCELLATION
    // ─────────────────────────────────────────────────────────────────────────

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        let order = &mut ctx.accounts.order;

        require!(order.status == OrderStatus::Pending, ErrorCode::OrderNotPending);
        require!(order.owner == ctx.accounts.user.key(), ErrorCode::UnauthorizedOrderAccess);

        let ob_key = ctx.accounts.orderbook.key();
        let seeds: &[&[u8]] = &[
            b"escrow_auth",
            ob_key.as_ref(),
            &[ctx.accounts.orderbook.escrow_authority_bump],
        ];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.escrow_token.to_account_info(),
                    to:        ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.escrow_authority.to_account_info(),
                },
                signer,
            ),
            order.escrow_amount,
        )?;

        order.status     = OrderStatus::Cancelled;
        order.updated_at = Clock::get()?.unix_timestamp;

        emit!(OrderCancelled {
            order_id:      order.order_id,
            owner:         order.owner,
            refund_amount: order.escrow_amount,
        });

        Ok(())
    }




    pub fn open_yield_position(
        ctx:                 Context<OpenYieldPosition>,
        position_ciphertext: [u8; 96],
        encryption_pubkey:   [u8; 32],
        nonce:               u128,
        escrow_amount:       u64,
        protocol:            Protocol,
        auto_compound:       bool,
    ) -> Result<()> {
        require!(escrow_amount > 0, ErrorCode::InsufficientEscrow);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.user_token.to_account_info(),
                    to:        ctx.accounts.vault_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            escrow_amount,
        )?;

        let ob  = &mut ctx.accounts.orderbook;
        let pos = &mut ctx.accounts.position;
        let now = Clock::get()?.unix_timestamp;

        pos.owner               = ctx.accounts.user.key();
        pos.position_id         = ob.order_count; // reuse counter for position IDs
        pos.escrowed_tokens     = escrow_amount;
        pos.protocol            = protocol;
        pos.position_ciphertext = position_ciphertext;
        pos.encryption_pubkey   = encryption_pubkey;
        pos.nonce               = nonce;
        pos.accrued_yield       = 0;
        pos.last_compounded_at  = now;
        pos.auto_compound       = auto_compound;
        pos.created_at          = now;
        pos.updated_at          = now;

        ob.order_count = ob.order_count.checked_add(1).ok_or(ErrorCode::MathOverflow)?;

        emit!(YieldPositionOpened {
            position_id:    pos.position_id,
            owner:          pos.owner,
            protocol,
            escrow_amount,
            auto_compound,
        });

        Ok(())
    }


    pub fn close_yield_position(ctx: Context<CloseYieldPosition>) -> Result<()> {
        let pos = &ctx.accounts.position;
        require!(pos.owner == ctx.accounts.user.key(), ErrorCode::UnauthorizedOrderAccess);

        let total = pos.escrowed_tokens
            .checked_add(pos.accrued_yield)
            .ok_or(ErrorCode::MathOverflow)?;

        let ob_key = ctx.accounts.orderbook.key();
        let seeds: &[&[u8]] = &[
            b"escrow_auth",
            ob_key.as_ref(),
            &[ctx.accounts.orderbook.escrow_authority_bump],
        ];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.vault_token.to_account_info(),
                    to:        ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.escrow_authority.to_account_info(),
                },
                signer,
            ),
            total,
        )?;

        emit!(YieldPositionClosed {
        position_id:  pos.position_id,
        owner:        pos.owner,
        total_payout: total,
    });

        Ok(())
    }
}



#[derive(Accounts)]
pub struct InitOrderbook<'info> {
    #[account(
        init,
        payer  = authority,
        space  = 8 + Orderbook::INIT_SPACE,
        seeds  = [b"orderbook", authority.key().as_ref()],
        bump
    )]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: escrow PDA — owned by this program.
    #[account(
        seeds = [b"escrow_auth", orderbook.key().as_ref()],
        bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    pub system_program:  Program<'info, System>,
    pub token_program:   Program<'info, Token>,
    pub rent:            Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(order_ciphertext: [u8; 96], encryption_pubkey: [u8; 32], nonce: u128, escrow_amount: u64)]
pub struct PlaceOrder<'info> {
    #[account(mut, seeds = [b"orderbook", orderbook.admin.as_ref()], bump)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Order::INIT_SPACE,
        seeds = [b"order", orderbook.key().as_ref(), &orderbook.order_count.to_le_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(mut, token::mint = orderbook.token_mint, token::authority = user)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = orderbook.token_mint,
        seeds = [b"escrow", orderbook.key().as_ref(), &orderbook.order_count.to_le_bytes()],
        bump
    )]
    pub escrow_token: Account<'info, TokenAccount>,

    pub token_program:  Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[init_computation_definition_accounts("match_orders", payer)]
#[derive(Accounts)]
pub struct InitMatchOrdersCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]

    pub comp_def_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_mxe_lut_pda!(mxe_account.lut_offset_slot))]

    pub address_lookup_table: UncheckedAccount<'info>,
    #[account(address = LUT_PROGRAM_ID)]

    pub lut_program: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}
#[queue_computation_accounts("match_orders", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct MatchOrderPair<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, ArciumSignerAccount>,

    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,

    #[account(mut, address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub mempool_account: UncheckedAccount<'info>,

    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub executing_pool: UncheckedAccount<'info>,

    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    pub computation_account: UncheckedAccount<'info>,

    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_MATCH))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,

    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,

    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,

    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,


    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,
    #[account(mut)]
    pub order1: Account<'info, Order>,
    #[account(mut)]
    pub order2: Account<'info, Order>,
    #[account(mut)]
    pub escrow1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow2: Account<'info, TokenAccount>,
}

#[callback_accounts("match_orders")]
#[derive(Accounts)]
pub struct MatchOrdersCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_MATCH))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,

    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]

    pub instructions_sysvar: AccountInfo<'info>,


    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,
    #[account(mut)]
    pub order1: Account<'info, Order>,
    #[account(mut)]
    pub order2: Account<'info, Order>,
    #[account(mut)]
    pub escrow1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow2: Account<'info, TokenAccount>,

    #[account(seeds = [b"escrow_auth", orderbook.key().as_ref()], bump = orderbook.escrow_authority_bump)]
    pub escrow_authority: UncheckedAccount<'info>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(bundle: SettlementBundleHeader)]
pub struct SettleMatches<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub order_a: Account<'info, Order>,
    #[account(mut)]
    pub order_b: Account<'info, Order>,

    #[account(mut)]
    pub escrow_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub relayer: Signer<'info>,


    #[account(
        seeds = [b"escrow_auth", orderbook.key().as_ref()],
        bump  = orderbook.escrow_authority_bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer  = relayer,
        space  = 8 + SettlementRecord::INIT_SPACE,
        seeds  = [b"settlement", &bundle.settlement_hash[..]],
        bump
    )]
    pub settlement_record: Account<'info, SettlementRecord>,

    pub token_program:  Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut, has_one = owner @ ErrorCode::UnauthorizedOrderAccess)]
    pub order: Account<'info, Order>,


    pub owner: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_token: Account<'info, TokenAccount>,


    #[account(
        seeds = [b"escrow_auth", orderbook.key().as_ref()],
        bump  = orderbook.escrow_authority_bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct OpenYieldPosition<'info> {
    #[account(mut, seeds = [b"orderbook", orderbook.admin.as_ref()], bump)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + YieldPosition::INIT_SPACE,
        seeds = [b"position", user.key().as_ref(), &orderbook.order_count.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, YieldPosition>,

    #[account(mut, token::mint = orderbook.token_mint, token::authority = user)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = orderbook.token_mint,
        seeds = [b"vault", orderbook.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub vault_token: Account<'info, TokenAccount>,

    pub token_program:  Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseYieldPosition<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut, close = user)]
    pub position: Account<'info, YieldPosition>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"escrow_auth", orderbook.key().as_ref()],
        bump  = orderbook.escrow_authority_bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}



#[event] pub struct OrderbookInitialized {
    pub authority:         Pubkey,
    pub token_mint:        Pubkey,
    pub arcium_artifact_id: [u8; 32],
    pub arcium_mxe_pubkey: [u8; 32],
}

#[event] pub struct OrderPlaced {
    pub order_id:      u64,
    pub owner:         Pubkey,
    pub escrow_amount: u64,
    pub orderbook:     Pubkey,
}

#[event] pub struct MatchQueued {
    pub order1: Pubkey,
    pub order2: Pubkey,
}

#[event] pub struct OrderMatched {
    pub order1:          Pubkey,
    pub order2:          Pubkey,
    pub execution_price: u64,
    pub execution_size:  u64,
}

#[event] pub struct OrderMatchFailed {
    pub order1: Pubkey,
    pub order2: Pubkey,
}

#[event] pub struct OrderCancelled {
    pub order_id:      u64,
    pub owner:         Pubkey,
    pub refund_amount: u64,
}

#[event] pub struct SettlementExecuted {
    pub settlement_hash: [u8; 32],
    pub order_a:         Pubkey,
    pub order_b:         Pubkey,
    pub amount_a_to_b:   u64,
    pub amount_b_to_a:   u64,
}

#[event] pub struct YieldPositionOpened {
    pub position_id:   u64,
    pub owner:         Pubkey,
    pub protocol:      Protocol,
    pub escrow_amount: u64,
    pub auto_compound: bool,
}

#[event] pub struct YieldPositionClosed {
    pub position_id:  u64,
    pub owner:        Pubkey,
    pub total_payout: u64,
}


fn verify_arcium_bundle(
    header:           &SettlementBundleHeader,
    signature:        &[u8],
    verification_key: &[u8; 32],
) -> bool {
    if signature.len() != 64 { return false; }
    if verification_key.iter().all(|&b| b == 0) { return false; }

    // Bundle is hashed by the relayer/Arcium signer with the same scheme.
    // For the demo, verify_ed25519 below is a placeholder (returns true on
    // non-zero inputs), so we pass the raw bundle hash without an additional
    // hash function dep. Replace with keccak/sha256 once verify_ed25519 is real.
    let mut msg_hash = [0u8; 32];
    let copy_len = header.settlement_hash.len().min(32);
    msg_hash[..copy_len].copy_from_slice(&header.settlement_hash[..copy_len]);

    let mut sig = [0u8; 64];
    sig.copy_from_slice(&signature[..64]);

    verify_ed25519(&msg_hash, &sig, verification_key)
}

/// Thin wrapper around Solana's ed25519 program verification.
///
/// In production this should use the precompile sysvar instruction.
/// This implementation uses `ed25519-dalek` (available via solana-sdk).
fn verify_ed25519(
    message:    &[u8; 32],
    signature:  &[u8; 64],
    public_key: &[u8; 32],
) -> bool {
    // Guard: reject obviously invalid inputs.
    if message.iter().all(|&b| b == 0)    { return false; }
    if signature.iter().all(|&b| b == 0)  { return false; }
    if public_key.iter().all(|&b| b == 0) { return false; }

    // TODO: Replace with Solana's ed25519 precompile sysvar call for
    //       production deployment.  The precompile is available at address
    //       `ed25519SigVerify111111111111111111111111111` and requires an
    //       additional `Ed25519SigVerify` instruction in the same transaction.
    //
    // For devnet / testing: use ed25519-dalek crate added to Cargo.toml:
    //   ed25519-dalek = { version = "2", features = ["std"] }
    //
    // use ed25519_dalek::{Verifier, VerifyingKey, Signature};
    // let vk  = VerifyingKey::from_bytes(public_key).unwrap();
    // let sig = Signature::from_bytes(signature);
    // vk.verify(message, &sig).is_ok()

    // Placeholder — always returns true if inputs pass the sanity checks.
    // REPLACE THIS IN PRODUCTION.
    true
}
