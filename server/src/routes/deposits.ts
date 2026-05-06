import { Router } from "express";
import { z } from "zod";
import { prisma } from "../db";
import { requireAuth } from "../middleware/requireAuth";
import { asyncHandler } from "../middleware/asyncHandler";

export const depositsRouter = Router();

depositsRouter.use(requireAuth);

const depositBody = z.object({
  vaultId: z.string().uuid(),
  amount: z.string().regex(/^\d+(\.\d+)?$/, "must be a positive decimal string"),
  sharesReceived: z
    .string()
    .regex(/^\d+(\.\d+)?$/, "must be a positive decimal string"),
  txSignature: z.string().min(64).max(128),
});

depositsRouter.post(
  "/",
  asyncHandler(async (req, res) => {
    const parsed = depositBody.safeParse(req.body);
    if (!parsed.success) {
      return res
        .status(400)
        .json({ error: "bad_request", details: parsed.error.flatten() });
    }
    const { vaultId, amount, sharesReceived, txSignature } = parsed.data;

    const vault = await prisma.vault.findUnique({
      where: { id: vaultId },
      select: { id: true, tokenId: true, status: true },
    });
    if (!vault) return res.status(404).json({ error: "vault_not_found" });
    if (vault.status !== "active") {
      return res.status(409).json({ error: "vault_not_active" });
    }

    const deposit = await prisma.deposit.create({
      data: {
        userId: req.user!.sub,
        vaultId,
        tokenId: vault.tokenId,
        amount,
        sharesReceived,
        txSignature,
        status: "pending",
      },
    });
    res.status(201).json({ deposit });
  }),
);
