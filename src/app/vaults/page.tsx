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
    <div className="space-y-10 animate-fade-in">
      <header className="space-y-3">
        <p className="eyebrow">Vaults</p>
        <h1 className="text-4xl font-bold tracking-tight text-white md:text-5xl">
          <span className="gradient-text-warm">Stealth</span> yield strategies.
        </h1>
        <p className="max-w-2xl text-surface-400">
          Each vault routes capital through encrypted Arcium MPC computations.
          Deposit a token; the rebalance logic stays opaque to chain observers.
        </p>
      </header>

      {error && (
        <div className="card border-red-500/30 bg-red-500/[0.04] p-5 text-sm text-red-300">
          <span className="font-semibold">Couldn&apos;t load vaults.</span>{" "}
          {error}
        </div>
      )}

      {!vaults && !error && (
        <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
          {[0, 1].map((i) => (
            <VaultCardSkeleton key={i} />
          ))}
        </div>
      )}

      {vaults && vaults.length === 0 && (
        <div className="card p-16 text-center">
          <p className="text-surface-300">No active vaults yet.</p>
          <p className="mt-1 text-xs text-surface-500">
            Vaults will appear here once seeded.
          </p>
        </div>
      )}

      {vaults && vaults.length > 0 && (
        <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
          {vaults.map((v, i) => (
            <VaultCard key={v.id} vault={v} delay={i * 100} />
          ))}
        </div>
      )}
    </div>
  );
}

function VaultCard({ vault, delay }: { vault: VaultSummary; delay: number }) {
  return (
    <article
      className="card-premium relative p-7 animate-slide-up"
      style={{ animationDelay: `${delay}ms` }}
      data-vault-id={vault.id}
    >
      {/* Top gradient accent line */}
      <div className="pointer-events-none absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-accent/60 to-transparent" />

      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0 flex-1">
          <span className="chip-brand mb-3">{vault.token.symbol}</span>
          <h3 className="text-2xl font-bold tracking-tight text-white">
            {vault.name}
          </h3>
          <p className="mt-1.5 text-sm text-surface-400">
            {vault._count.strategies} active{" "}
            {vault._count.strategies === 1 ? "strategy" : "strategies"} ·{" "}
            {vault.token.name} denominated
          </p>
        </div>
        <ShieldBadge />
      </div>

      {/* Big APY display */}
      <div className="mt-7 flex items-baseline gap-2">
        <span className="stat-value text-5xl gradient-text">
          {formatAPY(vault.currentApy)}
        </span>
        <span className="text-xs uppercase tracking-wider text-surface-500">
          Projected APY
        </span>
      </div>

      {/* Stat row */}
      <div className="mt-6 grid grid-cols-3 gap-4 border-t border-white/[0.06] pt-5">
        <Stat label="TVL" value={formatTVL(vault.tvl)} />
        <Stat label="Positions" value={String(vault._count.positions)} />
        <Stat label="Status" value={<StatusDot status={vault.status} />} />
      </div>

      <button className="btn-primary mt-7 w-full">
        Deposit
        <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
          <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14m-7-7l7 7-7 7" />
        </svg>
      </button>
    </article>
  );
}

function Stat({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div>
      <p className="text-[10px] font-semibold uppercase tracking-[0.18em] text-surface-500">
        {label}
      </p>
      <p className="mt-1.5 stat-value text-base">{value}</p>
    </div>
  );
}

function StatusDot({ status }: { status: VaultSummary["status"] }) {
  const config = {
    active: { color: "bg-accent", label: "Active" },
    paused: { color: "bg-amber-400", label: "Paused" },
    deprecated: { color: "bg-red-400", label: "Deprecated" },
  }[status];
  return (
    <span className="flex items-center gap-2 text-sm font-medium text-white">
      <span className={`h-2 w-2 rounded-full ${config.color} ${status === "active" ? "animate-pulse-glow" : ""}`} />
      {config.label}
    </span>
  );
}

function ShieldBadge() {
  return (
    <div className="flex shrink-0 items-center gap-2 rounded-full border border-accent/25 bg-accent/[0.06] px-3 py-1.5">
      <svg className="h-4 w-4 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 3l8 4v6c0 5-3.5 8-8 8s-8-3-8-8V7l8-4z" />
      </svg>
      <span className="text-xs font-medium text-accent-300">Shielded</span>
    </div>
  );
}

function VaultCardSkeleton() {
  return (
    <div className="card p-7">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 space-y-3">
          <div className="skeleton h-5 w-14" />
          <div className="skeleton h-7 w-2/3" />
          <div className="skeleton h-3 w-1/2" />
        </div>
        <div className="skeleton h-8 w-20 rounded-full" />
      </div>
      <div className="skeleton mt-7 h-12 w-32" />
      <div className="mt-6 grid grid-cols-3 gap-4 border-t border-white/[0.06] pt-5">
        {[0, 1, 2].map((i) => (
          <div key={i} className="space-y-2">
            <div className="skeleton h-2 w-12" />
            <div className="skeleton h-4 w-16" />
          </div>
        ))}
      </div>
      <div className="skeleton mt-7 h-12 w-full" />
    </div>
  );
}
