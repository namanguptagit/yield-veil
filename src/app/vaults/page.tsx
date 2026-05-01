export default function VaultsPage() {
  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">Yield Vaults</h1>
        <p className="text-surface-400">Deposit into smart strategies. Executions are stealth-routed via Arcium MXE.</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Placeholder Vault Card 1 */}
        <div className="glass-card p-6 flex flex-col gap-6">
          <div className="flex justify-between items-start">
            <div>
              <div className="flex items-center gap-2 mb-1">
                <h3 className="text-xl font-bold text-white">SOL-USDC Yield</h3>
                <span className="bg-brand-500/20 text-brand-300 text-xs px-2 py-0.5 rounded-full font-medium">Low Risk</span>
              </div>
              <p className="text-sm text-surface-400">Automated Raydium + Orca LP compounding</p>
            </div>
            <div className="text-right">
              <span className="block text-2xl font-bold text-accent">24.5%</span>
              <span className="text-xs text-surface-400">Projected APY</span>
            </div>
          </div>
          
          <div className="flex justify-between items-center text-sm border-y border-white/[0.06] py-3">
            <div>
              <span className="text-surface-400 block">TVL</span>
              <span className="text-white font-medium">$1.2M</span>
            </div>
            <div>
              <span className="text-surface-400 block">Privacy</span>
              <span className="text-white font-medium flex items-center gap-1">
                <div className="w-2 h-2 rounded-full bg-accent animate-pulse-glow" /> 
                Arcium Shielded
              </span>
            </div>
          </div>

          <button className="btn-secondary w-full">Deposit</button>
        </div>

        {/* Placeholder Vault Card 2 */}
        <div className="glass-card p-6 flex flex-col gap-6">
          <div className="flex justify-between items-start">
            <div>
              <div className="flex items-center gap-2 mb-1">
                <h3 className="text-xl font-bold text-white">Delta-Neutral Drift</h3>
                <span className="bg-purple-500/20 text-purple-300 text-xs px-2 py-0.5 rounded-full font-medium">Med Risk</span>
              </div>
              <p className="text-sm text-surface-400">Funding rate arbitrage on Drift perps</p>
            </div>
            <div className="text-right">
              <span className="block text-2xl font-bold text-accent">31.2%</span>
              <span className="text-xs text-surface-400">Projected APY</span>
            </div>
          </div>
          
          <div className="flex justify-between items-center text-sm border-y border-white/[0.06] py-3">
            <div>
              <span className="text-surface-400 block">TVL</span>
              <span className="text-white font-medium">$840k</span>
            </div>
            <div>
              <span className="text-surface-400 block">Privacy</span>
              <span className="text-white font-medium flex items-center gap-1">
                <div className="w-2 h-2 rounded-full bg-accent animate-pulse-glow" /> 
                Arcium Shielded
              </span>
            </div>
          </div>

          <button className="btn-secondary w-full">Deposit</button>
        </div>
      </div>
    </div>
  );
}
