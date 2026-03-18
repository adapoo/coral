import { NextResponse } from "next/server";

const ALLOWED_PATHS = new Set([
  "/api/ping",
  "/api/urchin",
  "/api/hypixel",
  "/api/playerdb",
  "/api/top",
]);

export async function middleware(request: Request) {
  const url = new URL(request.url);
  if (!ALLOWED_PATHS.has(url.pathname)) return NextResponse.next();

  const secret = process.env.INTERNAL_API_SECRET;
  if (!secret) return new NextResponse("Server misconfigured", { status: 500 });

  const now = Date.now();

  // allow-list: ping and urchin may be fetched client-side from same-origin without HMAC.
  const isBrowserAllowedPath =
    url.pathname === "/api/ping" ||
    url.pathname === "/api/urchin" ||
    url.pathname === "/api/top";

  // verify signature if provided or required
  const ts = request.headers.get("x-app-ts") || "";
  const sig = request.headers.get("x-app-sig") || "";
  let authorized = false;
  if (ts && sig) {
    const tsNum = Number(ts);
    if (Number.isFinite(tsNum) && Math.abs(now - tsNum) <= 60_000) {
      const payload = `${request.method}:${url.pathname}:${ts}`;
      const enc = new TextEncoder();
      const data = enc.encode(secret + payload);
      const digest = await crypto.subtle.digest("SHA-256", data);
      const hex = Array.from(new Uint8Array(digest))
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");
      authorized = hex === sig;
    }
  }

  if (!authorized) {
    if (isBrowserAllowedPath) {
      const origin = request.headers.get("origin");
      const referer = request.headers.get("referer");
      const sameOrigin = (() => {
        try {
          if (origin) return new URL(origin).host === url.host;
          if (referer) return new URL(referer).host === url.host;
        } catch {}
        return false;
      })();
      if (!sameOrigin) {
        return NextResponse.redirect(new URL("/", request.url));
      }
    } else {
      return NextResponse.redirect(new URL("/", request.url));
    }
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/api/:path*"],
};
