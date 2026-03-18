import { NextResponse } from "next/server";
import { getTopPopularPlayers } from "@/lib/utils/server/popular";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const raw = searchParams.get("limit");
  const q = (searchParams.get("q") || "").toLowerCase().trim();
  const limit = Math.max(1, Math.min(500, Number(raw) || 200));
  const list = await getTopPopularPlayers(limit);
  const filtered = q
    ? list.filter(
        (p) => p.display && p.name && p.name.toLowerCase().startsWith(q)
      )
    : list.filter((p) => p.display);
  return NextResponse.json({ players: filtered.slice(0, limit) });
}
