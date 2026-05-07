import Link from "next/link";

export default function Home() {
  return (
    <div className="space-y-32">
      <Hero />
      <HowItWorks />
      <Differentiators />
      <FinalCTA />
    </div>
  );
}

function Hero() {
  return (
    <section className="relative pt-8">
      {/* Hero accent blobs */}
      <div className="pointer-events-none absolute -top-20 left-1/2 -z-10 h-[520px] w-[820px] -translate-x-1/2 rounded-full bg-brand-500/25 blur-[140px] animate-blob-drift" />
      <div className="pointer-events-none absolute -top-10 left-[20%] -z-10 h-[280px] w-[280px] rounded-full bg-accent/20 blur-[100px] animate-blob-drift animation-delay-500" />

      <div className="mx-auto max-w-5xl text-center">
        <div className="animate-fade-in">
          <span className="inline-flex items-center gap-2 rounded-full border border-white/[0.08] bg-white/[0.03] px-3 py-1.5 text-xs font-medium text-surface-300 backdrop-blur-md">
            <span className="relative flex h-2 w-2">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-accent opacity-75" />
              <span className="relative inline-flex h-2 w-2 rounded-full bg-accent" />
            </span>
            Live on Solana devnet · Powered by Arcium MXE
          </span>
        </div>

        <h1 className="mt-8 text-5xl font-extrabold tracking-tighter sm:text-6xl md:text-7xl lg:text-[88px] lg:leading-[0.95] animate-slide-up animation-delay-100">
          <span className="gradient-text-warm">Keep your alpha.</span>
          <br />
          <span className="gradient-text">Maximize your yield.</span>
        </h1>

        <p className="mx-auto mt-8 max-w-2xl text-lg text-surface-300 md:text-xl animate-fade-in animation-delay-300">
          The first privacy-preserving DeFi yield aggregator on Solana.
          Strategy executions and rebalances run inside Arcium MXE — invisible
          to MEV bots and copy-traders.
        </p>

        <div className="mt-10 flex flex-col items-center justify-center gap-3 sm:flex-row sm:gap-4 animate-fade-in animation-delay-500">
          <Link href="/vaults" className="btn-primary px-8 py-4 text-base">
            Explore Vaults
            <ArrowRight />
          </Link>
          <Link href="/dashboard" className="btn-secondary px-8 py-4 text-base">
            View Dashboard
          </Link>
        </div>

        {/* Inline stats / proof bar */}
        <div className="mt-16 grid grid-cols-3 gap-6 border-t border-white/[0.06] pt-10 animate-fade-in animation-delay-700">
          <ProofStat value="$2.04M" label="Aggregate TVL" />
          <ProofStat value="27.8%" label="Avg APY" accent />
          <ProofStat value="100%" label="Shielded txs" />
        </div>
      </div>
    </section>
  );
}

function ProofStat({ value, label, accent }: { value: string; label: string; accent?: boolean }) {
  return (
    <div className="text-center sm:text-left">
      <p className={`stat-value text-2xl md:text-3xl ${accent ? "gradient-text" : ""}`}>
        {value}
      </p>
      <p className="mt-1 text-xs uppercase tracking-wider text-surface-500">
        {label}
      </p>
    </div>
  );
}

function HowItWorks() {
  const steps = [
    {
      num: "01",
      title: "Deposit shielded",
      body: "Your token deposit enters an Arcium-encrypted escrow. Amounts and routing decisions become opaque to chain observers.",
    },
    {
      num: "02",
      title: "MPC routing",
      body: "Arcium's MPC cluster runs the strategy logic. The chain sees a queued computation, not the actual rebalance plan.",
    },
    {
      num: "03",
      title: "Yield, claimed",
      body: "Confirmed routing plans execute on Raydium, Orca, or Drift. Your position appreciates while the strategy stays private.",
    },
  ];

  return (
    <section className="mx-auto max-w-6xl">
      <div className="mb-12 text-center">
        <p className="eyebrow justify-center">How it works</p>
        <h2 className="mt-4 text-4xl font-bold tracking-tight text-white md:text-5xl">
          Three steps. <span className="text-surface-400">Zero leakage.</span>
        </h2>
      </div>

      <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
        {steps.map((s, i) => (
          <div
            key={s.num}
            className="card-premium p-7 animate-slide-up"
            style={{ animationDelay: `${i * 100}ms` }}
          >
            <div className="mb-6 flex items-start justify-between">
              <span className="font-mono text-xs font-bold tracking-wider text-accent-300">
                {s.num}
              </span>
              <div className="h-px flex-1 ml-4 mt-2 bg-gradient-to-r from-white/[0.08] to-transparent" />
            </div>
            <h3 className="mb-3 text-xl font-semibold text-white">{s.title}</h3>
            <p className="text-sm leading-relaxed text-surface-400">{s.body}</p>
          </div>
        ))}
      </div>
    </section>
  );
}

