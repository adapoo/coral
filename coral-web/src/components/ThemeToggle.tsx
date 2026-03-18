"use client";

import { useEffect, useState } from "react";
import { MoonStar, Sun } from "lucide-react";

function getInitialTheme(): "light" | "dark" {
  if (typeof document === "undefined") return "light";
  if (document.cookie.includes("coral_theme=dark")) return "dark";
  if (document.cookie.includes("coral_theme=light")) return "light";
  return window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

export function ThemeToggle() {
  const [theme, setTheme] = useState<"light" | "dark">("light");

  useEffect(() => {
    setTheme(getInitialTheme());
  }, []);

  useEffect(() => {
    if (typeof document === "undefined") return;
    document.documentElement.setAttribute("data-theme", theme);
    const expires = new Date(
      Date.now() + 365 * 24 * 60 * 60 * 1000
    ).toUTCString();
    document.cookie = `coral_theme=${theme}; Path=/; Expires=${expires}; SameSite=Lax`;
  }, [theme]);

  const toggle = () => setTheme((t) => (t === "light" ? "dark" : "light"));

  return (
    <button
      type="button"
      onClick={toggle}
      aria-label="Toggle theme"
      className="h-8 w-8 grid place-items-center rounded-md"
    >
      {theme === "dark" ? (
        <Sun size={16} strokeWidth={2.25} />
      ) : (
        <MoonStar size={16} strokeWidth={2.25} />
      )}
    </button>
  );
}
