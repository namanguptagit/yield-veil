import { Router } from "express";
import { prisma } from "../db";
import { requireAuth } from "../middleware/requireAuth";
import { asyncHandler } from "../middleware/asyncHandler";

export const positionsRouter = Router();

positionsRouter.use(requireAuth);

positionsRouter.get(
  "/",
  asyncHandler(async (req, res) => {
    const positions = await prisma.position.findMany({
      where: { userId: req.user!.sub, status: "active" },
      include: {
        vault: {
          include: {
            token: { select: { symbol: true, decimals: true, logoUrl: true } },
          },
        },
      },
      orderBy: { openedAt: "desc" },
    });
    res.json({ positions });
  }),
);
