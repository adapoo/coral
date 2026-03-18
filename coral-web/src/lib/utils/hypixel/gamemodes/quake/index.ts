import { formatCompactWhole } from "@/lib/utils/general/format";
import { C } from "@/lib/utils/general/colors";

export function getQuakePrefix(kills: number): string {
  const p = findPrefix(kills);
  const formatted = formatCompactWhole(kills);
  return p.fmt(formatted);
}

export function getQuakeTrigger(trigger: string): number {
  const parts = trigger.toLowerCase().split("_");
  const numeric = parts
    .map((token) =>
      INDEX_WORDS.includes(token)
        ? INDEX_WORDS.indexOf(token)
        : token === "point"
        ? "."
        : ""
    )
    .join("");
  const val = Number(numeric);
  return Number.isFinite(val) && val > 0 ? val : 1.3;
}

type Prefix = { req: number; fmt: (n: string) => string };

const PREFIXES: Prefix[] = [
  { fmt: (n) => `${C.DARK_GRAY}[${n}]`, req: 0 },
  { fmt: (n) => `${C.GRAY}[${n}]`, req: 25_000 },
  { fmt: (n) => `${C.WHITE}[${n}]`, req: 50_000 },
  { fmt: (n) => `${C.DARK_GREEN}[${n}]`, req: 75_000 },
  { fmt: (n) => `${C.YELLOW}[${n}]`, req: 100_000 },
  { fmt: (n) => `${C.GREEN}[${n}]`, req: 200_000 },
  { fmt: (n) => `${C.BLUE}[${n}]`, req: 300_000 },
  { fmt: (n) => `${C.DARK_AQUA}[${n}]`, req: 400_000 },
  { fmt: (n) => `${C.LIGHT_PURPLE}[${n}]`, req: 500_000 },
  { fmt: (n) => `${C.DARK_PURPLE}[${n}]`, req: 600_000 },
  { fmt: (n) => `${C.RED}[${n}]`, req: 750_000 },
  { fmt: (n) => `${C.GOLD}[${n}]`, req: 1_000_000 },
  { fmt: (n) => `${C.BLACK}[${n}]`, req: 2_000_000 },
];

const INDEX_WORDS = [
  "zero",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
];

function findPrefix(score: number): Prefix {
  let best = PREFIXES[0];
  for (const p of PREFIXES) {
    if (score >= p.req) best = p;
    else break;
  }
  return best;
}
