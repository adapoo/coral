import { NextResponse } from "next/server";
import { fetchPing } from "@/lib/api/ping";
import { kv } from "@vercel/kv";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const uuid = (searchParams.get("uuid") || "").trim();
  if (!uuid)
    return NextResponse.json({ error: "Missing uuid" }, { status: 400 });
  const apiKey = process.env.PING_API_KEY;
  try {
    // route-level distributed limiter
    const nowWin = Math.floor(Date.now() / 60_000) * 60_000;
    const key = `rl:/api/ping:${nowWin}`;
    try {
      const n = (await kv.incr(key)) ?? 0;
      if (n === 1) await kv.expire(key, 60);
      if (n > 120)
        return NextResponse.json({ success: false }, { status: 429 });
    } catch {}

    const resp = await fetchPing(uuid, apiKey);
    const res = NextResponse.json(resp ?? { success: false, data: [] });
    res.headers.set(
      "Cache-Control",
      "public, s-maxage=300, stale-while-revalidate=300"
    );
    return res;
  } catch {
    const res = NextResponse.json(
      { success: false, data: [] },
      { status: 200 }
    );
    res.headers.set(
      "Cache-Control",
      "public, s-maxage=300, stale-while-revalidate=300"
    );
    return res;
  }
}
