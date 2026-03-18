import { NextResponse } from "next/server";
import { internalGetJson } from "@/lib/utils/server/internal";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const raw = (searchParams.get("query") || "").trim();
  if (!raw) {
    return NextResponse.redirect(new URL(`/?e=inv`, request.url));
  }

  const resolvedResp = await internalGetJson<{
    success: boolean;
    player: { id: string; username: string } | null;
  }>(`/api/playerdb?identifier=${encodeURIComponent(raw)}`);
  const resolved = resolvedResp?.player ?? null;
  if (!resolved) {
    // invalid username/uuid
    return NextResponse.redirect(new URL(`/?e=inv`, request.url));
  }
  // uuid slug
  const slug = resolved.id;
  return NextResponse.redirect(
    new URL(`/player/${encodeURIComponent(slug)}`, request.url)
  );
}
