/*
use anchor_lang::prelude::*;

// ---------------------------------------
// CONSTANTS (PDA SEEDS)
// ---------------------------------------
pub const VAULT_SEED: &[u8] = b"vault";
pub const USER_POSITION_SEED: &[u8] = b"user_position";
pub const STRATEGY_SEED: &[u8] = b"strategy";

// ---------------------------------------
// ENUMS
// ---------------------------------------

/// Represents the risk cap for the strategy.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Represents how often the vault should trigger rebalancing.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RebalanceFrequency {
    Daily,
    Weekly,
    Dynamic, // Utilized for the MEV split-route/random delay logic
}

/// Represents the underlying yield sources integrated via adapters.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum YieldSource {
    Raydium,
    Drift,
    ArciumEncrypted, // When routing logic is completely hidden inside MXE
}

// ---------------------------------------
// STRUCTS (ACCOUNT SCHEMAS)
// ---------------------------------------

/// Vault Account: PDA-based account to hold pooled funds and reference the active strategy.
#[account]
pub struct Vault {
    pub authority: Pubkey,                 // 32 bytes - Vault manager/creator
    pub total_deposited: u64,              // 8 bytes - Total TVL in the vault
    pub active_strategy: Pubkey,           // 32 bytes - Reference to the Strategy PDA
    pub encrypted_routing_hash: [u8; 32],  // 32 bytes - Arcium MXE encrypted execution hash
    pub bump: u8,                          // 1 byte - PDA bump
}

impl Vault {
    // 8 (discriminator) + 32 + 8 + 32 + 32 + 1 = 113 bytes
    pub const SPACE: usize = 8 + 32 + 8 + 32 + 32 + 1;
}

/// User Position: Tracks individual user deposits and their share of the vault.
#[account]
pub struct UserPosition {
    pub owner: Pubkey,             // 32 bytes - User's wallet address
    pub vault: Pubkey,             // 32 bytes - The vault they deposited into
    pub deposited_amount: u64,     // 8 bytes - Initial principal deposited
    pub shares: u64,               // 8 bytes - Pro-rata shares of the vault
    pub bump: u8,                  // 1 byte - PDA bump
}

impl UserPosition {
    // 8 (discriminator) + 32 + 32 + 8 + 8 + 1 = 89 bytes
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1;
}

/// Strategy Account: Defines the rules, constraints, and MEV protections for yield routing.
#[account]
pub struct Strategy {
    pub creator: Pubkey,                           // 32 bytes - Strategy author
    pub apy_target: u16,                           // 2 bytes - Target APY (in basis points)
    pub risk_level: RiskLevel,                     // 1 byte - Risk cap
    pub rebalance_frequency: RebalanceFrequency,   // 1 byte - Expected frequency
    pub last_rebalance_ts: i64,                    // 8 bytes - Timestamp of last execution
    pub mev_time_delay: i64,                       // 8 bytes - Random time-delay for stealth rebalancing
    pub strategy_nft_mint: Pubkey,                 // 32 bytes - Metaplex cNFT attached to this strategy
    pub is_active: bool,                           // 1 byte - Strategy status
    pub bump: u8,                                  // 1 byte - PDA bump
}

impl Strategy {
    // 8 (discriminator) + 32 + 2 + 1 + 1 + 8 + 8 + 32 + 1 + 1 = 94 bytes
    pub const SPACE: usize = 8 + 32 + 2 + 1 + 1 + 8 + 8 + 32 + 1 + 1;
}


use anchor_lang::prelude::*;

// Added new enum for collaboration levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CollaborationLevel {
    None,
    Basic,
    Advanced,
    Full
}

// Added new enum for trust levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TrustLevel {
    Untrusted,
    Low,
    Medium,
    High,
    Verified
}

#[account]
pub struct AgentSystem {
    pub manager: Pubkey,
    pub name: String,
    pub is_active: bool,
    pub agent_count: u16,
    pub created_at: i64,
    pub last_updated: i64,
}

impl AgentSystem {
    pub const SPACE: usize = 32 + // manager: Pubkey
                              50 + // name: String (max 50 chars)
                              1 +  // is_active: bool
                              2 +  // agent_count: u16
                              8 +  // created_at: i64
                              8;   // last_updated: i64
}

#[account]
pub struct Agent {
    pub authority: Pubkey,
    pub system: Pubkey,
    pub name: String,
    pub agent_type: String,
    pub risk_preference: String,
    pub is_active: bool,
    pub strategy_count: u16,
    pub position_count: u16,
    pub created_at: i64,
    pub last_updated: i64,
    // Added new fields
    pub verification_key: Option<String>, // Public key for message verification
    pub default_collaboration_level: u8, // 0=None, 1=Basic, 2=Advanced, 3=Full
    pub min_trust_level: u8, // 0=Untrusted, 1=Low, 2=Medium, 3=High, 4=Verified
}

impl Agent {
    pub const SPACE: usize = 32 + // authority: Pubkey
                              32 + // system: Pubkey
                              50 + // name: String (max 50 chars)
                              30 + // agent_type: String (max 30 chars)
                              20 + // risk_preference: String (max 20 chars)
                              1 +  // is_active: bool
                              2 +  // strategy_count: u16
                              2 +  // position_count: u16
                              8 +  // created_at: i64
                              8 +  // last_updated: i64
                              65 + // verification_key: Option<String> (max 64 chars + 1)
                              1 +  // default_collaboration_level: u8
                              1;   // min_trust_level: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Allocation {
    pub opportunity_id: String,
    pub weight: f64,
    pub expected_apy: f64,
}

#[account]
pub struct TradingStrategy {
    pub authority: Pubkey,
    pub agent: Pubkey,
    pub name: String,
    pub risk_level: String,
    pub min_apy: f64,
    pub is_active: bool,
    pub allocations: Vec<Allocation>,
    pub created_at: i64,
    pub last_updated: i64,
}

impl TradingStrategy {
    pub const SPACE: usize = 32 + // authority: Pubkey
                              32 + // agent: Pubkey
                              50 + // name: String (max 50 chars)
                              20 + // risk_level: String (max 20 chars)
                              8 +  // min_apy: f64
                              1 +  // is_active: bool
                              200 + // allocations: Vec<Allocation> (estimated size)
                              8 +  // created_at: i64
                              8;   // last_updated: i64
}

#[account]
pub struct TradingPosition {
    pub authority: Pubkey,
    pub agent: Pubkey,
    pub opportunity_id: String,
    pub protocol_id: String,
    pub amount: u64,
    pub entry_apy: f64,
    pub current_apy: f64,
    pub status: String,
    pub created_at: i64,
    pub last_updated: i64,
}

impl TradingPosition {
    pub const SPACE: usize = 32 + // authority: Pubkey
                              32 + // agent: Pubkey
                              50 + // opportunity_id: String (max 50 chars)
                              30 + // protocol_id: String (max 30 chars)
                              8 +  // amount: u64
                              8 +  // entry_apy: f64
                              8 +  // current_apy: f64
                              10 + // status: String (max 10 chars)
                              8 +  // created_at: i64
                              8;   // last_updated: i64
}

#[account]
pub struct AgentCollaboration {
    pub source_agent: Pubkey,
    pub target_agent: Pubkey,
    pub collaboration_type: String,
    pub content: String,
    pub status: String,
    pub decision: Option<String>,
    pub response: Option<String>,
    pub created_at: i64,
    pub last_updated: i64,
    // Added new fields
    pub signature: Option<String>,
    pub verification_status: u8, // 0=unverified, 1=verified
    pub trust_score: u16, // 0-100, representing trust level percentage
}

impl AgentCollaboration {
    pub const SPACE: usize = 32 + // source_agent: Pubkey
                              32 + // target_agent: Pubkey
                              30 + // collaboration_type: String (max 30 chars)
                              200 + // content: String (max 200 chars)
                              10 + // status: String (max 10 chars)
                              50 + // decision: Option<String> (max 50 chars)
                              200 + // response: Option<String> (max 200 chars)
                              8 +  // created_at: i64
                              8 +  // last_updated: i64
                              130 + // signature: Option<String> (max 128 chars + 2)
                              1 +  // verification_status: u8
                              2;   // trust_score: u16
}

// Added new struct for agent collaboration relationships
#[account]
pub struct AgentRelationship {
    pub source_agent: Pubkey,
    pub target_agent: Pubkey,
    pub trust_level: u8, // 0=Untrusted, 1=Low, 2=Medium, 3=High, 4=Verified
    pub collaboration_level: u8, // 0=None, 1=Basic, 2=Advanced, 3=Full
    pub reputation: u16, // 0-100 score
    pub successful_interactions: u32,
    pub failed_interactions: u32,
    pub last_interaction: i64,
    pub created_at: i64,
    pub last_updated: i64,
}

impl AgentRelationship {
    pub const SPACE: usize = 32 + // source_agent: Pubkey
                              32 + // target_agent: Pubkey
                              1 +  // trust_level: u8
                              1 +  // collaboration_level: u8
                              2 +  // reputation: u16
                              4 +  // successful_interactions: u32
                              4 +  // failed_interactions: u32
                              8 +  // last_interaction: i64
                              8 +  // created_at: i64
                              8;   // last_updated: i64
}

// Added new struct for collaborative decisions
#[account]
pub struct CollaborativeDecision {
    pub decision_id: String,
    pub decision_type: String, // "trade", "risk_assessment", "strategy"
    pub participants: Vec<Pubkey>,
    pub confidence_score: f64,
    pub final_decision: String, // JSON string of final decision
    pub created_at: i64,
    pub signatures: Vec<String>, // List of signatures from participants
}

impl CollaborativeDecision {
    pub const SPACE: usize = 50 + // decision_id: String (max 50 chars)
                              20 + // decision_type: String (max 20 chars)
                              400 + // participants: Vec<Pubkey> (est. 10 participants)
                              8 +  // confidence_score: f64
                              1000 + // final_decision: String (max 1000 chars)
                              8 +  // created_at: i64
                              500;  // signatures: Vec<String> (est. 10 signatures)
}

// Added new struct for shared insights
#[account]
pub struct SharedInsight {
    pub source_agent: Pubkey,
    pub insight_type: String, // "yield_opportunity", "risk_assessment", etc.
    pub content: String, // JSON string of insight content
    pub confidence: f64,
    pub created_at: i64,
    pub trust_score: u16, // 0-100
    pub verification_status: u8, // 0=unverified, 1=partially_verified, 2=fully_verified
    pub signatures: Vec<String>, // List of signatures from validators
}

impl SharedInsight {
    pub const SPACE: usize = 32 + // source_agent: Pubkey
                              30 + // insight_type: String (max 30 chars)
                              500 + // content: String (max 500 chars)
                              8 +  // confidence: f64
                              8 +  // created_at: i64
                              2 +  // trust_score: u16
                              1 +  // verification_status: u8
                              500;  // signatures: Vec<String> (est. 10 signatures)
}

// Context struct declarations
#[derive(Accounts)]
pub struct InitializeAgentSystem<'info> {
    #[account(
        init,
        payer = manager_authority,
        space = 8 + AgentSystem::SPACE
    )]
    pub agent_system: Account<'info, AgentSystem>,
    #[account(mut)]
    pub manager_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAgent<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Agent::SPACE
    )]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub agent_system: Account<'info, AgentSystem>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TradingStrategy::SPACE
    )]
    pub strategy: Account<'info, TradingStrategy>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePosition<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TradingPosition::SPACE
    )]
    pub position: Account<'info, TradingPosition>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub position: Account<'info, TradingPosition>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePositionApy<'info> {
    #[account(mut)]
    pub position: Account<'info, TradingPosition>,
    pub agent: Account<'info, Agent>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Collaborate<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + AgentCollaboration::SPACE
    )]
    pub collaboration: Account<'info, AgentCollaboration>,
    pub source_agent: Account<'info, Agent>,
    pub target_agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RespondToCollaboration<'info> {
    #[account(mut)]
    pub collaboration: Account<'info, AgentCollaboration>,
    pub target_agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

// Added new context for establishing agent relationships
#[derive(Accounts)]
pub struct EstablishRelationship<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + AgentRelationship::SPACE
    )]
    pub relationship: Account<'info, AgentRelationship>,
    pub source_agent: Account<'info, Agent>,
    pub target_agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Added new context for shared insights
#[derive(Accounts)]
pub struct ShareInsight<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SharedInsight::SPACE
    )]
    pub insight: Account<'info, SharedInsight>,
    pub source_agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Added new context for validating insights
#[derive(Accounts)]
pub struct ValidateInsight<'info> {
    #[account(mut)]
    pub insight: Account<'info, SharedInsight>,
    pub validator_agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

// Added new context for collaborative decisions
#[derive(Accounts)]
pub struct CreateCollaborativeDecision<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + CollaborativeDecision::SPACE
    )]
    pub decision: Account<'info, CollaborativeDecision>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
} 
*/
// need to delete agent pipeline
