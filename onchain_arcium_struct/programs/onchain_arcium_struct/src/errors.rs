use anchor_lang::prelude::*;

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