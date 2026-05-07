"use client";

import { useEffect, useState } from "react";
import {
  api,
  ApiError,
  formatSignedUSD,
  formatUSD,
  type DashboardSummary,
} from "@/lib/api";

export default function DashboardPage() {
  const [data, setData] = useState<DashboardSummary | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [needsAuth, setNeedsAuth] = useState(false);

  useEffect(() => {
    api
      .get<DashboardSummary>("/dashboard/summary")
      .then(setData)
      .catch((e: ApiError) => {
        if (e.status === 401) setNeedsAuth(true);
        else setError(e.message);
      });
  }, []);

  const skeleton = !data && !error && !needsAuth;

  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">Dashboard</h1>
        <p className="text-surface-400">
          Overview of your aggregated yields and Arcium protected strategies.
        </p>
      </div>

      {needsAuth && (
        <div className="glass-card p-8 text-center">
          <p className="text-surface-300 mb-2">
            Connect your wallet and sign in to view your dashboard.
          </p>
          <p className="text-sm text-surface-500">
            Use the Connect button in the top right.
          </p>
        </div>
      )}

      {error && (
        <div className="glass-card p-4 text-sm text-red-400">
          Failed to load dashboard: {error}
        </div>
      )}

      {(skeleton || data) && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="glass-card p-6">
            <h3 className="text-sm font-medium text-surface-400 mb-1">
              Total Value Deposited
            </h3>
            <p className="text-3xl font-bold text-white">
              {data ? formatUSD(data.totalDeposited) : "—"}
            </p>
          </div>
          <div className="glass-card p-6">
            <h3 className="text-sm font-medium text-surface-400 mb-1">
              Total Yield Earned
            </h3>
            <p
              className={`text-3xl font-bold ${
                data && Number(data.totalUnrealizedPnl) < 0
                  ? "text-red-400"
                  : "text-accent"
              }`}
            >
              {data ? formatSignedUSD(data.totalUnrealizedPnl) : "—"}
            </p>
          </div>
          <div className="glass-card p-6">
            <h3 className="text-sm font-medium text-surface-400 mb-1">
              Active Positions
            </h3>
            <p className="text-3xl font-bold text-white">
              {data ? data.activePositions : "—"}
            </p>
          </div>
        </div>
      )}

      {data && data.activePositions === 0 && (
        <div className="glass-card p-8 min-h-[300px] flex items-center justify-center">
          <div className="text-center">
            <p className="text-surface-400 mb-4">
              You don&apos;t have any active positions yet.
            </p>
            <a href="/vaults" className="btn-primary">
              Browse Vaults
            </a>
          </div>
        </div>
      )}
    </div>
  );
}
