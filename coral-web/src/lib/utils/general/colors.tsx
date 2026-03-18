import type { ReactNode } from "react";

export const COLOR_VAR_MAP: Record<string, string> = {
  "0": "var(--color-0)",
  "1": "var(--color-1)",
  "2": "var(--color-2)",
  "3": "var(--color-3)",
  "4": "var(--color-4)",
  "5": "var(--color-5)",
  "6": "var(--color-6)",
  "7": "var(--color-7)",
  "8": "var(--color-8)",
  "9": "var(--color-9)",
  a: "var(--color-a)",
  b: "var(--color-b)",
  c: "var(--color-c)",
  d: "var(--color-d)",
  e: "var(--color-e)",
  f: "var(--color-f)",
};

export const SHADOW_VAR_MAP: Record<string, string> = {
  "var(--color-0)": "var(--shadow-0)",
  "var(--color-1)": "var(--shadow-1)",
  "var(--color-2)": "var(--shadow-2)",
  "var(--color-3)": "var(--shadow-3)",
  "var(--color-4)": "var(--shadow-4)",
  "var(--color-5)": "var(--shadow-5)",
  "var(--color-6)": "var(--shadow-6)",
  "var(--color-7)": "var(--shadow-7)",
  "var(--color-8)": "var(--shadow-8)",
  "var(--color-9)": "var(--shadow-9)",
  "var(--color-a)": "var(--shadow-a)",
  "var(--color-b)": "var(--shadow-b)",
  "var(--color-c)": "var(--shadow-c)",
  "var(--color-d)": "var(--shadow-d)",
  "var(--color-e)": "var(--shadow-e)",
  "var(--color-f)": "var(--shadow-f)",
};

export const C = {
  BLACK: "§0",
  DARK_BLUE: "§1",
  DARK_GREEN: "§2",
  DARK_AQUA: "§3",
  DARK_RED: "§4",
  DARK_PURPLE: "§5",
  GOLD: "§6",
  GRAY: "§7",
  DARK_GRAY: "§8",
  BLUE: "§9",
  GREEN: "§a",
  AQUA: "§b",
  RED: "§c",
  LIGHT_PURPLE: "§d",
  YELLOW: "§e",
  WHITE: "§f",
};

export const F = {
  BOLD: "§l",
  RESET: "§r",
  UNDERLINE: "§n",
  STRIKETHROUGH: "§m",
  ITALIC: "§o",
};

export const COLOR_TO_HEX: Record<string, string> = {
  [C.BLACK]: "#000000",
  [C.DARK_BLUE]: "#0000AA",
  [C.DARK_GREEN]: "#00AA00",
  [C.DARK_AQUA]: "#00AAAA",
  [C.DARK_RED]: "#AA0000",
  [C.DARK_PURPLE]: "#AA00AA",
  [C.GOLD]: "#FFAA00",
  [C.GRAY]: "#AAAAAA",
  [C.DARK_GRAY]: "#555555",
  [C.BLUE]: "#5555FF",
  [C.GREEN]: "#55FF55",
  [C.AQUA]: "#55FFFF",
  [C.RED]: "#FF5555",
  [C.LIGHT_PURPLE]: "#FF55FF",
  [C.YELLOW]: "#FFFF55",
  [C.WHITE]: "#FFFFFF",
};

export function colorCodeToVar(code: string): string {
  return COLOR_VAR_MAP[code] || "var(--color-7)";
}

export function colorCodeToHex(code: string): string {
  return COLOR_TO_HEX[code] || "#FFFFFF";
}

export function colorJSX(text: string, opts?: { withShadow?: boolean }) {
  const parts: ReactNode[] = [];
  let currentColor = "#FFFFFF";
  let bold = false;
  let buffer = "";
  const flush = () => {
    if (buffer.length === 0) return;
    const textShadow =
      opts?.withShadow !== false
        ? `3px 3px ${SHADOW_VAR_MAP[currentColor] || "rgba(0,0,0,0.35)"}`
        : undefined;
    parts.push(
      <span
        key={parts.length}
        style={{
          color: currentColor,
          fontWeight: bold ? 700 : 400,
          textShadow,
        }}
      >
        {buffer}
      </span>
    );
    buffer = "";
  };
  for (let i = 0; i < text.length; i++) {
    const ch = text[i];
    if (ch === "§" && i + 1 < text.length) {
      const code = text[i + 1];
      i++;
      if (code in COLOR_VAR_MAP) {
        flush();
        currentColor = COLOR_VAR_MAP[code as keyof typeof COLOR_VAR_MAP];
        continue;
      }
      if (code === "l") {
        flush();
        bold = true;
        continue;
      }
      if (code === "r") {
        flush();
        bold = false;
        currentColor = "#FFFFFF";
        continue;
      }
      continue;
    }
    buffer += ch;
  }
  flush();
  return <>{parts}</>;
}
