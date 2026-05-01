import { randomBytes } from "crypto";

const NONCE_TTL_MS = 5 * 60 * 1000;

type NonceEntry = { nonce: string; issuedAt: Date; expiresAt: number };

const store = new Map<string, NonceEntry>();

export function issueNonce(walletAddress: string): NonceEntry {
  const nonce = randomBytes(16).toString("hex");
  const issuedAt = new Date();
  const entry: NonceEntry = {
    nonce,
    issuedAt,
    expiresAt: Date.now() + NONCE_TTL_MS,
  };
  store.set(walletAddress, entry);
  return entry;
}

export function consumeNonce(
  walletAddress: string,
  nonce: string,
): NonceEntry | null {
  const entry = store.get(walletAddress);
  if (!entry) return null;
  if (entry.nonce !== nonce) return null;
  if (Date.now() > entry.expiresAt) {
    store.delete(walletAddress);
    return null;
  }
  store.delete(walletAddress);
  return entry;
}

export function buildSignMessage(nonce: string, issuedAt: Date): string {
  return `YieldVeil sign-in\n\nNonce: ${nonce}\nIssued at: ${issuedAt.toISOString()}`;
}
