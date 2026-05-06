import { Router } from "express";
import { prisma } from "../db";
import { asyncHandler } from "../middleware/asyncHandler";

export const strategiesRouter = Router();

strategiesRouter.get(
  "/:vaultId",
  asyncHandler(async (req, res) => {
    const strategies = await prisma.strategy.findMany({
      where: { vaultId: req.params.vaultId, isActive: true },
      include: {
        pool: {
          include: {
            protocol: { select: { name: true } },
            tokenA: { select: { symbol: true } },
            tokenB: { select: { symbol: true } },
          },
        },
      },
      orderBy: { allocationWeight: "desc" },
    });
    res.json({ strategies });
  }),
);
