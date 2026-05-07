use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Orderbook{
    pub admin: Pubkey,
    pub arcium_artifact_id: [u8; 32],
    pub arcium_verification_key: [u8; 32],
    pub arcium_mxe_public_key: [u8; 32],
    pub escrow_authority_bump: u8,
    pub order_count: u64,
    pub token_int: Pubkey,
    pub total_volume: u64,
    pub total_matches: u64,
    pub created_at: i64,
    pub token_mint: Pubkey,


}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy , PartialEq, Eq, Debug, InitSpace)]
#[repr(u8)]
pub enum OrderStatus {
    Pending = 0,
    Queued = 1,
    Processing = 2,
    Filled = 3,
    Cancelled = 4,

}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}


#[account]
#[derive(InitSpace)]
pub struct Order {
    pub owner: Pubkey,
    pub order_id: u64,
    pub encryption_pubkey: [u8; 32],
    pub nonce: u128,
    pub escrow_amount: u64,
    pub status: OrderStatus,
    pub order_ciphertext: [u8; 96],
    pub placed_at: i64,
    pub updated_at: i64,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
#[repr(u8)]
pub enum Protocol {
    Raydium  = 0,
    Drift    = 1,
    Solend   = 2,
    DarkPool = 3,
}

impl Default for Protocol{
    fn default() -> Self {
        Protocol::Raydium
    }
}

#[account]
#[derive(InitSpace)]
pub struct YieldPosition {

    pub owner: Pubkey,

    pub position_id: u64,

    pub escrowed_tokens: u64,

    pub protocol: Protocol,

    pub position_ciphertext: [u8; 96],

    pub encryption_pubkey: [u8; 32],

    pub nonce: u128,

    pub accrued_yield: u64,

    pub last_compounded_at: i64,

    pub auto_compound: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct SettlementRecord {
    pub settlement_hash: [u8; 32],
    pub bundle_timestamp: i64,
    pub relayer: Pubkey,
    pub is_yield_settlement: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SettlementBundleHeader {
    pub artifact_id: [u8; 32],
    pub settlement_hash: [u8; 32],
    pub timestamp: i64,
    pub amount_a_to_b: u64,
    pub amount_b_to_a: u64,
}

// MXE computatioon logic
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MatchOrdersOutput {
    pub matched: bool,
    pub execution_price: u64,
    pub execution_size: u64,
}

impl arcium_anchor::HasSize for MatchOrdersOutput {
    const SIZE: usize = 1 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RouteYieldOutput {
    pub success: bool,
    pub deposit_amount: u64,
    pub target_protocol: u8,
    pub yield_rate_bps: u64,
}

impl arcium_anchor::HasSize for RouteYieldOutput {
    const SIZE: usize = 1 + 8 + 1 + 8;
}




/// Ciphertext fields sent alongside a 'match-order-pair' Instruction Soo.
/// Key Notes also availibe in comments
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OrderEncryptedData {
    /// Encrypted `bool` — true = buy, false = sell.
    pub is_buy_enc:  [u8; 32],
    /// Encrypted `u64` price in lamports-per-token.
    pub price_enc:   [u8; 32],
    /// Encrypted `u64` token amount.
    pub size_enc:    [u8; 32],
    /// Encrypted `[u8; 32]` owner pubkey.
    pub owner_enc:   [u8; 32],
    /// Plaintext nonce (authenticated but not secret).
    pub nonce:       u128,
}