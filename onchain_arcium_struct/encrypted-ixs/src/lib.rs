#![allow(
    clippy::implicit_saturating_sub,
    clippy::manual_saturating_arithmetic,
)]

use arcis::*;

#[encrypted]
mod circuits {
    use arcis::*;

    pub const BATCH_ORDERS_LEN: usize = 4;
    pub const BATCH_ESCROWS_LEN: usize = 4;
    pub const BATCH_POSITIONS_LEN: usize = 4;

    #[derive(Clone, Copy)]
    pub struct EncOrder {
        pub is_buy: bool,
        pub price: u64,
        pub size: u64,
        pub owner: [u8; 32],
        pub timestamp: u64,
        pub order_nonce: u64,
    }

    #[derive(Clone, Copy)]
    pub struct EncEscrow {
        pub balance: u64,
        pub fee_reserve: u64,
        pub owner: [u8; 32],
        pub last_epoch: u64,
    }

    #[derive(Clone, Copy)]
    pub struct EncPosition {
        pub amount: u64,
        pub protocol: u8,
        pub min_apy_bps: u64,
        pub max_drawdown_bps: u64,
        pub auto_compound: bool,
        pub owner: [u8; 32],
        pub open_epoch: u64,
    }

    #[derive(Clone, Copy)]
    pub struct EncAccruedYield {
        pub epoch_yield: u64,
        pub lifetime_yield: u64,
        pub last_updated_epoch: u64,
        pub owner: [u8; 32],
    }

    #[derive(Clone, Copy)]
    pub struct MatchResult {
        pub matched: bool,
        pub execution_price: u64,
        pub execution_size: u64,
        pub fee_bps: u64,
        pub order1_is_maker: bool,
    }

    #[derive(Clone, Copy)]
    pub struct RouteResult {
        pub success: bool,
        pub deposit_amount: u64,
        pub target_protocol: u8,
        pub yield_rate_bps: u64,
        pub fee_amount: u64,
    }

    #[derive(Clone, Copy)]
    pub struct RiskCheckResult {
        pub healthy: bool,
        pub health_factor_bps: u64,
        pub available_collateral: u64,
        pub should_liquidate: bool,
    }

    #[derive(Clone, Copy)]
    pub struct BatchMatchSummary {
        pub matched_pairs: u64,
        pub total_volume: u64,
        pub vwap: u64,
        pub total_fees: u64,
    }

    #[derive(Clone, Copy)]
    pub struct OraclePrice {
        pub mark_price: u64,
        pub confidence: u64,
        pub publish_slot: u64,
    }

    #[derive(Clone, Copy)]
    pub struct OrderBatch {
        pub orders: [EncOrder; BATCH_ORDERS_LEN],
    }

    #[derive(Clone, Copy)]
    pub struct EscrowBatch {
        pub escrows: [EncEscrow; BATCH_ESCROWS_LEN],
    }

    #[derive(Clone, Copy)]
    pub struct PositionBatch {
        pub positions: [EncPosition; BATCH_POSITIONS_LEN],
    }

    #[derive(Clone, Copy)]
    pub struct RouteResultBatch {
        pub results: [RouteResult; BATCH_POSITIONS_LEN],
    }

    #[derive(Clone, Copy)]
    pub struct RebalanceOutput {
        pub new_position: EncPosition,
        pub route: RouteResult,
    }

    const ZERO_ROUTE_RESULT: RouteResult = RouteResult {
        success: false,
        deposit_amount: 0,
        target_protocol: 0,
        yield_rate_bps: 0,
        fee_amount: 0,
    };

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
        const FEE_BPS: u64 = 10;
        let o1 = order1.to_arcis();
        let o2 = order2.to_arcis();
        let e1 = escrow1.to_arcis();
        let e2 = escrow2.to_arcis();

        let sides_cross = o1.is_buy != o2.is_buy;
        let prices_cross = sides_cross
            && (if o1.is_buy { o1.price >= o2.price } else { o2.price >= o1.price });

        let execution_price = if prices_cross {
            if o1.is_buy { o2.price } else { o1.price }
        } else {
            0u64
        };
        let execution_size = if prices_cross { o1.size.min(o2.size) } else { 0u64 };
        let fee = if prices_cross {
            (execution_size * FEE_BPS) / 10_000
        } else {
            0u64
        };
        let net_size = execution_size - fee;

