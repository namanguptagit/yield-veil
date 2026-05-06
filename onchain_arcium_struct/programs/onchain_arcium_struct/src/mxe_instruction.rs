use arcis::*;
#[encrypted]
mod circuits {
    use arcis::*;
    // ─── Shared encrypted types

    #[derive(Copy, Clone)]
    pub struct EncOrder {
        pub is_buy: bool,
        pub price: u64,
        pub size: u64,
        pub owner: [u8; 32],
    }
    #[derive(Copy, Clone)]
    pub struct EncEscrow {
        pub balance: u64,
        pub owner: [u8; 32],
    }
    #[derive(Copy, Clone)]
    pub struct MatchResult {
        pub matched: bool,
        pub execution_price: u64,
        pub execution_size: u64,
    }
    #[derive(Copy, Clone)]
    pub struct PrivatePosition {
        pub amount: u64,
        pub protocol: u8,
        pub min_apy_bps: u64,
        pub auto_compound: bool,
        pub owner: [u8; 32],
    }
    #[derive(Copy, Clone)]
    pub struct YieldRouteResult {
        pub success: bool,
        pub deposit_amount: u64,
        pub target_protocol: u8,
        pub yield_rate_bps: u64,
    }
    #[derive(Copy, Clone)]
    struct Candidate {
        protocol: u8,
        apy: u64,
    }
    // Compile-time batch sizes (Arcis has no Vec — use fixed arrays).
    const BUY_BATCH: usize = 8;
    const SELL_BATCH: usize = 8;
    // ─── MXE Instructions

    /// Match two encrypted limit orders inside the MXE cluster.
    #[instruction]
    pub fn match_orders(
        order1: Enc<Shared, EncOrder>,
        order2: Enc<Shared, EncOrder>,
        escrow1: Enc<Mxe, EncEscrow>,
        escrow2: Enc<Mxe, EncEscrow>,
    ) -> (
        Enc<Shared, MatchResult>,
        Enc<Mxe, EncEscrow>,
        Enc<Mxe, EncEscrow>,
    ) {
        let o1 = order1.to_arcis();
        let o2 = order2.to_arcis();
        let e1 = escrow1.to_arcis();
        let e2 = escrow2.to_arcis();
        // 1. One side must be buy, the other sell.
        let directions_cross = o1.is_buy != o2.is_buy;
        // 2. Price crossing.
        let prices_cross = directions_cross
            && if o1.is_buy { o1.price >= o2.price } else { o2.price >= o1.price };
        // 3. Execution price (seller's price = passive side).
        let execution_price = if prices_cross {
            if o1.is_buy { o2.price } else { o1.price }
        } else {
            0u64
        };
        // 4. Execution size = min of the two order sizes.
        let execution_size = if prices_cross { o1.size.min(o2.size) } else { 0u64 };
        // 5. Escrow sufficiency.
        let buyer_escrow = if o1.is_buy { e1.balance } else { e2.balance };
        let seller_escrow = if o1.is_buy { e2.balance } else { e1.balance };
        let buyer_ok = prices_cross && buyer_escrow >= execution_size;
        let seller_ok = prices_cross && seller_escrow >= execution_size;
        let final_matched = prices_cross && buyer_ok && seller_ok;
        let result = MatchResult {
            matched: final_matched,
            execution_price,
            execution_size,
        };
        // 6. Update escrows. Both branches always execute in MPC; the
        // condition only selects which value flows out.
        let new_e1 = EncEscrow {
            balance: if final_matched {
                e1.balance.saturating_sub(execution_size)
            } else {
                e1.balance
            },
            owner: e1.owner,
        };
        let new_e2 = EncEscrow {
            balance: if final_matched {
                e2.balance.saturating_sub(execution_size)
            } else {
                e2.balance
            },
            owner: e2.owner,
        };
        // 7. Re-encrypt outputs to their respective owners.
        (
            order1.owner.from_arcis(result),
            escrow1.owner.from_arcis(new_e1),
            escrow2.owner.from_arcis(new_e2),
        )
    }
    /// Validate an order's parameters against the orderbook's permitted bounds.
    #[instruction]
    pub fn validate_order(
        order: Enc<Shared, EncOrder>,
        min_price: u64,
        max_price: u64,
        min_size: u64,
        max_size: u64,
    ) -> Enc<Shared, bool> {
        let o = order.to_arcis();
        let valid_price = o.price >= min_price && o.price <= max_price;
        let valid_size = o.size >= min_size && o.size <= max_size;
        let valid_owner = o.owner != [0u8; 32];
        order.owner.from_arcis(valid_price && valid_size && valid_owner)
    }
    /// Route a private yield position to the optimal protocol.
    #[instruction]
    pub fn route_yield(
        position: Enc<Shared, PrivatePosition>,
        raydium_apy_bps: u64,
        drift_apy_bps: u64,
        solend_apy_bps: u64,
        darkpool_apy_bps: u64,
    ) -> Enc<Shared, YieldRouteResult> {
        let pos = position.to_arcis();
        // 0.1% fee deducted before deposit.
        let fee_bps = 10u64;
        let fee = pos.amount.saturating_mul(fee_bps) / 10_000;
        let net = pos.amount.saturating_sub(fee);
        let candidates = [
            Candidate { protocol: 0, apy: raydium_apy_bps },
            Candidate { protocol: 1, apy: drift_apy_bps },
            Candidate { protocol: 2, apy: solend_apy_bps },
            Candidate { protocol: 3, apy: darkpool_apy_bps },
        ];
        let mut best_protocol = pos.protocol;
        let mut best_apy = 0u64;
        for c in candidates.iter() {
            let take = c.apy > best_apy && c.apy >= pos.min_apy_bps;
            best_apy = if take { c.apy } else { best_apy };
            best_protocol = if take { c.protocol } else { best_protocol };
        }
        let success = best_apy >= pos.min_apy_bps && net > 0;
        let result = YieldRouteResult {
            success,
            deposit_amount: if success { net } else { 0u64 },
            target_protocol: best_protocol,
            yield_rate_bps: best_apy,
        };
        position.owner.from_arcis(result)
    }
    /// Auto-compound: re-deposit accrued yield back into the same position.
    #[instruction]
    pub fn compound_yield(
        position: Enc<Shared, PrivatePosition>,
        accrued_yield: Enc<Mxe, u64>,
    ) -> (Enc<Shared, PrivatePosition>, Enc<Mxe, u64>) {
        let mut pos = position.to_arcis();
        let yield_amount = accrued_yield.to_arcis();
        pos.amount = pos.amount.saturating_add(yield_amount);
        let zero = 0u64;
        (
            position.owner.from_arcis(pos),
            accrued_yield.owner.from_arcis(zero),
        )
    }
    /// Calculate the midpoint execution price for a fixed-size batch.
    /// Arcis does not support `Vec`; batch sizes must be compile-time known.
    #[instruction]
    pub fn calculate_batch_midpoint(
        buy_orders: Enc<Shared, [EncOrder; BUY_BATCH]>,
        sell_orders: Enc<Shared, [EncOrder; SELL_BATCH]>,
    ) -> Enc<Shared, u64> {
        let buys = buy_orders.to_arcis();
        let sells = sell_orders.to_arcis();
        let mut best_price = 0u64;
        for buy in buys.iter() {
            for sell in sells.iter() {
                let crosses = buy.price >= sell.price;
                let mid = buy.price / 2 + sell.price / 2;
                let take = crosses && mid > best_price;
                best_price = if take { mid } else { best_price };
            }
        }
        buy_orders.owner.from_arcis(best_price)
    }
}