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
  const pnl = data ? Number(data.totalUnrealizedPnl) : 0;
  const deposited = data ? Number(data.totalDeposited) : 0;
  const pnlPct = deposited > 0 ? (pnl / deposited) * 100 : 0;

  return (
    <div className="space-y-10 animate-fade-in">
      <header className="space-y-3">
        <p className="eyebrow">Portfolio</p>
        <h1 className="text-4xl font-bold tracking-tight text-white md:text-5xl">
          Your <span className="gradient-text">shielded</span> dashboard.
        </h1>
        <p className="max-w-2xl text-surface-400">
          Aggregated view of every position you hold across YieldVeil vaults.
          All strategy executions remain confidential via Arcium MXE.
        </p>
      </header>

      {needsAuth && <AuthGate />}

      {error && (
        <div className="card border-red-500/30 bg-red-500/[0.04] p-5 text-sm text-red-300">
          <span className="font-semibold">Couldn&apos;t load dashboard.</span>{" "}
          {error}
        </div>
      )}

      {(skeleton || data) && (
        <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
          {/* Hero stat — Total Yield Earned, takes 2 cols */}
          <div className="card-premium p-8 lg:col-span-2">
            <div className="flex items-start justify-between">
              <div>
                <p className="text-[10px] font-semibold uppercase tracking-[0.2em] text-surface-500">
                  Total Yield Earned
                </p>
                <div className="mt-3 flex items-baseline gap-3">
                  {data ? (
                    <>
                      <span
                        className={`stat-value text-5xl md:text-6xl ${
                          pnl < 0 ? "text-red-400" : "gradient-text"
                        }`}
                      >
                        {formatSignedUSD(data.totalUnrealizedPnl)}
                      </span>
                      {deposited > 0 && (
                        <span
                          className={`text-sm font-semibold ${
                            pnl < 0 ? "text-red-400" : "text-accent"
                          }`}
                        >
                          {pnl >= 0 ? "+" : ""}
                          {pnlPct.toFixed(2)}%
                        </span>
                      )}
                    </>
                  ) : (
                    <div className="skeleton h-14 w-64" />
                  )}
                </div>
                <p className="mt-2 text-xs text-surface-500">
                  Unrealized · marked-to-market
                </p>
              </div>
              <TrendChart positive={pnl >= 0} />
            </div>
          </div>

          {/* Total deposited */}
          <SecondaryStat
            label="Total Value Deposited"
            value={data ? formatUSD(data.totalDeposited) : null}
          />

          {/* Active positions */}
          <SecondaryStat
            label="Active Positions"
            value={data ? String(data.activePositions) : null}
          />

          {/* Privacy / shielded indicator */}
          <div className="card p-6 lg:col-span-2">
            <div className="flex items-center gap-4">
              <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-accent/[0.08] ring-1 ring-accent/25">
                <svg className="h-5 w-5 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M12 3l8 4v6c0 5-3.5 8-8 8s-8-3-8-8V7l8-4z" />
                </svg>
              </div>
              <div className="min-w-0 flex-1">
                <p className="text-sm font-semibold text-white">
                  All executions Arcium-shielded
                </p>
                <p className="mt-0.5 text-xs text-surface-400">
                  Order matches and rebalance routes are encrypted before
                  hitting the chain. MEV bots and copy-traders see nothing.
                </p>
              </div>
              <span className="chip-accent shrink-0">Active</span>
            </div>
          </div>
        </div>
      )}

      {data && data.activePositions === 0 && (
        <div className="card flex min-h-[280px] items-center justify-center p-10">
          <div className="text-center">
            <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-2xl border border-white/[0.06] bg-white/[0.02]">
              <svg className="h-7 w-7 text-surface-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="1.5">
                <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 12.75l3.75 3.75 9.75-12" />
              </svg>
            </div>
            <p className="text-base font-semibold text-white">
              Ready when you are
            </p>
            <p className="mt-1 max-w-sm text-sm text-surface-400">
              Pick a vault, deposit a starter amount on devnet, watch the
              shielded rebalance flow.
            </p>
            <a href="/vaults" className="btn-primary mt-6">
              Browse Vaults
              <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14m-7-7l7 7-7 7" />
              </svg>
            </a>
          </div>
        </div>
      )}
    </div>
  );
}

function SecondaryStat({
  label,
  value,
}: {
  label: string;
  value: string | null;
}) {
  return (
    <div className="card-interactive p-6">
      <p className="text-[10px] font-semibold uppercase tracking-[0.2em] text-surface-500">
        {label}
      </p>
      {value === null ? (
        <div className="skeleton mt-3 h-9 w-32" />
      ) : (
        <p className="mt-3 stat-value text-3xl">{value}</p>
      )}
    </div>
  );
}

function TrendChart({ positive }: { positive: boolean }) {
  // Simple SVG sparkline. Static for v1; can wire to apy_snapshots later.
  const path = positive
    ? "M0,40 Q15,38 25,30 T55,24 T85,12 T110,6"
    : "M0,8 Q15,12 25,18 T55,28 T85,36 T110,42";
  const stroke = positive ? "#00f0b5" : "#f87171";
  const fill = positive ? "rgba(0,240,181,0.15)" : "rgba(248,113,113,0.15)";

  return (
    <svg
      className="h-16 w-32 shrink-0 hidden sm:block"
      viewBox="0 0 110 50"
      fill="none"
    >
      <path
        d={`${path} L110,50 L0,50 Z`}
        fill={fill}
      />
      <path d={path} stroke={stroke} strokeWidth="1.5" strokeLinecap="round" />
    </svg>
  );
}

function AuthGate() {
  return (
    <div className="card relative overflow-hidden p-12 text-center">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(59,82,252,0.1),transparent_70%)]" />
      <div className="relative">
        <div className="mx-auto mb-5 flex h-14 w-14 items-center justify-center rounded-2xl border border-brand-400/25 bg-brand-500/[0.08]">
          <svg className="h-6 w-6 text-brand-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="1.7">
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 11c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2zM3 21l3-3m11.5-2.5a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0z" />
          </svg>
        </div>
        <p className="text-lg font-semibold text-white">
          Connect your wallet to view your dashboard
        </p>
        <p className="mx-auto mt-1.5 max-w-sm text-sm text-surface-400">
          Your positions, yields, and Strategy cNFTs are tied to your wallet
          address.
        </p>
      </div>
    </div>
  );
}
