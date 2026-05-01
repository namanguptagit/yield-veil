export default function DashboardPage() {
  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">Dashboard</h1>
        <p className="text-surface-400">Overview of your aggregated yields and Arcium protected strategies.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="glass-card p-6">
          <h3 className="text-sm font-medium text-surface-400 mb-1">Total Value Deposited</h3>
          <p className="text-3xl font-bold text-white">$0.00</p>
        </div>
        <div className="glass-card p-6">
          <h3 className="text-sm font-medium text-surface-400 mb-1">Total Yield Earned</h3>
          <p className="text-3xl font-bold text-accent">+$0.00</p>
        </div>
        <div className="glass-card p-6">
          <h3 className="text-sm font-medium text-surface-400 mb-1">Active Positions</h3>
          <p className="text-3xl font-bold text-white">0</p>
        </div>
      </div>

      <div className="glass-card p-8 min-h-[300px] flex items-center justify-center">
        <div className="text-center">
          <p className="text-surface-400 mb-4">You don't have any active positions yet.</p>
          <a href="/vaults" className="btn-primary">Browse Vaults</a>
        </div>
      </div>
    </div>
  );
}
