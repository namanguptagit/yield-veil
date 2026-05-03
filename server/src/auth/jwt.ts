import jwt, { type SignOptions } from "jsonwebtoken";
import { env } from "../env";

export type AuthClaims = {
  sub: string;
  walletAddress: string;
};

export function signJwt(claims: AuthClaims): string {
  return jwt.sign(claims, env.jwtSecret, {
    expiresIn: env.jwtExpiresIn,
  } as SignOptions);
}

export function verifyJwt(token: string): AuthClaims {
  const decoded = jwt.verify(token, env.jwtSecret);
  if (typeof decoded === "string") throw new Error("invalid_token");
  const { sub, walletAddress } = decoded as jwt.JwtPayload & {
    walletAddress?: string;
  };
  if (!sub || !walletAddress) throw new Error("invalid_token");
  return { sub, walletAddress };
}
