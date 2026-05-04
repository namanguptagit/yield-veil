use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use arcium_anchor::HasSize;
use crate::constants::*;
use crate::errors::ErrorCode;

use borsh::{BorshDeserialize, BorshSerialize};





// ---------------------------------------
// ENUMS
// ---------------------------------------

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RebalanceFrequency {
    Daily,
    Weekly,
    Dynamic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum YieldSource {
    Raydium,
    Drift,
    ArciumEncrypted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Paused,
    Deprecated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Amm,
    Clmm,
    Orderbook,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PayloadType {
    RebalanceInput,
    OrderData,
    StrategyParams,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PayloadStatus {
    Pending,
    Processing,
    Computed,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TxStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PositionStatus {
    Active,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CronJobType {
    Scan,
    Rebalance,
    Compound,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CronJobStatus {
    Active,
    Paused,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogStatus {
    Success,
    Failure,
    Timeout,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Filled,
    Cancelled,
}


// ---------------------------------------
// HELPER STRUCTURES
// ---------------------------------------






#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrderEncryptedData {
    pub is_buy: [u8; 32],
    pub price: [u8; 32],
    pub size: [u8; 32],
    pub owner: [u8; 32],
    pub nonce: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SettlementBundleHeader {
    pub artifact_id: [u8; 32],
    pub settlement_hash: [u8; 32],
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SettlementBundle {
    pub order1_id: u64,
    pub order2_id: u64,
    pub matched: bool,
    pub execution_price: u64,
    pub execution_size: u64,
    pub commitment_hash: [u8; 32],
    pub arcium_signature: [u8; 64],
    pub artifact_id: [u8; 32],
}

// ---------------------------------------
// ACCOUNT SCHEMAS
// ---------------------------------------

#[account]
pub struct Orderbook {
    pub admin: Pubkey,
    pub order_count: u64,
    pub token_mint: Pubkey,
    pub escrow_authority_bump: u8,
    pub arcium_circuit_id: [u8; 32],
    pub arcium_verification_key: [u8; 32],
}

impl Orderbook {
    pub const SPACE: usize = 8 + // discriminator
        32 + // admin
        8 + // order_count
        32 + // token_mint
        1 + // escrow_authority_bump
        32 + // arcium_circuit_id
        32; // arcium_verification_key
}

impl OrderCommitment {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 16 + 8 + 1 + 8 + 8;
}


#[account]
pub struct OrderCommitment {

    pub owner: Pubkey,               // 32
    pub commitment_hash: [u8; 32],   // 32
    pub encryption_pubkey: [u8; 32],  // 32
    pub nonce: u128,                 // 16
    pub escrow_amount: u64,          // 8
    pub state: u8,                   // 1
    pub order_id: u64,               // 8
    pub timestamp: i64,              // 8
}

#[account]
pub struct Order {
    pub owner: Pubkey,               // 32
    pub order_id: u64,               // 8
    pub encryption_pubkey: [u8; 32], // 32
    pub nonce: u128,                 // 16
    pub escrow_amount: u64,          // 8
    pub status: OrderStatus,         // 1
    pub order_encrypted: [u8; 32],   // 32
    pub price_encrypted: [u8; 32],   // 32
    pub size_encrypted: [u8; 32],    // 32
    pub is_buy_encrypted: [u8; 32],  // 32
}

impl Order {
    pub const SPACE: usize = 8 + // discriminator
        32 + // owner
        8 + // order_id
        32 + // encryption_pubkey
        16 + // nonce
        8 + // escrow_amount
        1 + // status
        32 + // order_encrypted
        32 + // price_encrypted
        32 + // size_encrypted
        32; // is_buy_encrypted
}

#[account]
pub struct SettlementRecord {
    pub settlement_hash: [u8; 32], // 32
    pub timestamp: i64,            // 8
    pub relayer: Pubkey,           // 32
}

impl SettlementRecord {
    pub const SPACE: usize = 8 + 32 + 8 + 32;
}

// ---------------------------------------
// INSTRUCTION CONTEXTS (ORDERBOOK/MATCHING)
// ---------------------------------------

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

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        // The authority is the Orderbook PDA
        token::authority = orderbook,
        seeds = [ESCROW_SEED, orderbook.key().as_ref()],
        bump
    )]
    pub escrow_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = Order::SPACE,
        seeds = [
            ORDER_SEED,
            orderbook.key().as_ref(),
            &orderbook.order_count.to_le_bytes()
        ],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}




#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,
    #[account(mut)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(bundle_header: SettlementBundleHeader)]
pub struct SettleMatches<'info> {
    #[account(mut, has_one = admin)]
    pub orderbook: Account<'info, Orderbook>,


    pub admin: UncheckedAccount<'info>,

    #[account(mut)]
    pub order_a: Account<'info, OrderCommitment>,
    #[account(mut)]
    pub order_b: Account<'info, OrderCommitment>,
    #[account(mut)]
    pub escrow_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_b: Account<'info, TokenAccount>,
    /// CHECK: Relayer or ARX-signed authority - used only for logging and access control
    #[account(mut)]
    pub relayer: Signer<'info>,
    /// CHECK: derived PDA
    #[account(
        seeds = [ESCROW_AUTH_SEED, orderbook.key().as_ref()],
        bump = orderbook.escrow_authority_bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer = relayer,
        space = SettlementRecord::SPACE,
        seeds = [SETTLEMENT_SEED, bundle_header.settlement_hash.as_ref()],
        bump
    )]
    pub settlement_record: Account<'info, SettlementRecord>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// ---------------------------------------
// EVENTS
// ---------------------------------------

#[event]
pub struct OrderbookInitialized {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub arcium_artifact_id: [u8; 32],
}

#[event]
pub struct OrderPlaced {
    pub order_id: u64,
    pub owner: Pubkey,
    pub escrow_amount: u64,
}

#[event]
pub struct MatchInitiated {
    pub order1: Pubkey,
    pub order2: Pubkey,
}

#[event]
pub struct OrderMatched {
    pub order1: Pubkey,
    pub order2: Pubkey,
    pub execution_price: u64,
    pub execution_size: u64,
}

#[event]
pub struct OrderMatchFailed {
    pub order1: Pubkey,
    pub order2: Pubkey,
}

#[event]
pub struct OrderCancelled {
    pub order_id: u64,
    pub owner: Pubkey,
    pub refund_amount: u64,
}

#[event]
pub struct SettlementExecuted {
    pub settlement_hash: [u8; 32],
    pub order_a: Pubkey,
    pub order_b: Pubkey,
}

// ---------------------------------------
// ERRORS
// ---------------------------------------

