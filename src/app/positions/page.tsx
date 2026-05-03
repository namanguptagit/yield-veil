export default function PositionsPage() {
  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">My Positions</h1>
        <p className="text-surface-400">Manage your active vault positions and view historical Strategy cNFTs.</p>
      </div>

      <div className="glass-card p-8 min-h-[400px] flex items-center justify-center">
        <div className="text-center">
          <div className="w-16 h-16 bg-white/[0.02] border border-white/[0.08] rounded-2xl flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8 text-surface-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
          <p className="text-lg text-white font-medium mb-2">No Active Positions</p>
          <p className="text-surface-400 mb-6 max-w-sm">When you deposit into a YieldVeil vault, your active positions and minted cNFTs will appear here.</p>
          <a href="/vaults" className="btn-primary">Browse Vaults</a>
        </div>
      </div>
    </div>
  );
}
