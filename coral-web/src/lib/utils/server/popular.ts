import { kv } from "@vercel/kv";

export type PopularPlayer = {
  uuid: string;
  name?: string | null;
  display?: string | null;
  score: number;
};

const ZSET_KEY = "pop:players";
const HASH_NAMES = "pop:names";
const HASH_DISPLAY = "pop:displays";

export async function recordPlayerPopularity(
  uuid: string,
  name?: string | null,
  display?: string | null
): Promise<void> {
  if (!uuid) return;
  try {
    await kv.zincrby(ZSET_KEY, 1, uuid);
    if (name) await kv.hset(HASH_NAMES, { [uuid]: name });
    if (display) await kv.hset(HASH_DISPLAY, { [uuid]: display });
  } catch {}
}

export async function getTopPopularPlayers(
  limit = 200
): Promise<PopularPlayer[]> {
  try {
    const startedAt = Date.now();
    const entries = (await kv.zrange(ZSET_KEY, 0, Math.max(0, limit - 1), {
      rev: true,
      withScores: true,
    })) as Array<string | number>;
    const result: PopularPlayer[] = [];
    const uuids: string[] = [];
    for (let i = 0; i < entries.length; i += 2) {
      const uuid = String(entries[i]);
      const score = Number(entries[i + 1] ?? 0);
      uuids.push(uuid);
      result.push({ uuid, name: null, display: null, score });
    }
    if (uuids.length) {
      const rawNames = (await kv.hmget(HASH_NAMES, ...uuids)) as unknown;
      const rawDisps = (await kv.hmget(HASH_DISPLAY, ...uuids)) as unknown;

      const names: Array<string | null> = Array.isArray(rawNames)
        ? (rawNames as Array<string | null>)
        : uuids.map((u) =>
            rawNames && typeof rawNames === "object"
              ? ((rawNames as Record<string, unknown>)[u] as
                  | string
                  | null
                  | undefined) ?? null
              : null
          );
      const disps: Array<string | null> = Array.isArray(rawDisps)
        ? (rawDisps as Array<string | null>)
        : uuids.map((u) =>
            rawDisps && typeof rawDisps === "object"
              ? ((rawDisps as Record<string, unknown>)[u] as
                  | string
                  | null
                  | undefined) ?? null
              : null
          );

      for (let i = 0; i < names.length && i < result.length; i++)
        result[i].name = normalizeNullable(names[i]);
      for (let i = 0; i < disps.length && i < result.length; i++)
        result[i].display = normalizeNullable(disps[i]);
    }
    return result;
  } catch {
    return [];
  }
}

function normalizeNullable(v: string | null | undefined): string | null {
  if (v == null) return null;
  const s = String(v).trim();
  return s.length > 0 ? s : null;
}
