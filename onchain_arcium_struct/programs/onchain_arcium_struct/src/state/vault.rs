use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub bump: u8,
    pub authority: Pubkey,
    pub total_deposited: u64,
    pub active_strategy: Pubkey,
    pub encrypted_routing_hash: [u8; 32],
    pub last_rebalance_ts: i64,
}

impl Vault {
    pub const SPACE: usize = 8 + // discriminator
        1 + // bump
        32 + // authority
        8 + // total_deposited
        32 + // active_strategy
        32 + // encrypted_routing_hash
        8; // last_rebalance_ts
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RebalanceFrequency {
    Daily,
    Weekly,
    Monthly,
}

#[account]
pub struct Strategy {
    pub bump: u8,
    pub creator: Pubkey,
    pub apy_target: u16,
    pub risk_level: RiskLevel,
    pub frequency: RebalanceFrequency,
    pub last_rebalance_ts: i64,
    pub mev_time_delay: u64,
    pub strategy_nft_mint: Pubkey,
    pub is_active: bool,
}

impl Strategy {
    pub const SPACE: usize = 8 + // discriminator
        1 + // bump
        32 + // creator
        2 + // apy_target
        1 + // risk_level
        1 + // frequency
        8 + // last_rebalance_ts
        8 + // mev_time_delay
        32 + // strategy_nft_mint
        1; // is_active
}

#[account]
pub struct UserPosition {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub deposited_amount: u64,
    pub shares: u64,
}

impl UserPosition {
    pub const SPACE: usize = 8 + // discriminator
        32 + // owner
        32 + // vault
        8 + // deposited_amount
        8; // shares
}