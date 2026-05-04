"use client";

import React, { useCallback, useMemo } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";

export function WalletButton() {
  const { publicKey, wallet, disconnect, connecting, connected } = useWallet();
  const { setVisible } = useWalletModal();

  
  const handleClick = useCallback(() => {
    if (!wallet) {
      
      setVisible(true);
    } else if (!connected) {
      
    }
  }, [wallet, connected, setVisible]);

  
  const base58 = useMemo(() => publicKey?.toBase58(), [publicKey]);
  const truncatedAddress = useMemo(() => {
    if (!base58) return null;
    return `${base58.slice(0, 4)}...${base58.slice(-4)}`;
  }, [base58]);

  
  if (connected && base58) {
    return (
      <div className="flex items-center gap-2 animate-fade-in">
        <div className="hidden sm:flex items-center gap-2 rounded-xl border border-white/[0.08] bg-white/[0.03] px-3 py-2">
          <div className="h-2 w-2 rounded-full bg-accent animate-pulse-glow" />
          <span className="text-xs font-mono text-surface-300">
            {truncatedAddress}
          </span>
        </div>
        <button
          onClick={() => disconnect()}
          className="rounded-lg border border-white/[0.08] bg-white/[0.03] px-3 py-2 text-xs font-medium text-surface-400 transition-all hover:bg-red-500/10 hover:text-red-400 hover:border-red-500/30"
        >
          Disconnect
        </button>
      </div>
    );
  }

  
  return (
    <button
      onClick={handleClick}
      disabled={connecting}
      className="btn-wallet flex items-center gap-2"
    >
      {connecting ? (
        <>
          <svg className="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          Connecting...
        </>
      ) : (
        <>
          <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path strokeLinecap="round" strokeLinejoin="round" d="M21 12a2.25 2.25 0 00-2.25-2.25H15a3 3 0 11-6 0H5.25A2.25 2.25 0 003 12m18 0v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6m18 0V9M3 12V9m18 0a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 013 9m18 0V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 013 6v3" />
          </svg>
          Connect Wallet
        </>
      )}
    </button>
  );
}
