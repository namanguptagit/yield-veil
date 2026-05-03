import { Router } from "express";
import { prisma } from "../db";

export const healthRouter = Router();

healthRouter.get("/", async (_req, res) => {
  try {
    await prisma.$queryRaw`SELECT 1`;
    res.json({ status: "ok", db: "ok" });
  } catch (err) {
    res.status(503).json({
      status: "degraded",
      db: "down",
      error: err instanceof Error ? err.message : String(err),
    });
  }
});
