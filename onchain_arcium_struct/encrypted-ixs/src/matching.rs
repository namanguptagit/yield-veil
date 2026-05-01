use arcis::*;

#[encrypted]
pub mod circuits {
    use arcis::*;

    
    // All the structs or data structures
    
    pub struct Order {
        
        pub is_buy: u8,
        
        pub price: u64,
        
        pub size: u64,
        
        pub owner: [u8; 32],
    }

    
    pub struct EscrowState {
        pub balance: u64,
        pub owner: [u8; 32],
    }

    
    pub struct MatchResult {
        
        pub matched: u8,
        pub execution_price: u64,
        pub execution_size: u64,
    }

    
    pub struct MatchOutput {
        pub result: MatchResult,
        pub new_escrow1: EscrowState,
        pub new_escrow2: EscrowState,
    }

    
    
    #[instruction]
    pub fn match_orders(
        order1_ctxt: Enc<Shared, Order>,
        order2_ctxt: Enc<Shared, Order>,
        escrow1_ctxt: Enc<Mxe, EscrowState>,
        escrow2_ctxt: Enc<Mxe, EscrowState>,
    ) -> Enc<Shared, MatchOutput> {
        
        
        let o1 = order1_ctxt.to_arcis();
        let o2 = order2_ctxt.to_arcis();
        let e1 = escrow1_ctxt.to_arcis();
        let e2 = escrow2_ctxt.to_arcis();
        

        let opposite_sides = if o1.is_buy != o2.is_buy { 1 } else { 0 };
        
        // price crossover
        let price_match = if o1.is_buy == 1 {
            if o1.price >= o2.price { 1 } else { 0 }
        } else {
            if o2.price >= o1.price { 1 } else { 0 }
        };

        let is_matched = opposite_sides * price_match;
        
        // Execution Details
        let execution_price = if is_matched == 1 {
            if o1.is_buy == 1 { o2.price } else { o1.price }
        } else {
            0
        };
        
        let execution_size = if is_matched == 1 {
            if o1.size < o2.size { o1.size } else { o2.size }
        } else {
            0
        };
        
        // Validate Escrow Balances
        let sufficient_balance = if is_matched == 1 {
            if o1.is_buy == 1 {
                if e1.balance >= execution_size { 1 } else { 0 }
            } else {
                if e2.balance >= execution_size { 1 } else { 0 } // Fix: Seller is o2, so check e2
            }
        } else {
            0
        };
        
        let final_matched = is_matched * sufficient_balance;
        
        // its Update escrow States
        let new_e1 = if final_matched == 1 {
            if o1.is_buy == 1 {
                EscrowState { balance: e1.balance - execution_size, owner: e1.owner }
            } else {
                EscrowState { balance: e1.balance, owner: e1.owner }
            }
        } else {
            e1
        };
        
        let new_e2 = if final_matched == 1 {
            if o2.is_buy == 1 {
                EscrowState { balance: e2.balance - execution_size, owner: e2.owner }
            } else {
                EscrowState { balance: e2.balance, owner: e2.owner }
            }
        } else {
            e2
        };
        
        
        let output = MatchOutput {
            result: MatchResult {
                matched: final_matched,
                execution_price,
                execution_size,
            },
            new_escrow1: new_e1,
            new_escrow2: new_e2,
        };
        
        order1_ctxt.owner.from_arcis(output)
    }

    /// evaluate & validate order parameters
    #[instruction]
    pub fn validate_order(
        order_ctxt: Enc<Shared, Order>,
        min_price: u64,
        max_price: u64,
        min_size: u64,
        max_size: u64,
    ) -> Enc<Shared, u8> {
        let o = order_ctxt.to_arcis();
        
        let valid_price = if o.price >= min_price && o.price <= max_price { 1 } else { 0 };
        let valid_size = if o.size >= min_size && o.size <= max_size { 1 } else { 0 };
        
        // for non-zero owner
        let empty_owner = [0u8; 32];
        let valid_owner = if o.owner != empty_owner { 1 } else { 0 }; 
        
    
        let is_valid = valid_price * valid_size * valid_owner;
        
        order_ctxt.owner.from_arcis(is_valid)
    }

    /// Function to calculate optimal execution price for a batch of orders.
    /// Note: MPC requires statically known boundaries, so arrays must be fixed-size.
    #[instruction]
    pub fn calculate_execution_price(
        buy_orders_ctxt: Enc<Shared, [Order; 5]>,
        sell_orders_ctxt: Enc<Shared, [Order; 5]>,
    ) -> Enc<Shared, u64> {
        let buys = buy_orders_ctxt.to_arcis();
        let sells = sell_orders_ctxt.to_arcis();
        
        let mut best_price = 0u64;
        
        for i in 0..5 {
            for j in 0..5 {
                let buy_order = &buys[i];
                let sell_order = &sells[j];
                
                // Ensure they are active orders (size > 0)
                if buy_order.size > 0 && sell_order.size > 0 {
                    if buy_order.price >= sell_order.price {
                        let mid_price = (buy_order.price + sell_order.price) / 2;
                        if mid_price > best_price {
                            best_price = mid_price;
                        }
                    }
                }
            }
        }
        
        buy_orders_ctxt.owner.from_arcis(best_price)
    }
}
