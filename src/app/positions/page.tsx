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
    <div className="space-y-10 animate-fade-in">
      <header className="space-y-3">
        <p className="eyebrow">Positions</p>
        <h1 className="text-4xl font-bold tracking-tight text-white md:text-5xl">
          Active <span className="gradient-text">vault holdings.</span>
        </h1>
        <p className="max-w-2xl text-surface-400">
          Each row is a vault you&apos;ve deposited into. Strategy NFTs (cNFTs)
          track your historical participation on-chain.
        </p>
      </header>

      {needsAuth && (
        <div className="card relative overflow-hidden p-12 text-center">
          <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(59,82,252,0.1),transparent_70%)]" />
          <div className="relative">
            <div className="mx-auto mb-5 flex h-14 w-14 items-center justify-center rounded-2xl border border-brand-400/25 bg-brand-500/[0.08]">
              <svg className="h-6 w-6 text-brand-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="1.7">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 11c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2zM3 21l3-3m11.5-2.5a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0z" />
              </svg>
            </div>
            <p className="text-lg font-semibold text-white">
              Connect your wallet to view your positions
            </p>
            <p className="mx-auto mt-1.5 max-w-sm text-sm text-surface-400">
              Your positions and Strategy cNFTs are tied to your wallet address.
            </p>
          </div>
        </div>
      )}

      {error && (
        <div className="card border-red-500/30 bg-red-500/[0.04] p-5 text-sm text-red-300">
          <span className="font-semibold">Couldn&apos;t load positions.</span>{" "}
          {error}
        </div>
      )}

      {!positions && !error && !needsAuth && (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
          {[0, 1].map((i) => (
            <PositionCardSkeleton key={i} />
          ))}
        </div>
      )}

      {positions && positions.length === 0 && (
        <div className="card flex min-h-[400px] items-center justify-center p-10">
          <div className="text-center">
            <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-2xl border border-white/[0.06] bg-white/[0.02]">
              <svg className="h-7 w-7 text-surface-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="1.5">
                <path strokeLinecap="round" strokeLinejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </div>
            <p className="text-base font-semibold text-white">
              No active positions
            </p>
            <p className="mx-auto mt-1.5 max-w-sm text-sm text-surface-400">
              When you deposit into a YieldVeil vault, your active positions
              and minted cNFTs will appear here.
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

      {positions && positions.length > 0 && (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
          {positions.map((p, i) => (
            <PositionCard key={p.id} position={p} delay={i * 100} />
          ))}
        </div>
      )}
    </div>
  );
}

function PositionCard({
  position,
  delay,
}: {
  position: PositionItem;
  delay: number;
}) {
  const pnl = Number(position.unrealizedPnl);
  const deposited = Number(position.depositValue);
  const pnlPct = deposited > 0 ? (pnl / deposited) * 100 : 0;
  const positive = pnl >= 0;

  return (
    <article
      className="card-premium p-6 animate-slide-up"
      style={{ animationDelay: `${delay}ms` }}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <span className="chip-brand mb-2 inline-flex">
            {position.vault.token.symbol}
          </span>
          <h3 className="truncate text-lg font-bold text-white">
            {position.vault.name}
          </h3>
          <p className="mt-1 text-xs text-surface-400">
            APY {formatAPY(position.vault.currentApy)}
          </p>
        </div>
        <div className="text-right">
          <span
            className={`stat-value text-xl ${
              positive ? "text-accent" : "text-red-400"
            }`}
          >
            {formatSignedUSD(position.unrealizedPnl)}
          </span>
          <p
            className={`mt-0.5 text-[11px] font-semibold ${
              positive ? "text-accent-300" : "text-red-300"
            }`}
          >
            {positive ? "+" : ""}
            {pnlPct.toFixed(2)}%
          </p>
        </div>
      </div>

      <div className="mt-5 grid grid-cols-2 gap-4 border-t border-white/[0.06] pt-4 text-sm">
        <div>
          <span className="block text-[10px] font-semibold uppercase tracking-[0.18em] text-surface-500">
            Deposited
          </span>
          <span className="mt-1.5 stat-value text-base">
            {formatUSD(position.depositValue)}
          </span>
        </div>
        <div>
          <span className="block text-[10px] font-semibold uppercase tracking-[0.18em] text-surface-500">
            Current Value
          </span>
          <span className="mt-1.5 stat-value text-base">
            {formatUSD(position.currentValue)}
          </span>
        </div>
      </div>
    </article>
  );
}

function PositionCardSkeleton() {
  return (
    <div className="card p-6">
      <div className="flex items-start justify-between gap-3">
        <div className="flex-1 space-y-2">
          <div className="skeleton h-5 w-14" />
          <div className="skeleton h-5 w-2/3" />
          <div className="skeleton h-3 w-1/3" />
        </div>
        <div className="space-y-2 text-right">
          <div className="skeleton ml-auto h-5 w-20" />
          <div className="skeleton ml-auto h-3 w-12" />
        </div>
      </div>
      <div className="mt-5 grid grid-cols-2 gap-4 border-t border-white/[0.06] pt-4">
        {[0, 1].map((i) => (
          <div key={i} className="space-y-2">
            <div className="skeleton h-2 w-16" />
            <div className="skeleton h-4 w-20" />
          </div>
        ))}
      </div>
    </div>
  );
}
