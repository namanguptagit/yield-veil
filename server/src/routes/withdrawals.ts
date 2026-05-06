import { Router } from "express";
import { z } from "zod";
import { prisma } from "../db";
import { requireAuth } from "../middleware/requireAuth";
import { asyncHandler } from "../middleware/asyncHandler";

export const withdrawalsRouter = Router();

withdrawalsRouter.use(requireAuth);

const withdrawBody = z.object({
  vaultId: z.string().uuid(),
  amount: z.string().regex(/^\d+(\.\d+)?$/),
  sharesBurned: z.string().regex(/^\d+(\.\d+)?$/),
  txSignature: z.string().min(64).max(128),
});

withdrawalsRouter.post(
  "/",
  asyncHandler(async (req, res) => {
    const parsed = withdrawBody.safeParse(req.body);
    if (!parsed.success) {
      return res
        .status(400)
        .json({ error: "bad_request", details: parsed.error.flatten() });
    }
    const { vaultId, amount, sharesBurned, txSignature } = parsed.data;

    const vault = await prisma.vault.findUnique({
      where: { id: vaultId },
      select: { id: true, tokenId: true, status: true },
    });
    if (!vault) return res.status(404).json({ error: "vault_not_found" });

    const withdrawal = await prisma.withdrawal.create({
      data: {
        userId: req.user!.sub,
        vaultId,
        tokenId: vault.tokenId,
        amount,
        sharesBurned,
        txSignature,
        status: "pending",
      },
    });
    res.status(201).json({ withdrawal });
  }),
);