        let (buyer_balance, seller_balance) = if o1.is_buy {
            (e1.balance, e2.balance)
        } else {
            (e2.balance, e1.balance)
        };
        let buyer_ok = prices_cross && buyer_balance >= net_size;
        let seller_ok = prices_cross && seller_balance >= execution_size;
        let final_matched = prices_cross && buyer_ok && seller_ok;
        let order1_is_maker = o1.timestamp <= o2.timestamp;

        let result = MatchResult {
            matched: final_matched,
            execution_price,
            execution_size: if final_matched { net_size } else { 0 },
            fee_bps: FEE_BPS,
            order1_is_maker,
        };

        let (new_e1, new_e2) = if final_matched {
            if o1.is_buy {
                let updated_buyer = EncEscrow {
                    balance: e1.balance - net_size,
                    fee_reserve: e1.fee_reserve + fee,
                    owner: e1.owner,
                    last_epoch: e1.last_epoch,
                };
                let updated_seller = EncEscrow {
                    balance: e2.balance - execution_size + net_size,
                    fee_reserve: e2.fee_reserve,
                    owner: e2.owner,
                    last_epoch: e2.last_epoch,
                };
                (updated_buyer, updated_seller)
            } else {
                let updated_seller = EncEscrow {
                    balance: e1.balance - execution_size + net_size,
                    fee_reserve: e1.fee_reserve,
                    owner: e1.owner,
                    last_epoch: e1.last_epoch,
                };
                let updated_buyer = EncEscrow {
                    balance: e2.balance - net_size,
                    fee_reserve: e2.fee_reserve + fee,
                    owner: e2.owner,
                    last_epoch: e2.last_epoch,
                };
                (updated_seller, updated_buyer)
            }
        } else {
            (e1, e2)
        };

