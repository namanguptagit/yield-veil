use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

// Arcium SDK imports for on-chain program
use arcium_anchor::prelude::*;
use arcium_macros::{arcium_callback, callback_accounts, queue_computation_accounts};
use arcium_client::idl::arcium::types::CallbackAccount;

pub mod constants;
pub mod errors;
pub mod state;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::vault::*;
use crate::state::orderbook::*;

declare_id!("AAuywNJFtz2TiY428fj8C44FMnYV22M1wR4q7MQa7hfB");

const COMP_DEF_OFFSET_MATCH_ORDERS: u32 = comp_def_offset("match_orders");
const COMP_DEF_OFFSET_REBALANCE: u32 = comp_def_offset("rebalance");

#[program]
pub mod yieldveil {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, encrypted_routing_hash: [u8; 32]) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.bump = ctx.bumps.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.total_deposited = 0;
        vault.active_strategy = Pubkey::default();
        vault.encrypted_routing_hash = encrypted_routing_hash;
        Ok(())
    }

    pub fn init_orderbook(ctx: Context<InitOrderbook>) -> Result<()> {
        let ob = &mut ctx.accounts.orderbook;
        ob.admin = ctx.accounts.authority.key();
        ob.order_count = 0;
        ob.token_mint = ctx.accounts.token_mint.key();
        ob.escrow_authority_bump = ctx.bumps.escrow_authority;
        ob.arcium_circuit_id = [0u8; 32];
        ob.arcium_verification_key = [0u8; 32];
        Ok(())
    }

    pub fn create_strategy(
        ctx: Context<CreateStrategy>,
        apy_target: u16,
        risk_level: RiskLevel,
        frequency: RebalanceFrequency,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;

        strategy.bump = ctx.bumps.strategy;
        strategy.creator = ctx.accounts.creator.key();
        strategy.apy_target = apy_target;
        strategy.risk_level = risk_level;
        strategy.frequency = frequency;
        strategy.last_rebalance_ts = 0;
        strategy.mev_time_delay = 0;
        strategy.strategy_nft_mint = Pubkey::default();
        strategy.is_active = true;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token.to_account_info(),
                    to: ctx.accounts.vault_escrow.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault;
        vault.total_deposited = vault
            .total_deposited
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        let pos = &mut ctx.accounts.user_position;
        pos.owner = ctx.accounts.owner.key();
        pos.vault = vault.key();
        pos.deposited_amount = pos
            .deposited_amount
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;
        pos.shares = pos
            .shares
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            ctx.accounts.user_position.deposited_amount >= amount,
            ErrorCode::InsufficientBalance
        );

        let vault_bump = ctx.accounts.vault.bump;
        let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED, &[vault_bump]]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_escrow.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault;
        vault.total_deposited = vault
            .total_deposited
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        let pos = &mut ctx.accounts.user_position;
        pos.deposited_amount = pos
            .deposited_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;
        pos.shares = pos
            .shares
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    pub fn place_order(
        ctx: Context<PlaceOrder>,
        order_encrypted: [u8; 32],
        price_encrypted: [u8; 32],
        size_encrypted: [u8; 32],
        is_buy_encrypted: [u8; 32],
        escrow_amount: u64,
        nonce: u128,
        encryption_pubkey: [u8; 32],
    ) -> Result<()> {
        require!(escrow_amount > 0, ErrorCode::InvalidOrderParams);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token.to_account_info(),
                    to: ctx.accounts.escrow_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            escrow_amount,
        )?;

        let ob = &mut ctx.accounts.orderbook;
        let order = &mut ctx.accounts.order;

        order.owner = ctx.accounts.user.key();
        order.order_id = ob.order_count;
        order.encryption_pubkey = encryption_pubkey;
        order.nonce = nonce;
        order.escrow_amount = escrow_amount;
        order.status = OrderStatus::Pending;
        order.order_encrypted = order_encrypted;
        order.price_encrypted = price_encrypted;
        order.size_encrypted = size_encrypted;
        order.is_buy_encrypted = is_buy_encrypted;

        ob.order_count = ob
            .order_count
            .checked_add(1)
            .ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }

    pub fn match_order_pair(ctx: Context<MatchOrderPair>, computation_offset: u64) -> Result<()> {
        require!(ctx.accounts.order1.status == OrderStatus::Pending, ErrorCode::OrderNotPending);
        require!(ctx.accounts.order2.status == OrderStatus::Pending, ErrorCode::OrderNotPending);

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        let args = ArgBuilder::new()
            .encrypted_u8(ctx.accounts.order1.order_encrypted)
            .encrypted_u8(ctx.accounts.order1.price_encrypted)
            .encrypted_u8(ctx.accounts.order1.size_encrypted)
            .encrypted_u8(ctx.accounts.order1.is_buy_encrypted)
            .encrypted_u8(ctx.accounts.order2.order_encrypted)
            .encrypted_u8(ctx.accounts.order2.price_encrypted)
            .encrypted_u8(ctx.accounts.order2.size_encrypted)
            .encrypted_u8(ctx.accounts.order2.is_buy_encrypted)
            .build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![MatchOrdersCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[
                    CallbackAccount { pubkey: ctx.accounts.order1.key(), is_writable: true },
                    CallbackAccount { pubkey: ctx.accounts.order2.key(), is_writable: true },
                ],
            )?],
            1,
            0,
        )?;
        ctx.accounts.order1.status = OrderStatus::Processing;
        ctx.accounts.order2.status = OrderStatus::Processing;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "match_orders")]
    pub fn match_orders_callback(
        ctx: Context<MatchOrdersCallback>,
        output: SignedComputationOutputs<MatchOrdersOutput>,
    ) -> Result<()> {
        // Verify and extract the output from the Arcium computation
        let matched = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(output_data) => {
                // Extract the matched field from your circuit's output
                // Adapt this based on your actual circuit output structure
                output_data.matched // Assuming your circuit returns a struct with a 'matched' field
            },
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        if matched {
            ctx.accounts.order1.status = OrderStatus::Filled;
            ctx.accounts.order2.status = OrderStatus::Filled;
        } else {
            ctx.accounts.order1.status = OrderStatus::Pending;
            ctx.accounts.order2.status = OrderStatus::Pending;
        }
        Ok(())
    }

    pub fn rebalance(ctx: Context<Rebalance>, computation_offset: u64) -> Result<()> {
        require!(ctx.accounts.vault.total_deposited > 0, ErrorCode::EmptyVault);

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        let args = ArgBuilder::new()
            .plaintext_u64(ctx.accounts.vault.total_deposited)
            .plaintext_u16(ctx.accounts.strategy.apy_target)
            .plaintext_u8(ctx.accounts.strategy.risk_level as u8)
            .plaintext_u8(ctx.accounts.strategy.frequency as u8)
            .encrypted_u8(ctx.accounts.vault.encrypted_routing_hash)
            .build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![RebalanceCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[CallbackAccount { pubkey: ctx.accounts.vault.key(), is_writable: true }],
            )?],
            1,
            0,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "rebalance")]
    pub fn rebalance_callback(
        ctx: Context<RebalanceCallback>,
        output: SignedComputationOutputs<RebalanceOutput>,
    ) -> Result<()> {
        let ciphertexts = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(output_data) => output_data.ciphertexts,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        let vault = &mut ctx.accounts.vault;
        vault.encrypted_routing_hash = ciphertexts;
        vault.last_rebalance_ts = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

// ============================================================================
// CONTEXT STRUCTS - All defined at crate root as required by Arcium macros
// ============================================================================

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [VAULT_SEED],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitOrderbook<'info> {
    #[account(
        init,
        payer = authority,
        space = Orderbook::SPACE,
        seeds = [ORDERBOOK_SEED],
        bump,
    )]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    /// CHECK: This is the PDA authority for the escrow
    #[account(
        seeds = [ESCROW_AUTH_SEED, orderbook.key().as_ref()],
        bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = escrow_authority,
        seeds = [ESCROW_SEED, orderbook.key().as_ref()],
        bump
    )]
    pub escrow_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
    #[account(
        init,
        payer = creator,
        space = Strategy::SPACE,
        seeds = [STRATEGY_SEED, creator.key().as_ref()],
        bump,
    )]
    pub strategy: Account<'info, Strategy>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = owner,
        space = UserPosition::SPACE,
        seeds = [USER_POSITION_SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_position: Account<'info, UserPosition>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_escrow: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [USER_POSITION_SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_position: Account<'info, UserPosition>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_escrow: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(
        init,
        payer = user,
        space = Order::SPACE,
        seeds = [ORDER_SEED, orderbook.key().as_ref(), &orderbook.order_count.to_le_bytes()],
        bump,
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// ============================================================================
// ARCIUM COMPUTATION CONTEXTS
// ============================================================================

// ---------------- QUEUE: MatchOrderPair ----------------
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
    /// CHECK: checked by arcium program
    pub mempool_account: UncheckedAccount<'info>,

    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: checked by arcium program
    pub executing_pool: UncheckedAccount<'info>,

    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: checked by arcium program
    pub computation_account: UncheckedAccount<'info>,

    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_MATCH_ORDERS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,

    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,

    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,

    // Custom accounts
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,
    #[account(mut)]
    pub order1: Account<'info, Order>,
    #[account(mut)]
    pub order2: Account<'info, Order>,

    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// ---------------- CALLBACK: MatchOrdersCallback ----------------
#[callback_accounts("match_orders")]
#[derive(Accounts)]
pub struct MatchOrdersCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,

    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_MATCH_ORDERS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,

    /// CHECK: checked by arcium program
    pub computation_account: UncheckedAccount<'info>,

    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,

    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions sysvar
    pub instructions_sysvar: AccountInfo<'info>,

    // Custom callback accounts (must match order in callback_ix call)
    #[account(mut)]
    pub order1: Account<'info, Order>,
    #[account(mut)]
    pub order2: Account<'info, Order>,
}

// ---------------- QUEUE: Rebalance ----------------
#[queue_computation_accounts("rebalance", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct Rebalance<'info> {
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
    /// CHECK: checked by arcium program
    pub mempool_account: UncheckedAccount<'info>,

    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: checked by arcium program
    pub executing_pool: UncheckedAccount<'info>,

    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: checked by arcium program
    pub computation_account: UncheckedAccount<'info>,

    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_REBALANCE))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,

    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,

    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,

    // Custom accounts
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub strategy: Account<'info, Strategy>,

    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// ---------------- CALLBACK: RebalanceCallback ----------------
#[callback_accounts("rebalance")]
#[derive(Accounts)]
pub struct RebalanceCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,

    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_REBALANCE))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,

    /// CHECK: checked by arcium program
    pub computation_account: UncheckedAccount<'info>,

    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,

    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions sysvar
    pub instructions_sysvar: AccountInfo<'info>,

    // Custom callback account
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}