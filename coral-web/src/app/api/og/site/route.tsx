import { ImageResponse } from "next/og";
import React from "react";

export const runtime = "edge";

export async function GET(request: Request) {
  try {
    const { origin } = new URL(request.url);

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
          <div style={{ display: "flex", alignItems: "center", gap: 24 }}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src={`${origin}/logo.png`}
              width={128}
              height={128}
              style={{ borderRadius: 16 }}
              alt="Coral"
            />
            <div style={{ display: "flex", flexDirection: "column" }}>
              <div style={{ fontSize: 80, fontWeight: 700, lineHeight: 1 }}>
                Coral
              </div>
              <div style={{ fontSize: 30, opacity: 0.9 }}>
                Hypixel Stats by Urchin
              </div>
            </div>
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
