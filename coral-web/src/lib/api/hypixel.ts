export type HypixelPlayerResponse = {
  success: boolean;
  player?: any;
  cause?: string;
};

import { USER_AGENT } from "@/lib/config";

const CORALDB_SERVER = "http://207.211.176.142";
const API_SECRET = process.env.CORALDB_API_SECRET;
const headers = { 'Authorization': `Bearer ${API_SECRET}` };

export async function fetchHypixelPlayer(
  uuid: string,
  maxAge = 30
): Promise<HypixelPlayerResponse> {
  const startedAt = Date.now();
  
  try {
    const cacheUrl = `${CORALDB_SERVER}/api/v1/player/${uuid}/cache?maxAge=${maxAge}`;
    const cacheRes = await fetch(cacheUrl, { headers });
    
    if (cacheRes.ok) {
      const cacheData = await cacheRes.json();
      if (cacheData.cached && cacheData.data) {
        console.log("[hypixel]", {
          url: cacheUrl,
          status: cacheRes.status,
          ok: cacheRes.ok,
          durationMs: Date.now() - startedAt,
          success: true,
          source: 'cache',
          age: cacheData.ageSeconds,
        });
        return cacheData.data;
      }
    }
    
    if (cacheRes.status !== 404) {
      console.warn(`Cache API error: ${cacheRes.status}`);
    }
  } catch (error) {
    console.warn('Cache lookup failed, falling back to Hypixel API');
  }
  
  const apiKey = process.env.HYPIXEL_API_KEY;
  if (!apiKey) {
    return { success: false, cause: "Missing HYPIXEL_API_KEY" };
  }
  
  const u = new URL("https://api.hypixel.net/v2/player");
  u.searchParams.set("uuid", uuid);
  
  const res = await fetch(u.toString(), {
    cache: "no-store",
    headers: {
      "API-Key": apiKey,
      "User-Agent": USER_AGENT,
    },
    signal: AbortSignal.timeout(10000)
  });
  
  const json = (await res.json()) as HypixelPlayerResponse;
  
  try {
    console.log("[hypixel]", {
      url: u.toString(),
      status: res.status,
      ok: res.ok,
      durationMs: Date.now() - startedAt,
      success: json?.success,
      cause: json?.cause,
      source: "api",
    });
  } catch {}
  
  if (!json.success || !json.player) {
    return json;
  }
  
  try {
    const storeUrl = `${CORALDB_SERVER}/api/v1/player/${uuid}/store`;
    await fetch(storeUrl, {
      method: "POST",
      headers: {
        ...headers,
        "Content-Type": "application/json"
      },
      body: JSON.stringify(json)
    });
  } catch (error) {
    console.warn("Failed to store player data in cache:", error);
  }
  
  return json;
}