import { Router } from "express";
import { prisma } from "../db";
import { asyncHandler } from "../middleware/asyncHandler";

export const vaultsRouter = Router();

vaultsRouter.get(
  "/",
  asyncHandler(async (_req, res) => {
    const vaults = await prisma.vault.findMany({
      where: { status: "active" },
      include: {
        token: {
          select: { symbol: true, name: true, decimals: true, logoUrl: true },
        },
        _count: { select: { strategies: true, positions: true } },
      },
      orderBy: { tvl: "desc" },
    });
    res.json({ vaults });
  }),
);

vaultsRouter.get(
  "/:id",
  asyncHandler(async (req, res) => {
    const vault = await prisma.vault.findUnique({
      where: { id: req.params.id },
      include: {
        token: true,
        strategies: {
          where: { isActive: true },
          include: {
            pool: {
              include: {
                protocol: { select: { name: true, websiteUrl: true } },
                tokenA: { select: { symbol: true } },
                tokenB: { select: { symbol: true } },
              },
            },
          },
        },
      },
    });
    if (!vault) return res.status(404).json({ error: "vault_not_found" });
    res.json({ vault });
  }),
);
