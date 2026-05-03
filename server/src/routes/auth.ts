import { Router } from "express";
import { z } from "zod";
import { prisma } from "../db";
import {
  issueNonce,
  consumeNonce,
  buildSignMessage,
} from "../auth/nonces";
import { verifySolanaSignature } from "../auth/signature";
import { signJwt } from "../auth/jwt";

export const authRouter = Router();

const walletSchema = z.string().min(32).max(44);

const nonceBody = z.object({ walletAddress: walletSchema });

authRouter.post("/nonce", (req, res) => {
  const parsed = nonceBody.safeParse(req.body);
  if (!parsed.success) {
    return res.status(400).json({ error: "bad_request" });
  }
  const { nonce, issuedAt } = issueNonce(parsed.data.walletAddress);
  res.json({
    nonce,
    issuedAt: issuedAt.toISOString(),
    message: buildSignMessage(nonce, issuedAt),
  });
});

const verifyBody = z.object({
  walletAddress: walletSchema,
  signature: z.string().min(64).max(128),
  nonce: z.string().length(32),
});

authRouter.post("/verify", async (req, res) => {
  const parsed = verifyBody.safeParse(req.body);
  if (!parsed.success) {
    return res.status(400).json({ error: "bad_request" });
  }
  const { walletAddress, signature, nonce } = parsed.data;

  const entry = consumeNonce(walletAddress, nonce);
  if (!entry) {
    return res.status(401).json({ error: "invalid_or_expired_nonce" });
  }

  const message = buildSignMessage(entry.nonce, entry.issuedAt);
  const ok = verifySolanaSignature(message, signature, walletAddress);
  if (!ok) {
    return res.status(401).json({ error: "invalid_signature" });
  }

  const user = await prisma.user.upsert({
    where: { walletAddress },
    create: { walletAddress },
    update: {},
    select: { id: true, walletAddress: true, displayName: true },
  });

  const token = signJwt({ sub: user.id, walletAddress: user.walletAddress });
  res.json({ token, user });
});

authRouter.get("/me", async (req, res) => {
  const header = req.headers.authorization;
  if (!header?.startsWith("Bearer ")) {
    return res.status(401).json({ error: "missing_token" });
  }
  try {
    const { verifyJwt } = await import("../auth/jwt");
    const claims = verifyJwt(header.slice("Bearer ".length));
    const user = await prisma.user.findUnique({
      where: { id: claims.sub },
      select: { id: true, walletAddress: true, displayName: true },
    });
    if (!user) return res.status(404).json({ error: "user_not_found" });
    res.json({ user });
  } catch {
    return res.status(401).json({ error: "invalid_token" });
  }
});
