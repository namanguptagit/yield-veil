use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::orderbook::{RiskLevel, RebalanceFrequency};
// ---------------------------------------
// ACCOUNT SCHEMAS
// ---------------------------------------

#[account]
pub struct Vault {
    pub authority: Pubkey,                // 32
    pub total_deposited: u64,             // 8
    pub active_strategy: Pubkey,          // 32
    pub encrypted_routing_hash: [u8; 32], // 32
    pub bump: u8,                         // 1
}

impl Vault {
    pub const SPACE: usize = 8 + 32 + 8 + 32 + 32 + 1;
}

#[account]
pub struct UserPosition {
    pub owner: Pubkey,         // 32
    pub vault: Pubkey,         // 32
    pub deposited_amount: u64, // 8
    pub shares: u64,           // 8
    pub bump: u8,              // 1
}

impl UserPosition {
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1;
}

#[account]
pub struct Strategy {
    pub creator: Pubkey,               // 32
    pub apy_target: u16,               // 2
    pub risk_level: RiskLevel,         // 1
    pub frequency: RebalanceFrequency, // 1
    pub last_rebalance_ts: i64,        // 8
    pub mev_time_delay: i64,           // 8
    pub strategy_nft_mint: Pubkey,     // 32
    pub is_active: bool,               // 1
    pub bump: u8,                      // 1
}

impl Strategy {
    pub const SPACE: usize = 8 + 32 + 2 + 1 + 1 + 8 + 8 + 32 + 1 + 1;
}

// ---------------------------------------
// INSTRUCTION CONTEXTS (VAULT/STRATEGY)
// ---------------------------------------

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
    #[account(
        init,
        payer = creator,
        space = Strategy::SPACE,
        seeds = [STRATEGY_SEED, creator.key().as_ref()],
        bump
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
        seeds = [USER_POSITION_SEED, owner.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [USER_POSITION_SEED, owner.key().as_ref(), vault.key().as_ref()],
        bump = user_position.bump,
        close = owner
    )]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub owner: Signer<'info>,
}