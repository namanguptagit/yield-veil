use arcis::*;

#[encrypted]
pub mod circuits {
    use arcis::*;

    pub struct RebalanceInput {
        pub current_vault_balance: u64,
        pub target_apy: u16,
        pub risk_level: u8, 
    }

    pub struct RebalanceOutput {
        pub amount_to_deploy: u64,
        pub reserved_for_withdrawals: u64,
    }

    #[instruction]
    pub fn compute_rebalance(
        input_ctxt: Enc<Shared, RebalanceInput>
    ) -> Enc<Shared, RebalanceOutput> {
        let input = input_ctxt.to_arcis();

        // Standard MPC logic: Use match or if/else for branches
        let deployment_bps = match input.risk_level {
            0 => 5000, 
            1 => 7500, 
            2 => 9000, 
            _ => 0,
        };

        let amount_to_deploy = ((input.current_vault_balance as u128 * deployment_bps as u128) / 10000) as u64;
        let reserved_for_withdrawals = input.current_vault_balance - amount_to_deploy;

        let result = RebalanceOutput {
            amount_to_deploy,
            reserved_for_withdrawals,
        };

        input_ctxt.owner.from_arcis(result)
    }
}
