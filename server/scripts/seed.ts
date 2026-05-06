import { PrismaClient } from "@prisma/client";
import { PrismaPg } from "@prisma/adapter-pg";
import "dotenv/config";

const url = process.env.DATABASE_URL;
if (!url) throw new Error("DATABASE_URL is required");
const prisma = new PrismaClient({ adapter: new PrismaPg({ connectionString: url }) });

const PROTOCOL_RAYDIUM_ID = "11111111-1111-4111-8111-aaaaaaaaaaaa";
const PROTOCOL_ORCA_ID = "22222222-2222-4222-8222-bbbbbbbbbbbb";
const PROTOCOL_DRIFT_ID = "33333333-3333-4333-8333-cccccccccccc";

const STRATEGY_RAY_LP_ID = "11111111-1111-4111-8111-111111111111";
const STRATEGY_ORCA_CLMM_ID = "22222222-2222-4222-8222-222222222222";
const STRATEGY_DRIFT_FUNDING_ID = "33333333-3333-4333-8333-333333333333";

async function main() {
  console.log("seeding tokens…");
  const sol = await prisma.token.upsert({
    where: { mintAddress: "So11111111111111111111111111111111111111112" },
    create: {
      mintAddress: "So11111111111111111111111111111111111111112",
      symbol: "SOL",
      name: "Solana",
      decimals: 9,
      logoUrl:
        "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png",
    },
    update: {},
  });
  const usdc = await prisma.token.upsert({
    where: { mintAddress: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" },
    create: {
      mintAddress: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      symbol: "USDC",
      name: "USD Coin",
      decimals: 6,
      logoUrl:
        "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v/logo.png",
    },
    update: {},
  });

  console.log("seeding protocols…");
  await prisma.protocol.upsert({
    where: { id: PROTOCOL_RAYDIUM_ID },
    create: {
      id: PROTOCOL_RAYDIUM_ID,
      name: "Raydium",
      contractAddress: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
      websiteUrl: "https://raydium.io",
    },
    update: {},
  });
  await prisma.protocol.upsert({
    where: { id: PROTOCOL_ORCA_ID },
    create: {
      id: PROTOCOL_ORCA_ID,
      name: "Orca",
      contractAddress: "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
      websiteUrl: "https://orca.so",
    },
    update: {},
  });
  await prisma.protocol.upsert({
    where: { id: PROTOCOL_DRIFT_ID },
    create: {
      id: PROTOCOL_DRIFT_ID,
      name: "Drift",
      contractAddress: "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
      websiteUrl: "https://drift.trade",
    },
    update: {},
  });

  console.log("seeding pools…");
  const raydiumSolUsdcPool = await prisma.pool.upsert({
    where: { poolAddress: "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2" },
    create: {
      poolAddress: "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2",
      protocolId: PROTOCOL_RAYDIUM_ID,
      poolType: "amm",
      tokenAId: sol.id,
      tokenBId: usdc.id,
      feeTier: "0.0025",
      currentApy: "22.4",
      tvl: "8400000",
      riskScore: 25,
    },
    update: {},
  });
  const orcaSolUsdcPool = await prisma.pool.upsert({
    where: { poolAddress: "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtL45VYXu5BEvgC" },
    create: {
      poolAddress: "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtL45VYXu5BEvgC",
      protocolId: PROTOCOL_ORCA_ID,
      poolType: "clmm",
      tokenAId: sol.id,
      tokenBId: usdc.id,
      feeTier: "0.0030",
      currentApy: "26.8",
      tvl: "5200000",
      riskScore: 30,
    },
    update: {},
  });
  const driftSolUsdcPool = await prisma.pool.upsert({
    where: { poolAddress: "DriftSoLUSDCFundingPool11111111111111111111" },
    create: {
      poolAddress: "DriftSoLUSDCFundingPool11111111111111111111",
      protocolId: PROTOCOL_DRIFT_ID,
      poolType: "orderbook",
      tokenAId: sol.id,
      tokenBId: usdc.id,
      feeTier: "0.0010",
      currentApy: "31.2",
      tvl: "12000000",
      riskScore: 55,
    },
    update: {},
  });

  console.log("seeding vaults…");
  const solUsdcVault = await prisma.vault.upsert({
    where: { vaultAddress: "Vau1tSoLUSDCYield11111111111111111111111111" },
    create: {
      vaultAddress: "Vau1tSoLUSDCYield11111111111111111111111111",
      name: "SOL-USDC Yield",
      tokenId: usdc.id,
      tvl: "1200000",
      currentApy: "24.5",
      totalShares: "1180000",
      status: "active",
    },
    update: {
      tvl: "1200000",
      currentApy: "24.5",
    },
  });
  const driftDeltaVault = await prisma.vault.upsert({
    where: { vaultAddress: "Vau1tDriftDelta1Neutral11111111111111111111" },
    create: {
      vaultAddress: "Vau1tDriftDelta1Neutral11111111111111111111",
      name: "Delta-Neutral Drift",
      tokenId: usdc.id,
      tvl: "840000",
      currentApy: "31.2",
      totalShares: "830000",
      status: "active",
    },
    update: {
      tvl: "840000",
      currentApy: "31.2",
    },
  });

  console.log("seeding strategies…");
  await prisma.strategy.upsert({
    where: { id: STRATEGY_RAY_LP_ID },
    create: {
      id: STRATEGY_RAY_LP_ID,
      vaultId: solUsdcVault.id,
      name: "Raydium SOL-USDC LP",
      strategyType: "raydium_lp",
      riskLevel: "low",
      allocationWeight: "50",
      isEncrypted: false,
      programId: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
      poolId: raydiumSolUsdcPool.id,
    },
    update: {},
  });
  await prisma.strategy.upsert({
    where: { id: STRATEGY_ORCA_CLMM_ID },
    create: {
      id: STRATEGY_ORCA_CLMM_ID,
      vaultId: solUsdcVault.id,
      name: "Orca SOL-USDC CLMM",
      strategyType: "orca_clmm",
      riskLevel: "low",
      allocationWeight: "50",
      isEncrypted: false,
      programId: "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
      poolId: orcaSolUsdcPool.id,
    },
    update: {},
  });
  await prisma.strategy.upsert({
    where: { id: STRATEGY_DRIFT_FUNDING_ID },
    create: {
      id: STRATEGY_DRIFT_FUNDING_ID,
      vaultId: driftDeltaVault.id,
      name: "Drift Funding Arb",
      strategyType: "drift_perps",
      riskLevel: "medium",
      allocationWeight: "100",
      isEncrypted: true,
      programId: "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
      poolId: driftSolUsdcPool.id,
    },
    update: {},
  });

  console.log("done.");
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(() => prisma.$disconnect());
