use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Paused,
    Deprecated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
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

/// Output type for order matching computation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MatchOrdersOutput {
    pub matched: bool,
    pub execution_price: u64,
    pub execution_size: u64,
}

// ============================================================
// ACCOUNT STRUCTURES
// ============================================================

#[account]
pub struct Orderbook {
    pub admin: Pubkey,
    pub arcium_artifact_id: [u8; 32],
    pub arcium_verification_key: [u8; 32],
    pub arcium_mxe_public_key: [u8; 32],
    pub escrow_authority_bump: u8,
    pub order_count: u64,
    pub token_mint: Pubkey,
    // other fields...
}

#[account]
pub struct OrderCommitment {
    pub owner: Pubkey,
    pub commitment_hash: [u8; 32],
    pub encryption_pubkey: [u8; 32],
    pub nonce: u128,
    pub escrow_amount: u64,
    pub state: u8, // 0=Committed,1=Processing,2=Matched,3=Settled,4=Cancelled
    pub order_id: u64,
    pub timestamp: i64,
}

#[account]
pub struct Order {
    pub owner: Pubkey,
    pub order_id: u64,
    pub encryption_pubkey: [u8; 32],
    pub nonce: u128,
    pub escrow_amount: u64,
    pub status: OrderStatus,
    // Encrypted order data
    pub order_encrypted: [u8; 32],
    pub price_encrypted: [u8; 32],
    pub size_encrypted: [u8; 32],
    pub is_buy_encrypted: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum OrderStatus {
    Pending,
    Queued,      // Submitted to Arcium for processing
    Processing,  // Currently being processed by MPC
    Filled,
    Cancelled,
}

// Keep a record of settlement bundles to prevent replay
#[account]
pub struct SettlementRecord {
    pub settlement_hash: [u8; 32],
    pub timestamp: i64,
    pub relayer: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SettlementBundleHeader {
    pub artifact_id: [u8; 32],
    pub settlement_hash: [u8; 32],
    pub timestamp: i64,
}

// ============================================================
// CONTEXT STRUCTURES
// ============================================================

#[derive(Accounts)]
pub struct InitOrderbook<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 32 + 1 + 8 + 32, // discriminator + admin + artifact_id + verification_key + mxe_pubkey + bump + order_count + token_mint
        seeds = [b"orderbook"],
        bump,

    )]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = orderbook,
        seeds = [b"escrow"],
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
        space = 8 + 32 + 8 + 32 + 16 + 8 + 1 + 32 + 32 + 32 + 32, // discriminator + owner + order_id + encryption_pubkey + nonce + escrow_amount + status + 4 encrypted fields
        seeds = [b"order", orderbook.key().as_ref(), &orderbook.order_count.to_le_bytes()],
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
pub struct MatchOrderPair<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub order1: Account<'info, Order>,

    #[account(mut)]
    pub order2: Account<'info, Order>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct MatchCallback<'info> {
    #[account(mut)]
    pub orderbook: Account<'info, Orderbook>,

    #[account(mut)]
    pub order1: Account<'info, Order>,

    #[account(mut)]
    pub order2: Account<'info, Order>,

    #[account(mut)]
    pub order1_escrow: Account<'info, TokenAccount>,

    #[account(mut)]
    pub order2_escrow: Account<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
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

// The expected on-chain instruction accounts for settle_matches
#[derive(Accounts)]
#[instruction(bundle_header: SettlementBundleHeader)]
pub struct SettleMatches<'info> {
    #[account(mut, has_one = admin)]
    pub orderbook: Account<'info, Orderbook>,

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
    #[account(seeds = [b"escrow_auth", orderbook.key().as_ref()], bump = orderbook.escrow_authority_bump)]
    pub escrow_authority: UncheckedAccount<'info>,

    #[account(

        init,
        payer = relayer,
        space = 8 + 32 + 8 + 32, // discriminator + settlement_hash + timestamp + relayer
        seeds = [b"settlement", bundle_header.settlement_hash.as_ref()],
        bump
    )]

    pub settlement_record: Account<'info, SettlementRecord>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// ============================================================
// EVENT STRUCTURES
// ============================================================

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

// ============================================================
// ERROR CODES
// ============================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Order is not in pending status")]
    OrderNotPending,
    #[msg("Order is not in processing status")]
    OrderNotProcessing,
    #[msg("Insufficient escrow balance")]
    InsufficientEscrow,
    #[msg("Invalid order parameters")]
    InvalidOrderParams,
    #[msg("Arcium artifact ID mismatch")]
    InvalidArtifactId,
    #[msg("Invalid Arcium signature")]
    InvalidArciumSignature,
    #[msg("Settlement replay detected")]
    SettlementReplay,
    #[msg("Invalid order state")]
    InvalidOrderState,
    #[msg("Settlement data mismatch")]
    SettlementMismatch,
    #[msg("Order already cancelled")]
    OrderAlreadyCancelled,
    #[msg("Invalid escrow authority")]
    InvalidEscrowAuthority,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("Order not found")]
    OrderNotFound,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
}

// ============================================================
// HELPER DATA STRUCTURES
// ============================================================

// Helper struct for encrypted order data
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrderEncryptedData {
    pub is_buy: [u8; 32],
    pub price: [u8; 32],
    pub size: [u8; 32],
    pub owner: [u8; 32],
    pub nonce: u128,
}

// Settlement bundle from Arcium MPC computation
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