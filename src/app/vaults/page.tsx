"use client";

import { useEffect, useState } from "react";
import {
  api,
  ApiError,
  formatAPY,
  formatTVL,
  type VaultSummary,
} from "@/lib/api";

export default function VaultsPage() {
  const [vaults, setVaults] = useState<VaultSummary[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api
      .get<{ vaults: VaultSummary[] }>("/vaults")
      .then((data) => setVaults(data.vaults))
      .catch((e: ApiError) => setError(e.message));
  }, []);

  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">Yield Vaults</h1>
        <p className="text-surface-400">
          Deposit into smart strategies. Executions are stealth-routed via
          Arcium MXE.
        </p>
      </div>

      {error && (
        <div className="glass-card p-4 text-sm text-red-400">
          Failed to load vaults: {error}
        </div>
      )}

      {!vaults && !error && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {[0, 1].map((i) => (
            <div
              key={i}
              className="glass-card p-6 h-48 animate-pulse bg-white/[0.02]"
            />
          ))}
        </div>
      )}

      {vaults && vaults.length === 0 && (
        <div className="glass-card p-8 text-center text-surface-400">
          No active vaults yet.
        </div>
      )}

      {vaults && vaults.length > 0 && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {vaults.map((v) => (
            <div
              key={v.id}
              className="glass-card p-6 flex flex-col gap-6"
              data-vault-id={v.id}
            >
              <div className="flex justify-between items-start">
                <div>
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="text-xl font-bold text-white">{v.name}</h3>
                    <span className="bg-brand-500/20 text-brand-300 text-xs px-2 py-0.5 rounded-full font-medium">
                      {v.token.symbol}
                    </span>
                  </div>
                  <p className="text-sm text-surface-400">
                    {v._count.strategies} active strategies • {v.token.name}{" "}
                    denominated
                  </p>
                </div>
                <div className="text-right">
                  <span className="block text-2xl font-bold text-accent">
                    {formatAPY(v.currentApy)}
                  </span>
                  <span className="text-xs text-surface-400">Projected APY</span>
                </div>
              </div>

              <div className="flex justify-between items-center text-sm border-y border-white/[0.06] py-3">
                <div>
                  <span className="text-surface-400 block">TVL</span>
                  <span className="text-white font-medium">
                    {formatTVL(v.tvl)}
                  </span>
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
          ))}
        </div>
      )}
    </div>
  );
}
