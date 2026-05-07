"use client";

import { useEffect, useState } from "react";
import {
  api,
  ApiError,
  formatAPY,
  formatSignedUSD,
  formatUSD,
  type PositionItem,
} from "@/lib/api";

export default function PositionsPage() {
  const [positions, setPositions] = useState<PositionItem[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [needsAuth, setNeedsAuth] = useState(false);

  useEffect(() => {
    api
      .get<{ positions: PositionItem[] }>("/positions")
      .then((data) => setPositions(data.positions))
      .catch((e: ApiError) => {
        if (e.status === 401) setNeedsAuth(true);
        else setError(e.message);
      });
  }, []);

  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">My Positions</h1>
        <p className="text-surface-400">
          Manage your active vault positions and view historical Strategy cNFTs.
        </p>
      </div>

      {needsAuth && (
        <div className="glass-card p-8 text-center">
          <p className="text-surface-300 mb-2">
            Connect your wallet and sign in to view your positions.
          </p>
          <p className="text-sm text-surface-500">
            Use the Connect button in the top right.
          </p>
        </div>
      )}

      {error && (
        <div className="glass-card p-4 text-sm text-red-400">
          Failed to load positions: {error}
        </div>
      )}

      {positions && positions.length === 0 && (
        <div className="glass-card p-8 min-h-[400px] flex items-center justify-center">
          <div className="text-center">
            <div className="w-16 h-16 bg-white/[0.02] border border-white/[0.08] rounded-2xl flex items-center justify-center mx-auto mb-4">
              <svg
                className="w-8 h-8 text-surface-500"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                />
              </svg>
            </div>
            <p className="text-lg text-white font-medium mb-2">
              No Active Positions
            </p>
            <p className="text-surface-400 mb-6 max-w-sm">
              When you deposit into a YieldVeil vault, your active positions and
              minted cNFTs will appear here.
            </p>
            <a href="/vaults" className="btn-primary">
              Browse Vaults
            </a>
          </div>
        </div>
      )}

      {positions && positions.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {positions.map((p) => (
            <div key={p.id} className="glass-card p-6">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <h3 className="text-lg font-bold text-white">
                    {p.vault.name}
                  </h3>
                  <p className="text-xs text-surface-400">
                    {p.vault.token.symbol} • APY {formatAPY(p.vault.currentApy)}
                  </p>
                </div>
                <span
                  className={`text-sm font-semibold ${
                    Number(p.unrealizedPnl) < 0
                      ? "text-red-400"
                      : "text-accent"
                  }`}
                >
                  {formatSignedUSD(p.unrealizedPnl)}
                </span>
              </div>
              <div className="grid grid-cols-2 gap-4 text-sm border-t border-white/[0.06] pt-4">
                <div>
                  <span className="text-surface-400 block">Deposited</span>
                  <span className="text-white font-medium">
                    {formatUSD(p.depositValue)}
                  </span>
                </div>
                <div>
                  <span className="text-surface-400 block">Current Value</span>
                  <span className="text-white font-medium">
                    {formatUSD(p.currentValue)}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
