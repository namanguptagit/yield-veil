import type { Request, Response, NextFunction } from "express";
import { verifyJwt, type AuthClaims } from "../auth/jwt";

declare global {
  namespace Express {
    interface Request {
      user?: AuthClaims;
    }
  }
}

export function requireAuth(req: Request, res: Response, next: NextFunction) {
  const header = req.headers.authorization;
  if (!header?.startsWith("Bearer ")) {
    return res.status(401).json({ error: "missing_token" });
  }
  try {
    req.user = verifyJwt(header.slice("Bearer ".length));
    next();
  } catch {
    return res.status(401).json({ error: "invalid_token" });
  }
}
