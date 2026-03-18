import { NextResponse } from "next/server";
import { resolveMinecraftPlayer } from "@/lib/api/playerdb";
import { kv } from "@vercel/kv";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const ident = (searchParams.get("identifier") || "").trim();
  if (!ident)
    return NextResponse.json({ error: "Missing identifier" }, { status: 400 });
  try {
    // route-level distributed limiter
    const nowWin = Math.floor(Date.now() / 60_000) * 60_000;
    const key = `rl:/api/playerdb:${nowWin}`;
    try {
      const n = (await kv.incr(key)) ?? 0;
      if (n === 1) await kv.expire(key, 60);
      if (n > 500)
        return NextResponse.json(
          { success: false, player: null },
          { status: 429 }
        );
    } catch {}

    const player = await resolveMinecraftPlayer(ident);
    const res = NextResponse.json({ success: !!player, player });
    res.headers.set(
      "Cache-Control",
      "public, s-maxage=300, stale-while-revalidate=300"
    );
    return res;
  } catch {
    return NextResponse.json({ success: false, player: null }, { status: 200 });
  }
}
