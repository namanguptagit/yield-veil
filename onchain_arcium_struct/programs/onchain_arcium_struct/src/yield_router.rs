
#[cfg(not(target_arch = "bpf"))]
pub mod client {
    use crate::state::Protocol;



    #[derive(Debug, Clone)]
    pub struct ProtocolApy {
        pub raydium:   u64,
        pub drift:     u64,
        pub solend:    u64,
        pub dark_pool: u64,
    }

    impl ProtocolApy {

        pub fn best_above(&self, min_apy_bps: u64) -> Option<(Protocol, u64)> {
            let candidates = [
                (Protocol::Raydium,  self.raydium),
                (Protocol::Drift,    self.drift),
                (Protocol::Solend,   self.solend),
                (Protocol::DarkPool, self.dark_pool),
            ];

            candidates
                .iter()
                .filter(|(_, apy)| *apy >= min_apy_bps)
                .max_by_key(|(_, apy)| apy)
                .map(|(p, a)| (*p, *a))
        }


        pub fn blended(&self, weights: [u64; 4]) -> u64 {
            let total: u64 = weights.iter().sum();
            if total == 0 { return 0; }
            let sum = self.raydium   * weights[0]
                + self.drift     * weights[1]
                + self.solend    * weights[2]
                + self.dark_pool * weights[3];
            sum / total
        }
    }


    pub fn optimal_allocation(
        apy:                &ProtocolApy,
        risk_tolerance_bps: u64,
    ) -> [u64; 4] {

        let protocol_risk: [u64; 4] = [5_000, 7_500, 2_500, 9_000];

        let mut weights = [0u64; 4];
        let apys = [apy.raydium, apy.drift, apy.solend, apy.dark_pool];

        for i in 0..4 {

            if protocol_risk[i] <= risk_tolerance_bps {
                // Weight by APY.
                weights[i] = apys[i];
            }
        }

        let total: u64 = weights.iter().sum();
        if total == 0 {

            return [0, 0, 10_000, 0];
        }


        for w in weights.iter_mut() {
            *w = (*w * 10_000) / total;
        }


        let remainder = 10_000u64.saturating_sub(weights.iter().sum());
        let max_idx   = weights
            .iter()
            .enumerate()
            .max_by_key(|(_, &w)| w)
            .map(|(i, _)| i)
            .unwrap_or(2);
        weights[max_idx] += remainder;

        weights
    }


    pub fn privacy_score(hops: u8, auto_compound: bool, public_relayer: bool) -> f64 {
        let base        = 60.0_f64;
        let hop_bonus   = (hops as f64).min(5.0) * 6.0;
        let compound_b  = if auto_compound { 8.0 } else { 0.0 };
        let relayer_b   = if public_relayer { 12.0 } else { 4.0 };
        (base + hop_bonus + compound_b + relayer_b).min(100.0)
    }
}