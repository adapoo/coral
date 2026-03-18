export type PlayerDbResponse = {
  success: boolean;
  data?: {
    player?: {
      id?: string;
      username?: string;
      meta?: Record<string, unknown>;
      raw?: unknown;
      avatar?: string;
    };
  };
};

export type ResolvedPlayer = {
  username: string;
  id: string;
  avatar: string;
  meta: Record<string, unknown>;
};

import { USER_AGENT } from "@/lib/config";

export async function resolveMinecraftPlayer(
  identifier: string
): Promise<ResolvedPlayer | null> {
  const url = new URL(`https://playerdb.co/api/player/minecraft/${identifier}`);
  const startedAt = Date.now();
  const res = await fetch(url.toString(), {
    cache: "no-store",
    headers: {
      "User-Agent": USER_AGENT,
    },
  });
  const durationMs = Date.now() - startedAt;
  const json = (await res.json()) as PlayerDbResponse;
  try {
    console.log("[playerdb]", {
      url: url.toString(),
      status: res.status,
      ok: res.ok,
      durationMs,
      success: json?.success ?? false,
      code: (json as any)?.code ?? undefined,
    });
  } catch {}
  if (
    !json?.success ||
    !json?.data?.player?.username ||
    !json?.data?.player?.id
  ) {
    return null;
  }
  const { username, id } = json.data.player;
  const avatar = json.data.player.avatar || `https://mc-heads.net/avatar/${id}`;
  return {
    username: username!,
    id: id!,
    avatar,
    meta: json.data.player.meta ?? {},
  };
}
