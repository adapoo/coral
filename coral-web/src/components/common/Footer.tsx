import React from "react";
import { ThemeToggle } from "@/components/ThemeToggle";

export function Footer() {
  return (
    <footer className="mt-16 mb-8 w-full px-6">
      <div
        className="max-w-7xl mx-auto flex items-center justify-between gap-4"
        style={{ fontFamily: "var(--font-inter)" }}
      >
        <div className="text-xs opacity-70 leading-relaxed">
          <p>
            Coral is an independent project by{" "}
            <a
              href="https://urchin.ws"
              target="_blank"
              rel="noreferrer"
              className="underline hover:opacity-90"
            >
              Urchin
            </a>{" "}
            and is not affiliated with Hypixel Inc., Mojang AB, or Microsoft.
          </p>
          <p className="mt-1">
            Powered by the{" "}
            <a
              href="https://hypixel.net"
              target="_blank"
              rel="noreferrer"
              className="underline hover:opacity-90"
            >
              Hypixel API
            </a>
            . Skins provided by{" "}
            <a
              href="https://visage.surgeplay.com"
              target="_blank"
              rel="noreferrer"
              className="underline hover:opacity-90"
            >
              Visage
            </a>
            .
          </p>
        </div>
        <div className="shrink-0 flex items-center gap-2">
          {/* @ts-ignore */}
          <ThemeToggle />
        </div>
      </div>
    </footer>
  );
}
