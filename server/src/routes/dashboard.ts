import { Router } from "express";
import { prisma } from "../db";
import { requireAuth } from "../middleware/requireAuth";
import { asyncHandler } from "../middleware/asyncHandler";

export const dashboardRouter = Router();

dashboardRouter.use(requireAuth);

dashboardRouter.get(
  "/summary",
  asyncHandler(async (req, res) => {
    const userId = req.user!.sub;

    const [agg, activeCount] = await Promise.all([
      prisma.position.aggregate({
        where: { userId, status: "active" },
        _sum: { depositValue: true, currentValue: true, unrealizedPnl: true },
      }),
      prisma.position.count({ where: { userId, status: "active" } }),
    ]);

    res.json({
      totalDeposited: agg._sum.depositValue ?? 0,
      totalCurrentValue: agg._sum.currentValue ?? 0,
      totalUnrealizedPnl: agg._sum.unrealizedPnl ?? 0,
      activePositions: activeCount,
    });
  }),
);
