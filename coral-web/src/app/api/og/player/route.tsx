/* eslint-disable */
import { ImageResponse } from "next/og";
import React from "react";

export const runtime = "edge";

export async function GET(request: Request) {
  try {
    const { origin, searchParams } = new URL(request.url);
    const name = (searchParams.get("name") || "Player").slice(0, 32);
    const rank = (searchParams.get("rank") || "").slice(0, 64);

    const minecraftFont = await fetch(
      new URL("/fonts/minecraft.ttf", origin)
    ).then((res) => res.arrayBuffer());

    return new ImageResponse(
      (
        <div
          style={{
            width: 1200,
            height: 630,
            position: "relative",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            color: "#fff",
            fontFamily: "Minecraft",
            backgroundImage: `url(${origin}/nighttime.png)`,
            backgroundSize: "cover",
            backgroundPosition: "center",
          }}
        >
          <div
            style={{
              position: "absolute",
              inset: 0,
              background: "rgba(0,0,0,0.35)",
            }}
          />
          <div
            style={{
              position: "absolute",
              top: 40,
              left: 40,
              display: "flex",
              alignItems: "center",
              gap: 16,
            }}
          >
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src={`${origin}/logo.png`}
              width={64}
              height={64}
              style={{ borderRadius: 12 }}
              alt="Coral"
            />
            <div style={{ fontSize: 34, opacity: 0.9 }}>Coral by Urchin</div>
          </div>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
            }}
          >
            <div style={{ fontSize: 92, fontWeight: 700, lineHeight: 1 }}>
              {name}
            </div>
            {rank ? (
              <div style={{ fontSize: 34, opacity: 0.9 }}>{rank}</div>
            ) : null}
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
        fonts: [
          {
            name: "Minecraft",
            data: minecraftFont,
            style: "normal",
            weight: 400,
          },
        ],
      }
    );
  } catch {
    return new Response("OG error", { status: 500 });
  }
}
