import Link from "next/link";

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[70vh] text-center max-w-4xl mx-auto">
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-brand-500/20 rounded-full blur-[120px] pointer-events-none" />
      
      <h1 className="text-5xl md:text-7xl font-extrabold tracking-tight mb-8 z-10 animate-slide-up">
        Keep Your Alpha. <br />
        <span className="gradient-text">Maximize Your Yield.</span>
      </h1>
      
      <p className="text-xl text-surface-300 mb-10 max-w-2xl z-10 animate-fade-in animation-delay-200">
        YieldVeil is the first privacy-preserving DeFi aggregator on Solana. 
        Using Arcium MXE confidential computing, your strategy executions and rebalances remain completely hidden from MEV bots and copy-traders.
      </p>

      <div className="flex flex-col sm:flex-row gap-4 z-10 animate-fade-in animation-delay-300">
        <Link href="/vaults" className="btn-primary py-4 px-8 text-lg">
          Explore Vaults
        </Link>
        <Link href="/dashboard" className="btn-secondary py-4 px-8 text-lg">
          View Dashboard
        </Link>
      </div>

      <div className="mt-24 grid grid-cols-1 md:grid-cols-3 gap-6 w-full z-10 animate-slide-up animation-delay-500">
        <div className="glass-card p-6 text-left">
          <div className="bg-brand-500/20 w-12 h-12 rounded-xl flex items-center justify-center mb-4">
            <svg className="w-6 h-6 text-brand-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
          </div>
          <h3 className="text-lg font-semibold mb-2 text-white">Encrypted Execution</h3>
          <p className="text-sm text-surface-400">Orderbooks and AMM rebalancing executed in Arcium's trusted execution environments. No front-running.</p>
        </div>
        <div className="glass-card p-6 text-left">
          <div className="bg-accent/20 w-12 h-12 rounded-xl flex items-center justify-center mb-4">
            <svg className="w-6 h-6 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
            </svg>
          </div>
          <h3 className="text-lg font-semibold mb-2 text-white">Aggregated Yield</h3>
          <p className="text-sm text-surface-400">Smart routing across Raydium, Drift, and Orca. Let our engine find the most efficient compounding paths.</p>
        </div>
        <div className="glass-card p-6 text-left">
          <div className="bg-purple-500/20 w-12 h-12 rounded-xl flex items-center justify-center mb-4">
            <svg className="w-6 h-6 text-purple-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
          <h3 className="text-lg font-semibold mb-2 text-white">Strategy cNFTs</h3>
          <p className="text-sm text-surface-400">Every position is backed by a compressed NFT. Track your historical yields and strategy metadata on-chain.</p>
        </div>
      </div>
    </div>
  );
}
