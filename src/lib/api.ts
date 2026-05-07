const SERVER_URL =
  process.env.NEXT_PUBLIC_SERVER_URL || "http://localhost:4000";

export function getAuthToken(): string | null {
  if (typeof window === "undefined") return null;
  return localStorage.getItem("auth_token");
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public body: unknown,
  ) {
    super(message);
  }
}

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const token = getAuthToken();
  const headers: Record<string, string> = {
    "content-type": "application/json",
    ...((init.headers as Record<string, string>) ?? {}),
  };
  if (token) headers.authorization = `Bearer ${token}`;

  const res = await fetch(`${SERVER_URL}${path}`, { ...init, headers });
  const body = await res.json().catch(() => null);
  if (!res.ok) {
    const msg =
      (body && typeof body === "object" && "error" in body
        ? String((body as { error: unknown }).error)
        : null) ?? res.statusText;
    throw new ApiError(msg, res.status, body);
  }
  return body as T;
}

export const api = {
  get: <T>(path: string) => request<T>(path),
  post: <T>(path: string, body: unknown) =>
    request<T>(path, { method: "POST", body: JSON.stringify(body) }),
};

// ---------- Response types (matching server routes) ----------

export type VaultToken = {
  symbol: string;
  name: string;
  decimals: number;
  logoUrl: string | null;
};

export type VaultSummary = {
  id: string;
  vaultAddress: string;
  name: string;
  tokenId: string;
  tvl: string;
  currentApy: string;
  totalShares: string;
  status: "active" | "paused" | "deprecated";
  token: VaultToken;
  _count: { strategies: number; positions: number };
};

export type DashboardSummary = {
  totalDeposited: string | number;
  totalCurrentValue: string | number;
  totalUnrealizedPnl: string | number;
  activePositions: number;
};

export type PositionItem = {
  id: string;
  vaultId: string;
  shares: string;
  depositValue: string;
  currentValue: string;
  unrealizedPnl: string;
  status: "active" | "closed";
  openedAt: string;
  vault: {
    id: string;
    name: string;
    currentApy: string;
    token: { symbol: string; decimals: number; logoUrl: string | null };
  };
};

// ---------- Display formatters ----------

export function formatTVL(raw: string | number): string {
  const n = typeof raw === "string" ? Number(raw) : raw;
  if (!Number.isFinite(n)) return "$0";
  if (n >= 1_000_000) return `$${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `$${(n / 1_000).toFixed(0)}k`;
  return `$${n.toFixed(0)}`;
}

export function formatAPY(raw: string | number): string {
  const n = typeof raw === "string" ? Number(raw) : raw;
  return `${(Number.isFinite(n) ? n : 0).toFixed(1)}%`;
}

export function formatUSD(raw: string | number): string {
  const n = typeof raw === "string" ? Number(raw) : raw;
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(Number.isFinite(n) ? n : 0);
}

export function formatSignedUSD(raw: string | number): string {
  const n = typeof raw === "string" ? Number(raw) : raw;
  const sign = n >= 0 ? "+" : "";
  return `${sign}${formatUSD(n)}`;
}
