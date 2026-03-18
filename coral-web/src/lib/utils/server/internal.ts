import crypto from "crypto";

export function signInternalRequest(
  method: string,
  path: string,
  ts: number,
  secret: string
): string {
  const payload = `${method.toUpperCase()}:${path}:${ts}`;
  return crypto
    .createHash("sha256")
    .update(secret + payload)
    .digest("hex");
}

export async function internalGetJson<T = unknown>(
  pathAndQuery: string
): Promise<T | null> {
  const secret = process.env.INTERNAL_API_SECRET;
  const baseUrl = process.env.INTERNAL_BASE_URL || "https://coral.urchin.ws";
  if (!secret) throw new Error("INTERNAL_API_SECRET not set");
  const ts = Date.now();
  const sig = signInternalRequest(
    "GET",
    pathAndQuery.split("?")[0],
    ts,
    secret
  );
  const url = `${baseUrl}${pathAndQuery}`;
  const res = await fetch(url, {
    headers: {
      "x-app-ts": String(ts),
      "x-app-sig": sig,
      Accept: "application/json",
    },
  });
  if (!res.ok) return null;
  return (await res.json()) as T;
}
