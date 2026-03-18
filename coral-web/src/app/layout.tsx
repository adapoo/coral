import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Footer } from "@/components/common/Footer";
import { Analytics } from "@vercel/analytics/next";
import { SpeedInsights } from "@vercel/speed-insights/next";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
});

const siteUrl = process.env.INTERNAL_BASE_URL || "https://coral.urchin.ws";

export const metadata: Metadata = {
  metadataBase: new URL(siteUrl),
  title: {
    default: "Coral",
    template: "%s - Coral",
  },
  description:
    "Coral by Urchin – a clean, fast Hypixel stats viewer for Minecraft players (Bed Wars, SkyWars, Duels, Pit, and more).",
  keywords: [
    "Hypixel",
    "Minecraft",
    "stats",
    "Bedwars",
    "SkyWars",
    "Duels",
    "Pit",
    "Coral",
    "Urchin",
  ],
  openGraph: {
    type: "website",
    url: siteUrl,
    siteName: "Coral",
    title: "Coral – Hypixel Stats by Urchin",
    description:
      "View Hypixel player stats with a beautiful UI. Bed Wars, SkyWars, Duels, Pit, and more.",
    images: [
      {
        url: "/api/og/site",
        width: 1200,
        height: 630,
        alt: "Coral by Urchin",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    creator: "@UrchinAPI",
    title: "Coral – Hypixel Stats by Urchin",
    description:
      "View Hypixel player stats with a beautiful UI. Bed Wars, SkyWars, Duels, Pit, and more.",
    images: ["/api/og/site"],
  },
  alternates: {
    canonical: siteUrl,
  },
  icons: {
    icon: "/favicon.ico",
  },
  verification: {
    google: "WBx1oBDYRm4sIIBpcibN6CwkEs7t5sgG30Ng-ZW4X00",
  },
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <head>
        <link
          rel="preload"
          as="font"
          type="font/woff2"
          href="/fonts/minecraft.woff2"
          crossOrigin="anonymous"
        />
        <link
          rel="preload"
          as="font"
          type="font/woff2"
          href="/fonts/minecraft-bold.woff2"
          crossOrigin="anonymous"
        />
        <link
          rel="preload"
          as="font"
          type="font/woff2"
          href="/fonts/minecraft-italic.woff2"
          crossOrigin="anonymous"
        />
        <link
          rel="preload"
          as="font"
          type="font/woff2"
          href="/fonts/minecraft-bold-italic.woff2"
          crossOrigin="anonymous"
        />
        <link
          rel="preload"
          as="font"
          type="font/woff2"
          href="/fonts/unifont.woff2"
          crossOrigin="anonymous"
        />
      </head>
      <body className={`${inter.variable} antialiased tracking-normal`}>
        {children}
        <Footer />
        <Analytics />
        <SpeedInsights />
      </body>
    </html>
  );
}
