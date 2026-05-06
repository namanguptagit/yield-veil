"use client";

import React, { useCallback, useMemo } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";
import bs58 from "bs58";

const SERVER_URL = process.env.NEXT_PUBLIC_SERVER_URL || "http://localhost:4000";

export function WalletButton() {
  const { publicKey, wallet, disconnect, signMessage, connected } = useWallet();
  const { setVisible } = useWalletModal();
  
  const [isSigning, setIsSigning] = useState(false);
  const [token, setToken] = useState<string | null>(null);

  // Load token from localStorage on mount
  useEffect(() => {
    const storedToken = localStorage.getItem("auth_token");
    if (storedToken) setToken(storedToken);
  }, []);

  const handleSignIn = useCallback(async () => {
    if (!publicKey || !signMessage) return;

    try {
      setIsSigning(true);
      const walletAddress = publicKey.toBase58();

      // 1. Get Nonce from Rohit's /auth/nonce
      const nonceRes = await fetch(`${SERVER_URL}/auth/nonce`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ walletAddress }),
      });
      const { nonce, message } = await nonceRes.json();

      // 2. Sign the message using the wallet
      const messageBytes = new TextEncoder().encode(message);
      const signatureBytes = await signMessage(messageBytes);
      const signature = bs58.encode(signatureBytes);

      // 3. Verify on Backend
      const verifyRes = await fetch(`${SERVER_URL}/auth/verify`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ walletAddress, signature, nonce }),
      });

      const { token: jwt } = await verifyRes.json();

      if (jwt) {
        localStorage.setItem("auth_token", jwt);
        setToken(jwt);
      }
    } catch (err) {
      console.error("Authentication failed:", err);
    } finally {
      setIsSigning(false);
    }
  }, [publicKey, signMessage]);
  
  const handleDisconnect = useCallback(async () => {
    await disconnect();
    localStorage.removeItem("auth_token");
    setToken(null);
  }, [disconnect]);

  
  const base58 = useMemo(() => publicKey?.toBase58(), [publicKey]);

  const truncatedAddress = useMemo(() => {
    const base58 = publicKey?.toBase58();
    if (!base58) return null;
    return `${base58.slice(0, 4)}...${base58.slice(-4)}`;
  }, [publicKey]);

  
  if (!connected || !publicKey) {
    return (
      <button 
        onClick={() => setVisible(true)} 
        className="btn-wallet flex items-center gap-2"
      >
        <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M21 12a2.25 2.25 0 00-2.25-2.25H15a3 3 0 11-6 0H5.25A2.25 2.25 0 003 12m18 0v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6m18 0V9M3 12V9m18 0a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 013 9m18 0V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 013 6v3" />
        </svg>
        Connect Wallet
      </button>
    );
  }

  if (connected && !token) {
    return (
      <button
        onClick={handleSignIn}
        disabled={isSigning}
        className="rounded-lg bg-accent px-4 py-2 text-xs font-bold text-black hover:bg-accent/90 disabled:opacity-50"
      >
        {isSigning ? "Signing..." : "Verify Identity (Sign In)"}
      </button>
    );
  }
  

  
  return (
    <div className="flex items-center gap-2">
      <div className="hidden sm:flex items-center gap-2 rounded-xl border border-white/[0.08] bg-white/[0.03] px-3 py-2">
        <div className="h-2 w-2 rounded-full bg-accent animate-pulse" />
        <span className="text-xs font-mono text-surface-300">{truncatedAddress}</span>
      </div>
      <button
        onClick={handleDisconnect}
        className="rounded-lg border border-white/[0.08] bg-white/[0.03] px-3 py-2 text-xs font-medium text-surface-400 hover:text-red-400"
      >
        Disconnect
      </button>
    </div>
  );
}
