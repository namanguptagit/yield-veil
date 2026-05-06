import type { Metadata } from "next";
import "./globals.css";
import { Navbar } from "@/components/layout/Navbar";
import "@solana/wallet-adapter-react-ui/styles.css";
import { SolanaProvider } from "@/components/counter/provider/Solana";


export const metadata: Metadata = {
  title: "YieldVeil — Privacy-First DeFi Yield Aggregator",
  description:
    "Maximize your Solana yields with encrypted strategy execution. Your alpha stays hidden with Arcum MXE confidential computing.",
  keywords: [
    "DeFi",
    "Solana",
    "yield aggregator",
    "privacy",
    "Arcum",
    "MEV protection",
  ],
  openGraph: {
    title: "YieldVeil — Privacy-First DeFi Yield Aggregator",
    description:
      "Maximize your Solana yields with encrypted strategy execution.",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className="antialiased min-h-screen bg-surface-950 text-foreground flex flex-col">
        <SolanaProvider>
          
        <Navbar />
        <main className="flex-grow pt-24 pb-12 px-6 lg:px-8 max-w-7xl mx-auto w-full">
          {children}
        </main>
        </SolanaProvider>
      </body>
    </html>
  );
}
