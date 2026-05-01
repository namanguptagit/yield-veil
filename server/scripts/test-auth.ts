import nacl from "tweetnacl";
import bs58 from "bs58";

const API = "http://localhost:4000";

async function main() {
  const kp = nacl.sign.keyPair();
  const walletAddress = bs58.encode(kp.publicKey);
  console.log("wallet:", walletAddress);

  const nonceRes = await fetch(`${API}/auth/nonce`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ walletAddress }),
  });
  const nonceJson = await nonceRes.json();
  console.log("nonce response:", nonceJson);

  const messageBytes = new TextEncoder().encode(nonceJson.message);
  const signature = nacl.sign.detached(messageBytes, kp.secretKey);
  const signatureBase58 = bs58.encode(signature);

  const verifyRes = await fetch(`${API}/auth/verify`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      walletAddress,
      signature: signatureBase58,
      nonce: nonceJson.nonce,
    }),
  });
  const verifyJson = await verifyRes.json();
  console.log("verify response:", verifyJson);

  if (!verifyJson.token) {
    console.error("FAIL: no token");
    process.exit(1);
  }

  const meRes = await fetch(`${API}/auth/me`, {
    headers: { authorization: `Bearer ${verifyJson.token}` },
  });
  console.log("me response:", await meRes.json());

  console.log("\nNegative test: replay same nonce");
  const replayRes = await fetch(`${API}/auth/verify`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      walletAddress,
      signature: signatureBase58,
      nonce: nonceJson.nonce,
    }),
  });
  console.log("replay response:", replayRes.status, await replayRes.json());

  console.log("\nNegative test: bad signature");
  const nonceRes2 = await fetch(`${API}/auth/nonce`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ walletAddress }),
  });
  const nonce2 = await nonceRes2.json();
  const badSig = bs58.encode(new Uint8Array(64));
  const badRes = await fetch(`${API}/auth/verify`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      walletAddress,
      signature: badSig,
      nonce: nonce2.nonce,
    }),
  });
  console.log("bad sig response:", badRes.status, await badRes.json());
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
