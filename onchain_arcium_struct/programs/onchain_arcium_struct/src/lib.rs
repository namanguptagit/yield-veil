use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
use crate::state::orderbook::*;
use crate::state::vault::*;

declare_id!("CJAexGmZWBZSaMQdrFjC3rCWSyQre1J41P668WW1Tr32");

#[program]
pub mod your_program_name {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        Ok(())
    }

    pub fn create_strategy(ctx: Context<CreateStrategy>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn init_orderbook(ctx: Context<InitOrderbook>) -> Result<()> {
        Ok(())
    }
}