function Differentiators() {
  const features = [
    {
      tone: "brand" as const,
      title: "Encrypted Execution",
      body: "Order matching and rebalances run in Arcium's confidential MPC. No front-running, no copy-trading.",
      icon: (
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.7}
          d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
        />
      ),
    },
    {
      tone: "accent" as const,
      title: "Aggregated Yield",
      body: "Smart routing across Raydium, Drift, and Orca. The engine finds the most efficient compounding paths.",
      icon: (
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.7}
          d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
        />
      ),
    },
    {
      tone: "purple" as const,
      title: "Strategy cNFTs",
      body: "Every position is backed by a compressed NFT. Track historical yields and strategy metadata on-chain.",
      icon: (
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.7}
          d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
        />
      ),
    },
  ];

  return (
    <section className="mx-auto max-w-6xl">
      <div className="mb-12 text-center">
        <p className="eyebrow justify-center">Why YieldVeil</p>
        <h2 className="mt-4 text-4xl font-bold tracking-tight text-white md:text-5xl">
          Built for serious capital.
        </h2>
      </div>

      <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
        {features.map((f, i) => (
          <FeatureCard key={f.title} {...f} delay={i * 100} />
        ))}
      </div>
    </section>
  );
}

function FeatureCard({
  tone,
  title,
  body,
  icon,
  delay,
}: {
  tone: "brand" | "accent" | "purple";
  title: string;
  body: string;
  icon: React.ReactNode;
  delay: number;
}) {
  const toneStyle = {
    brand: "bg-brand-500/15 text-brand-300 ring-brand-400/25",
    accent: "bg-accent/15 text-accent ring-accent/25",
    purple: "bg-purple-500/15 text-purple-300 ring-purple-400/25",
  }[tone];

  return (
    <div
      className="card-interactive p-7 animate-slide-up"
      style={{ animationDelay: `${delay}ms` }}
    >
      <div className={`mb-5 flex h-12 w-12 items-center justify-center rounded-xl ring-1 ${toneStyle}`}>
        <svg className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          {icon}
        </svg>
      </div>
      <h3 className="mb-2 text-lg font-semibold text-white">{title}</h3>
      <p className="text-sm leading-relaxed text-surface-400">{body}</p>
    </div>
  );
}

function FinalCTA() {
  return (
    <section className="mx-auto max-w-5xl pb-20">
      <div className="ring-gradient relative overflow-hidden rounded-3xl p-12 text-center md:p-16">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(59,82,252,0.15),transparent_70%)]" />
        <div className="relative z-10">
          <h2 className="text-3xl font-bold tracking-tight text-white md:text-4xl">
            Stop leaking alpha to bots.
          </h2>
          <p className="mx-auto mt-3 max-w-xl text-surface-300">
            Try it on devnet. Connect your wallet, deposit test USDC, watch a
            shielded rebalance happen.
          </p>
          <div className="mt-8 flex flex-col justify-center gap-3 sm:flex-row">
            <Link href="/vaults" className="btn-primary px-8 py-4 text-base">
              Browse Vaults
              <ArrowRight />
            </Link>
            <a
              href="https://docs.arcium.com"
              target="_blank"
              rel="noopener noreferrer"
              className="btn-ghost"
            >
              Read about Arcium
              <ArrowExternal />
            </a>
          </div>
        </div>
      </div>
    </section>
  );
}

function ArrowRight() {
  return (
    <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14m-7-7l7 7-7 7" />
    </svg>
  );
}

function ArrowExternal() {
  return (
    <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M14 3h7m0 0v7m0-7L10 14M5 5h5v2H7v10h10v-3h2v5H5V5z" />
    </svg>
  );
}
