use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::orderbook::*;
use crate::state::vault::*;

declare_id!("CJAexGmZWBZSaMQdrFjC3rCWSyQre1J41P668WW1Tr32");

#[program]
pub mod yieldveil {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        encrypted_routing_hash: [u8; 32],
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.bump = ctx.bumps.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.total_deposited = 0;
        vault.active_strategy = Pubkey::default();
        vault.encrypted_routing_hash = encrypted_routing_hash;

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

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.key(),
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

        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.key(),
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
}