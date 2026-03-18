import { USER_AGENT } from "@/lib/config";

export type PingRecord = {
  day: string;
  timestamp: number;
  max: number;
  min: number;
  avg: number;
};

export type PingResponse = {
  success: boolean;
  data: PingRecord[];
};

export async function fetchPing(
  uuid: string,
  apiKey?: string
): Promise<PingResponse | null> {
  try {
    if (!uuid || !apiKey) return null;
    const undashed = String(uuid).replace(/-/g, "").toLowerCase();
    const u = new URL("https://privatemethod.xyz/api/lobby/ping");
    u.searchParams.set("key", apiKey);
    u.searchParams.set("uuid", undashed);
    const startedAt = Date.now();
    const res = await fetch(u.toString(), {
      headers: { "User-Agent": USER_AGENT },
    });
    const durationMs = Date.now() - startedAt;
    const json = (await res.json()) as PingResponse | null;
    try {
      const redactedUrl = u.toString().replace(/key=[^&]+/i, "key=REDACTED");
      console.log("[ping]", {
        url: redactedUrl,
        status: res.status,
        ok: res.ok,
        durationMs,
        success: json?.success ?? false,
        count: Array.isArray(json?.data) ? json?.data.length : 0,
      });
    } catch {}
    if (!res.ok || !json || !json.success) return null;
    return json;
  } catch {
    return null;
  }
}
