import { USER_AGENT } from "@/lib/config";

export type UrchinTag = {
  type: string;
  reason?: string | null;
  added_by_id?: string | null;
  added_by_username?: string | null;
  added_on?: string | null;
};

export type UrchinResponse = {
  uuid: string;
  tags: UrchinTag[];
};

export async function fetchUrchin(
  uuid: string,
  apiKey?: string
): Promise<UrchinResponse | null> {
  try {
    if (!uuid || !apiKey) return null;
    const u = new URL(`https://urchin.ws/player/${uuid}`);
    u.searchParams.set("key", apiKey);
    const startedAt = Date.now();
    const res = await fetch(u.toString(), {
      headers: { "User-Agent": USER_AGENT },
    });
    const durationMs = Date.now() - startedAt;
    const json = (await res.json()) as UrchinResponse | null;
    try {
      const redactedUrl = u.toString().replace(/key=[^&]+/i, "key=REDACTED");
      console.log("[urchin]", {
        url: redactedUrl,
        status: res.status,
        ok: res.ok,
        durationMs,
        tags: Array.isArray(json?.tags) ? json?.tags.length : 0,
      });
    } catch {}
    if (!res.ok || !json || !json.uuid) return null;
    return json;
  } catch {
    return null;
  }
}
