import nacl from "tweetnacl";
import bs58 from "bs58";

export function verifySolanaSignature(
  message: string,
  signatureBase58: string,
  walletAddress: string,
): boolean {
  try {
    const messageBytes = new TextEncoder().encode(message);
    const signatureBytes = bs58.decode(signatureBase58);
    const pubkeyBytes = bs58.decode(walletAddress);

    if (signatureBytes.length !== 64) return false;
    if (pubkeyBytes.length !== 32) return false;

    return nacl.sign.detached.verify(
      messageBytes,
      signatureBytes,
      pubkeyBytes,
    );
  } catch {
    return false;
  }
}