        (
            order1.owner.from_arcis(result),
            escrow1.owner.from_arcis(new_e1),
            escrow2.owner.from_arcis(new_e2),
        )
    }

    #[instruction]
    pub fn validate_order(
        order: Enc<Shared, EncOrder>,
        min_price: u64,
        max_price: u64,
        min_size: u64,
        max_size: u64,
    ) -> Enc<Shared, bool> {
        let o = order.to_arcis();
        let price_ok = o.price >= min_price && o.price <= max_price;
        let size_ok = o.size >= min_size && o.size <= max_size;
        let owner_ok = o.owner != [0u8; 32];
        let nonce_ok = o.order_nonce != 0;
        order.owner.from_arcis(price_ok && size_ok && owner_ok && nonce_ok)
    }

    #[instruction]
    pub fn calculate_vwap(
        buy_orders: Enc<Shared, OrderBatch>,
        sell_orders: Enc<Shared, OrderBatch>,
    ) -> Enc<Shared, u64> {
        let buys = buy_orders.to_arcis();
        let sells = sell_orders.to_arcis();
        let mut total_volume: u64 = 0;
        let mut weighted_price_sum: u64 = 0;
        for bi in 0..BATCH_ORDERS_LEN {
            for si in 0..BATCH_ORDERS_LEN {
                let buy = buys.orders[bi];
                let sell = sells.orders[si];
                let active = buy.size > 0 && sell.size > 0;
                if active && buy.price >= sell.price {
                    let exec_price = sell.price;
                    let exec_size = buy.size.min(sell.size);
                    weighted_price_sum += exec_price * exec_size;
                    total_volume += exec_size;
                }
            }
        }
        let vwap = if total_volume > 0 {
            weighted_price_sum / total_volume
        } else {
            0u64
        };
        buy_orders.owner.from_arcis(vwap)
    }

    #[instruction]
    pub fn route_yield(
        position: Enc<Shared, EncPosition>,
        raydium_apy_bps: u64,
        drift_apy_bps: u64,
        solend_apy_bps: u64,
        dark_pool_apy_bps: u64,
    ) -> Enc<Shared, RouteResult> {
        const FEE_BPS: u64 = 10;
        const MAX_PROTO: u8 = 3;
        let pos = position.to_arcis();
        let fee_amount = (pos.amount * FEE_BPS) / 10_000;
        let net_deposit = pos.amount - fee_amount;
        let apy_table: [u64; 4] = [
            raydium_apy_bps,
            drift_apy_bps,
            solend_apy_bps,
            dark_pool_apy_bps,
        ];
        let preferred = pos.protocol.min(MAX_PROTO) as usize;
        let mut best_protocol = preferred as u8;
        let mut best_apy = 0u64;
        if apy_table[preferred] >= pos.min_apy_bps {
            best_apy = apy_table[preferred];
        }
        for (i, &apy) in apy_table.iter().enumerate() {
            if apy > best_apy && apy >= pos.min_apy_bps {
                best_apy = apy;
                best_protocol = i as u8;
            }
        }
        let success = best_apy >= pos.min_apy_bps && net_deposit > 0;
        let result = RouteResult {
            success,
            deposit_amount: if success { net_deposit } else { 0 },
            target_protocol: if success { best_protocol } else { pos.protocol },
            yield_rate_bps: best_apy,
            fee_amount,
        };
        position.owner.from_arcis(result)
    }

    #[instruction]
    pub fn rebalance_position(
        position: Enc<Shared, EncPosition>,
        raydium_apy_bps: u64,
        drift_apy_bps: u64,
        solend_apy_bps: u64,
        dark_pool_apy_bps: u64,
    ) -> Enc<Shared, RebalanceOutput> {
        const FEE_BPS: u64 = 5;
        let pos = position.to_arcis();
        let apy_table: [u64; 4] = [
            raydium_apy_bps,
            drift_apy_bps,
            solend_apy_bps,
            dark_pool_apy_bps,
        ];
        let current_apy = apy_table[pos.protocol as usize];
        let needs_rebalance = current_apy < pos.min_apy_bps;
        let (new_protocol, new_apy) = if needs_rebalance {
            let mut best_p = pos.protocol;
            let mut best_apy = 0u64;
            for (i, &apy) in apy_table.iter().enumerate() {
                if apy > best_apy {
                    best_apy = apy;
                    best_p = i as u8;
                }
            }
            (best_p, best_apy)
        } else {
            (pos.protocol, current_apy)
        };
        let fee = (pos.amount * FEE_BPS) / 10_000;
        let net_amount = pos.amount - fee;
        let new_pos = EncPosition {
            amount: net_amount,
            protocol: new_protocol,
            min_apy_bps: pos.min_apy_bps,
            max_drawdown_bps: pos.max_drawdown_bps,
            auto_compound: pos.auto_compound,
            owner: pos.owner,
            open_epoch: pos.open_epoch,
        };
        let route_result = RouteResult {
            success: needs_rebalance,
            deposit_amount: net_amount,
            target_protocol: new_protocol,
            yield_rate_bps: new_apy,
            fee_amount: fee,
        };
        position.owner.from_arcis(RebalanceOutput {
            new_position: new_pos,
            route: route_result,
        })
    }

    #[instruction]
    pub fn compound_yield(
        position: Enc<Shared, EncPosition>,
        accrued_yield: Enc<Mxe, EncAccruedYield>,
        current_epoch: u64,
    ) -> (Enc<Shared, EncPosition>, Enc<Mxe, EncAccruedYield>) {
        let mut pos = position.to_arcis();
        let yield_ = accrued_yield.to_arcis();
        let should_compound = pos.auto_compound && yield_.epoch_yield > 0;
        let (new_amount, new_lifetime) = if should_compound {
            (
                pos.amount + yield_.epoch_yield,
                yield_.lifetime_yield + yield_.epoch_yield,
            )
        } else {
            (pos.amount, yield_.lifetime_yield)
        };
        pos.amount = new_amount;
        let reset_yield = EncAccruedYield {
            epoch_yield: 0,
            lifetime_yield: new_lifetime,
            last_updated_epoch: current_epoch,
            owner: yield_.owner,
        };
        (
            position.owner.from_arcis(pos),
            accrued_yield.owner.from_arcis(reset_yield),
        )
    }

    #[instruction]
    pub fn accrue_epoch_yield(
        position: Enc<Shared, EncPosition>,
        accrued_yield: Enc<Mxe, EncAccruedYield>,
        epoch_yield_bps: u64,
        current_epoch: u64,
    ) -> Enc<Mxe, EncAccruedYield> {
        let pos = position.to_arcis();
        let yield_ = accrued_yield.to_arcis();
        let new_epoch_yield = (pos.amount * epoch_yield_bps) / 10_000;
        let updated = EncAccruedYield {
            epoch_yield: yield_.epoch_yield + new_epoch_yield,
            lifetime_yield: yield_.lifetime_yield + new_epoch_yield,
            last_updated_epoch: current_epoch,
            owner: yield_.owner,
        };
        accrued_yield.owner.from_arcis(updated)
    }

    #[instruction]
    pub fn withdraw_yield(
        position: Enc<Shared, EncPosition>,
        accrued_yield: Enc<Mxe, EncAccruedYield>,
        current_epoch: u64,
    ) -> (Enc<Shared, u64>, Enc<Mxe, EncAccruedYield>) {
        let _pos = position.to_arcis();
        let yield_ = accrued_yield.to_arcis();
        let withdrawal_amount = yield_.epoch_yield;
        let zeroed = EncAccruedYield {
            epoch_yield: 0,
            lifetime_yield: yield_.lifetime_yield,
            last_updated_epoch: current_epoch,
            owner: yield_.owner,
        };
        (
            position.owner.from_arcis(withdrawal_amount),
            accrued_yield.owner.from_arcis(zeroed),
        )
    }

    #[instruction]
    pub fn close_position(
        position: Enc<Shared, EncPosition>,
        accrued_yield: Enc<Mxe, EncAccruedYield>,
    ) -> Enc<Shared, u64> {
        let pos = position.to_arcis();
        let yield_ = accrued_yield.to_arcis();
        let total_payout = pos.amount + yield_.epoch_yield;
        position.owner.from_arcis(total_payout)
    }

    #[instruction]
    pub fn check_position_health(
        position: Enc<Shared, EncPosition>,
        escrow: Enc<Mxe, EncEscrow>,
        oracle: OraclePrice,
        leverage: u64,
    ) -> Enc<Shared, RiskCheckResult> {
        let pos = position.to_arcis();
        let esc = escrow.to_arcis();
        let collateral_value = (esc.balance * oracle.mark_price) / 1_000_000_000;
        let required_collateral = if leverage > 0 {
            (pos.amount * 10_000) / leverage
        } else {
            u64::MAX
        };
        let health_factor_bps = if required_collateral > 0 {
            (collateral_value * 10_000) / required_collateral
        } else {
            10_000
        };
        let available_collateral = if collateral_value > required_collateral {
            collateral_value - required_collateral
        } else {
            0
        };
        let should_liquidate = health_factor_bps < 10_500;
        let result = RiskCheckResult {
            healthy: health_factor_bps >= 10_000,
            health_factor_bps,
            available_collateral,
            should_liquidate,
        };
        position.owner.from_arcis(result)
    }

    #[instruction]
    pub fn max_safe_withdrawal(
        position: Enc<Shared, EncPosition>,
        escrow: Enc<Mxe, EncEscrow>,
        oracle: OraclePrice,
        leverage: u64,
        buffer_bps: u64,
    ) -> Enc<Shared, u64> {
        let pos = position.to_arcis();
        let esc = escrow.to_arcis();
        let collateral_value = (esc.balance * oracle.mark_price) / 1_000_000_000;
        let min_required = if leverage > 0 {
            (pos.amount * (10_000 + buffer_bps)) / leverage
        } else {
            u64::MAX
        };
        let max_withdraw = if collateral_value > min_required {
            collateral_value - min_required
        } else {
            0u64
        };
        position.owner.from_arcis(max_withdraw)
    }

    #[instruction]
    pub fn produce_settlement(
        order1: Enc<Shared, EncOrder>,
        order2: Enc<Shared, EncOrder>,
        match_result: Enc<Shared, MatchResult>,
    ) -> (Enc<Shared, u64>, Enc<Shared, u64>) {
        let o1 = order1.to_arcis();
        let result = match_result.to_arcis();
        let (amount_1_to_2, amount_2_to_1) = if result.matched {
            if o1.is_buy {
                (result.execution_size, 0u64)
            } else {
                (0u64, result.execution_size)
            }
        } else {
            (0u64, 0u64)
        };
        (
            order1.owner.from_arcis(amount_1_to_2),
            order2.owner.from_arcis(amount_2_to_1),
        )
    }

    #[instruction]
    pub fn produce_yield_settlement(
        position: Enc<Shared, EncPosition>,
        accrued_yield: Enc<Mxe, EncAccruedYield>,
    ) -> Enc<Shared, u64> {
        let pos = position.to_arcis();
        let yield_ = accrued_yield.to_arcis();
        let total = pos.amount + yield_.epoch_yield;
        position.owner.from_arcis(total)
    }

    #[instruction]
    pub fn batch_match_orders(
        buy_orders: Enc<Shared, OrderBatch>,
        sell_orders: Enc<Shared, OrderBatch>,
        buy_escrows: Enc<Mxe, EscrowBatch>,
        sell_escrows: Enc<Mxe, EscrowBatch>,
    ) -> (
        Enc<Shared, BatchMatchSummary>,
        Enc<Mxe, EscrowBatch>,
        Enc<Mxe, EscrowBatch>,
    ) {
        const FEE_BPS: u64 = 10;
        let buys = buy_orders.to_arcis();
        let sells = sell_orders.to_arcis();
        let mut be = buy_escrows.to_arcis();
        let mut se = sell_escrows.to_arcis();
        let mut matched_pairs: u64 = 0;
        let mut total_volume: u64 = 0;
        let mut total_fees: u64 = 0;
        let mut weighted_sum: u64 = 0;

        for bi in 0..BATCH_ORDERS_LEN {
            for si in 0..BATCH_ORDERS_LEN {
                let buy = buys.orders[bi];
                let sell = sells.orders[si];
                let active = be.escrows[bi].balance > 0
                    && se.escrows[si].balance > 0
                    && buy.size > 0
                    && sell.size > 0;
                if active && buy.price >= sell.price {
                    let exec_price = sell.price;
                    let exec_size = buy
                        .size
                        .min(sell.size)
                        .min(be.escrows[bi].balance)
                        .min(se.escrows[si].balance);
                    let fee = (exec_size * FEE_BPS) / 10_000;
                    let net = exec_size - fee;
                    be.escrows[bi].balance -= net;
                    be.escrows[bi].fee_reserve += fee;
                    se.escrows[si].balance = se.escrows[si].balance - exec_size + net;
                    matched_pairs += 1;
                    total_volume += net;
                    total_fees += fee;
                    weighted_sum += exec_price * net;
                }
            }
        }

        let vwap = if total_volume > 0 {
            weighted_sum / total_volume
        } else {
            0
        };
        let summary = BatchMatchSummary {
            matched_pairs,
            total_volume,
            vwap,
            total_fees,
        };

        (
            buy_orders.owner.from_arcis(summary),
            buy_escrows.owner.from_arcis(be),
            sell_escrows.owner.from_arcis(se),
        )
    }

    #[instruction]
    pub fn batch_route_yield(
        positions: Enc<Shared, PositionBatch>,
        raydium_apy_bps: u64,
        drift_apy_bps: u64,
        solend_apy_bps: u64,
        dark_pool_apy_bps: u64,
    ) -> Enc<Shared, RouteResultBatch> {
        const FEE_BPS: u64 = 10;
        let pos_batch = positions.to_arcis();
        let apy_table: [u64; 4] = [
            raydium_apy_bps,
            drift_apy_bps,
            solend_apy_bps,
            dark_pool_apy_bps,
        ];
        let mut results = [ZERO_ROUTE_RESULT; BATCH_POSITIONS_LEN];

        for (i, slot) in results.iter_mut().enumerate() {
            let pos = pos_batch.positions[i];
            let fee_amount = (pos.amount * FEE_BPS) / 10_000;
            let net_deposit = pos.amount - fee_amount;
            let preferred = pos.protocol.min(3) as usize;
            let mut best_protocol = preferred as u8;
            let mut best_apy = if apy_table[preferred] >= pos.min_apy_bps {
                apy_table[preferred]
            } else {
                0u64
            };
            for (j, &apy) in apy_table.iter().enumerate() {
                if apy > best_apy && apy >= pos.min_apy_bps {
                    best_apy = apy;
                    best_protocol = j as u8;
                }
            }
            let success = best_apy >= pos.min_apy_bps && net_deposit > 0;
            *slot = RouteResult {
                success,
                deposit_amount: if success { net_deposit } else { 0 },
                target_protocol: if success { best_protocol } else { pos.protocol },
                yield_rate_bps: best_apy,
                fee_amount,
            };
        }

        positions.owner.from_arcis(RouteResultBatch { results })
    }
}
