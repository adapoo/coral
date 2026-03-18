import { findThresholdLinear } from "@/lib/utils/general/threshold";
import { C, F, colorCodeToHex } from "@/lib/utils/general/colors";

const PRESTIGE_NAMES: { req: number; name: string; symbol: string }[] = [
  { req: 0, name: "None", symbol: "✫" },
  { req: 100, name: "Iron", symbol: "✫" },
  { req: 200, name: "Gold", symbol: "✫" },
  { req: 300, name: "Diamond", symbol: "✫" },
  { req: 400, name: "Emerald", symbol: "✫" },
  { req: 500, name: "Sapphire", symbol: "✫" },
  { req: 600, name: "Ruby", symbol: "✫" },
  { req: 700, name: "Crystal", symbol: "✫" },
  { req: 800, name: "Opal", symbol: "✫" },
  { req: 900, name: "Amethyst", symbol: "✫" },
  { req: 1000, name: "Rainbow", symbol: "✫" },
  { req: 1100, name: "Iron Prime", symbol: "✪" },
  { req: 1200, name: "Gold Prime", symbol: "✪" },
  { req: 1300, name: "Diamond Prime", symbol: "✪" },
  { req: 1400, name: "Emerald Prime", symbol: "✪" },
  { req: 1500, name: "Sapphire Prime", symbol: "✪" },
  { req: 1600, name: "Ruby Prime", symbol: "✪" },
  { req: 1700, name: "Crystal Prime", symbol: "✪" },
  { req: 1800, name: "Opal Prime", symbol: "✪" },
  { req: 1900, name: "Amethyst Prime", symbol: "✪" },
  { req: 2000, name: "Mirror", symbol: "✪" },
  { req: 2100, name: "Light", symbol: "⚝" },
  { req: 2200, name: "Dawn", symbol: "⚝" },
  { req: 2300, name: "Dusk", symbol: "⚝" },
  { req: 2400, name: "Air", symbol: "⚝" },
  { req: 2500, name: "Wind", symbol: "⚝" },
  { req: 2600, name: "Nebula", symbol: "⚝" },
  { req: 2700, name: "Thunder", symbol: "⚝" },
  { req: 2800, name: "Earth", symbol: "⚝" },
  { req: 2900, name: "Water", symbol: "⚝" },
  { req: 3000, name: "Fire", symbol: "⚝" },
  { req: 3100, name: "Sunshine", symbol: "✥" },
  { req: 3200, name: "Eclipse", symbol: "✥" },
  { req: 3300, name: "Gamma", symbol: "✥" },
  { req: 3400, name: "Majestic", symbol: "✥" },
  { req: 3500, name: "Andesine", symbol: "✥" },
  { req: 3600, name: "Marine", symbol: "✥" },
  { req: 3700, name: "Element", symbol: "✥" },
  { req: 3800, name: "Galaxy", symbol: "✥" },
  { req: 3900, name: "Atomic", symbol: "✥" },
  { req: 4000, name: "Sunset", symbol: "✥" },
  { req: 4100, name: "Time", symbol: "✥" },
  { req: 4200, name: "Winter", symbol: "✥" },
  { req: 4300, name: "Obsidian", symbol: "✥" },
  { req: 4400, name: "Spring", symbol: "✥" },
  { req: 4500, name: "Ice", symbol: "✥" },
  { req: 4600, name: "Summer", symbol: "✥" },
  { req: 4700, name: "Spinel", symbol: "✥" },
  { req: 4800, name: "Autumn", symbol: "✥" },
  { req: 4900, name: "Mystic", symbol: "✥" },
  { req: 5000, name: "Eternal", symbol: "✥" },
];

const PRESTIGE_COLORS: { req: number; fn: (n: number) => string }[] = [
  { req: 0, fn: (n) => `${C.GRAY}[${n}✫]` },
  { req: 100, fn: (n) => `${C.WHITE}[${n}✫]` },
  { req: 200, fn: (n) => `${C.GOLD}[${n}✫]` },
  { req: 300, fn: (n) => `${C.AQUA}[${n}✫]` },
  { req: 400, fn: (n) => `${C.DARK_GREEN}[${n}✫]` },
  { req: 500, fn: (n) => `${C.DARK_AQUA}[${n}✫]` },
  { req: 600, fn: (n) => `${C.DARK_RED}[${n}✫]` },
  { req: 700, fn: (n) => `${C.LIGHT_PURPLE}[${n}✫]` },
  { req: 800, fn: (n) => `${C.BLUE}[${n}✫]` },
  { req: 900, fn: (n) => `${C.DARK_PURPLE}[${n}✫]` },
  {
    req: 1000,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.RED}[${C.GOLD}${nums[0]}${C.YELLOW}${nums[1]}${C.GREEN}${nums[2]}${C.AQUA}${nums[3]}${C.LIGHT_PURPLE}✫${C.DARK_PURPLE}]`;
    },
  },
  { req: 1100, fn: (n) => `${C.GRAY}[${C.WHITE}${n}${C.GRAY}✪]` },
  { req: 1200, fn: (n) => `${C.GRAY}[${C.YELLOW}${n}${C.GOLD}✪${C.GRAY}]` },
  { req: 1300, fn: (n) => `${C.GRAY}[${C.AQUA}${n}${C.DARK_AQUA}✪${C.GRAY}]` },
  {
    req: 1400,
    fn: (n) => `${C.GRAY}[${C.GREEN}${n}${C.DARK_GREEN}✪${C.GRAY}]`,
  },
  { req: 1500, fn: (n) => `${C.GRAY}[${C.DARK_AQUA}${n}${C.BLUE}✪${C.GRAY}]` },
  { req: 1600, fn: (n) => `${C.GRAY}[${C.RED}${n}${C.DARK_RED}✪${C.GRAY}]` },
  {
    req: 1700,
    fn: (n) => `${C.GRAY}[${C.LIGHT_PURPLE}${n}${C.DARK_PURPLE}✪${C.GRAY}]`,
  },
  { req: 1800, fn: (n) => `${C.GRAY}[${C.BLUE}${n}${C.DARK_BLUE}✪${C.GRAY}]` },
  {
    req: 1900,
    fn: (n) => `${C.GRAY}[${C.DARK_PURPLE}${n}${C.DARK_GRAY}✪${C.GRAY}]`,
  },
  {
    req: 2000,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_GRAY}[${C.GRAY}${nums[0]}${C.WHITE}${nums[1]}${nums[2]}${C.GRAY}${nums[3]}✪${C.DARK_GRAY}]`;
    },
  },
  {
    req: 2100,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.WHITE}[${nums[0]}${C.YELLOW}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${F.BOLD}⚝${F.RESET}${C.GOLD}]`;
    },
  },
  {
    req: 2200,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.GOLD}[${nums[0]}${C.WHITE}${nums[1]}${nums[2]}${C.AQUA}${nums[3]}${C.DARK_AQUA}${F.BOLD}⚝${F.RESET}${C.DARK_AQUA}]`;
    },
  },
  {
    req: 2300,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_PURPLE}[${nums[0]}${C.LIGHT_PURPLE}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${C.YELLOW}${F.BOLD}⚝${F.RESET}${C.YELLOW}]`;
    },
  },
  {
    req: 2400,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.AQUA}[${nums[0]}${C.WHITE}${nums[1]}${nums[2]}${C.GRAY}${nums[3]}${F.BOLD}⚝${F.RESET}${C.DARK_GRAY}]`;
    },
  },
  {
    req: 2500,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.WHITE}[${nums[0]}${C.GREEN}${nums[1]}${nums[2]}${C.DARK_GREEN}${nums[3]}${F.BOLD}⚝${F.RESET}${C.DARK_GREEN}]`;
    },
  },
  {
    req: 2600,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_RED}[${nums[0]}${C.RED}${nums[1]}${nums[2]}${C.LIGHT_PURPLE}${nums[3]}${F.BOLD}⚝${F.RESET}${C.LIGHT_PURPLE}]`;
    },
  },
  {
    req: 2700,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.YELLOW}[${nums[0]}${C.WHITE}${nums[1]}${nums[2]}${C.DARK_GRAY}${nums[3]}${F.BOLD}⚝${F.RESET}${C.DARK_GRAY}]`;
    },
  },
  {
    req: 2800,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.GREEN}[${nums[0]}${C.DARK_GREEN}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${F.BOLD}⚝${F.RESET}${C.YELLOW}]`;
    },
  },
  {
    req: 2900,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.AQUA}[${nums[0]}${C.DARK_AQUA}${nums[1]}${nums[2]}${C.BLUE}${nums[3]}${F.BOLD}⚝${F.RESET}${C.DARK_BLUE}]`;
    },
  },
  {
    req: 3000,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.YELLOW}[${nums[0]}${C.GOLD}${nums[1]}${nums[2]}${C.RED}${nums[3]}${F.BOLD}⚝${F.RESET}${C.DARK_RED}]`;
    },
  },
  {
    req: 3100,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.BLUE}[${nums[0]}${C.DARK_GREEN}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${F.BOLD}✥${F.RESET}${C.YELLOW}]`;
    },
  },
  {
    req: 3200,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.RED}[${C.DARK_RED}${nums[0]}${C.GRAY}${nums[1]}${nums[2]}${C.DARK_RED}${nums[3]}${C.RED}${F.BOLD}✥${F.RESET}${C.RED}]`;
    },
  },
  {
    req: 3300,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.BLUE}[${nums[0]}${nums[1]}${C.LIGHT_PURPLE}${nums[2]}${C.RED}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_RED}]`;
    },
  },
  {
    req: 3400,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_GREEN}[${C.GREEN}${nums[0]}${C.LIGHT_PURPLE}${nums[1]}${nums[2]}${C.DARK_PURPLE}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_GREEN}]`;
    },
  },
  {
    req: 3500,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.RED}[${nums[0]}${C.DARK_RED}${nums[1]}${nums[2]}${C.DARK_GREEN}${nums[3]}${C.GREEN}${F.BOLD}✥${F.RESET}${C.GREEN}]`;
    },
  },
  {
    req: 3600,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.GREEN}[${nums[0]}${nums[1]}${C.AQUA}${nums[2]}${C.BLUE}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_BLUE}]`;
    },
  },
  {
    req: 3700,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_RED}[${nums[0]}${C.RED}${nums[1]}${nums[2]}${C.AQUA}${nums[3]}${C.DARK_AQUA}${F.BOLD}✥${F.RESET}${C.DARK_AQUA}]`;
    },
  },
  {
    req: 3800,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_BLUE}[${nums[0]}${C.BLUE}${nums[1]}${C.DARK_PURPLE}${nums[2]}${nums[3]}${C.LIGHT_PURPLE}${F.BOLD}✥${F.RESET}${C.DARK_BLUE}]`;
    },
  },
  {
    req: 3900,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.RED}[${nums[0]}${C.GREEN}${nums[1]}${nums[2]}${C.DARK_AQUA}${nums[3]}${C.BLUE}${F.BOLD}✥${F.RESET}${C.BLUE}]`;
    },
  },
  {
    req: 4000,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_PURPLE}[${nums[0]}${C.RED}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${F.BOLD}✥${F.RESET}${C.YELLOW}]`;
    },
  },
  {
    req: 4100,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.YELLOW}[${nums[0]}${C.GOLD}${nums[1]}${C.RED}${nums[2]}${C.LIGHT_PURPLE}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_PURPLE}]`;
    },
  },
  {
    req: 4200,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_BLUE}[${C.BLUE}${nums[0]}${C.DARK_AQUA}${nums[1]}${C.AQUA}${nums[2]}${C.WHITE}${nums[3]}${C.GRAY}${F.BOLD}✥${F.RESET}${C.GRAY}]`;
    },
  },
  {
    req: 4300,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.BLACK}[${C.DARK_PURPLE}${nums[0]}${C.DARK_GRAY}${nums[1]}${nums[2]}${C.DARK_PURPLE}${nums[3]}${F.BOLD}✥${F.RESET}${C.BLACK}]`;
    },
  },
  {
    req: 4400,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_GREEN}[${nums[0]}${C.GREEN}${nums[1]}${C.YELLOW}${nums[2]}${C.GOLD}${nums[3]}${C.DARK_PURPLE}${F.BOLD}✥${F.RESET}${C.LIGHT_PURPLE}]`;
    },
  },
  {
    req: 4500,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.WHITE}[${nums[0]}${C.AQUA}${nums[1]}${nums[2]}${C.DARK_GREEN}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_GREEN}]`;
    },
  },
  {
    req: 4600,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_GREEN}[${C.AQUA}${nums[0]}${C.YELLOW}${nums[1]}${nums[2]}${C.GOLD}${nums[3]}${C.LIGHT_PURPLE}${F.BOLD}✥${F.RESET}${C.DARK_PURPLE}]`;
    },
  },
  {
    req: 4700,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.WHITE}[${C.DARK_RED}${nums[0]}${C.RED}${nums[1]}${nums[2]}${C.BLUE}${nums[3]}${C.DARK_BLUE}${F.BOLD}✥${F.RESET}${C.BLUE}]`;
    },
  },
  {
    req: 4800,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_PURPLE}[${nums[0]}${C.RED}${nums[1]}${C.GOLD}${nums[2]}${C.YELLOW}${nums[3]}${C.AQUA}${F.BOLD}✥${F.RESET}${C.DARK_AQUA}]`;
    },
  },
  {
    req: 4900,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_GREEN}[${C.GREEN}${nums[0]}${C.WHITE}${nums[1]}${nums[2]}${C.GREEN}${nums[3]}${F.BOLD}✥${F.RESET}${C.DARK_GREEN}]`;
    },
  },
  {
    req: 5000,
    fn: (n) => {
      const nums = [...n.toString()];
      return `${C.DARK_RED}[${nums[0]}${C.DARK_PURPLE}${nums[1]}${C.BLUE}${nums[2]}${nums[3]}${C.DARK_BLUE}${F.BOLD}✥${F.RESET}${C.BLACK}]`;
    },
  },
];

export function getBedwarsExpRequirement(level: number) {
  const progress = level % 100;
  if (progress > 3) return 5000;
  const levels: Record<number, number> = { 0: 500, 1: 1000, 2: 2000, 3: 3500 };
  return levels[progress];
}

export function getBedwarsLevel(exp = 0): number {
  const prestiges = Math.floor(exp / 487000);
  let level = prestiges * 100;
  let remainingExp = exp - prestiges * 487000;
  for (let i = 0; i < 4; ++i) {
    const expForNextLevel = getBedwarsExpRequirement(i);
    if (remainingExp < expForNextLevel) break;
    level++;
    remainingExp -= expForNextLevel;
  }
  return level + remainingExp / getBedwarsExpRequirement(level + 1);
}

export function formatBedwarsStar(star: number): string {
  const s = Math.floor(star);
  return findThresholdLinear(PRESTIGE_COLORS, s).fn(s);
}

export function getBedwarsProgressBarColors(star: number): {
  className: string;
  style: React.CSSProperties;
} {
  const s = Math.floor(star);
  const prestige = findThresholdLinear(PRESTIGE_COLORS, s);
  const colorString = prestige.fn(s);

  // Extract unique colors from the formatted string
  const colors: string[] = [];
  const colorRegex = /(§[0-9a-f])/g;
  let match;

  while ((match = colorRegex.exec(colorString)) !== null) {
    const colorCode = match[1];
    const hexColor = colorCodeToHex(colorCode);
    if (hexColor && !colors.includes(hexColor)) {
      colors.push(hexColor);
    }
  }

  if (colors.length === 0) {
    return {
      className: "",
      style: {
        background: "linear-gradient(to right, #eab308, #ca8a04)",
      },
    };
  }

  if (colors.length === 1) {
    return {
      className: "",
      style: {
        background: `linear-gradient(to right, ${colors[0]}, ${colors[0]}cc)`,
      } as React.CSSProperties,
    };
  }

  if (colors.length === 2) {
    return {
      className: "",
      style: {
        background: `linear-gradient(to right, ${colors[0]}, ${colors[1]})`,
      } as React.CSSProperties,
    };
  }

  return {
    className: "",
    style: {
      background: `linear-gradient(to right, ${colors.join(", ")})`,
    } as React.CSSProperties,
  };
}

export function getBedwarsPrestige(star: number): string {
  const s = Math.floor(star);
  const prestige = findThresholdLinear(PRESTIGE_NAMES, s);
  const colorPrestige = findThresholdLinear(PRESTIGE_COLORS, s);

  const coloredStar = colorPrestige.fn(s);
  const colorMatches = coloredStar.match(/(§[0-9a-f])/g) || [];
  const colors = colorMatches.length > 0 ? colorMatches : [C.GRAY];

  const chars = [...prestige.name];
  const baseCharsPerColor = Math.floor(chars.length / colors.length);
  const extraChars = chars.length % colors.length;
  const middleStart = Math.floor((colors.length - extraChars) / 2);

  let result = "";
  let charIndex = 0;

  for (let colorIndex = 0; colorIndex < colors.length; colorIndex++) {
    const charsForThisColor =
      baseCharsPerColor +
      (colorIndex >= middleStart && colorIndex < middleStart + extraChars
        ? 1
        : 0);

    for (let j = 0; j < charsForThisColor; j++) {
      result += `${colors[colorIndex]}${chars[charIndex]}`;
      charIndex++;
    }
  }

  const starColor = colors[colors.length - 1];
  result += ` ${starColor}${prestige.symbol}`;

  return result;
}

const DOOR_NAMES = [
  "Throne Door",
  "Hotel Door",
  "Desert Door",
  "Electronic Door",
  "Door from the Sky",
  "Door as seen on TV",
  "Skyscraper Door",
  "Arcade Door",
  "Intricate Door",
  "Space Door",
  "D   o  0      r",
  "Garage Door",
  "Owner's Office",
];

export function getBedwarsHighestDoor(room: Record<string, unknown>): string {
  const unlockedDoorEntries = Object.entries(room).filter(([, v]) => v);
  const highestDoorIndex = unlockedDoorEntries.length - 1;
  return highestDoorIndex >= 0 ? DOOR_NAMES[highestDoorIndex] : "None";
}

export function getBedwarsWalletCapacity(bagType: string): number {
  switch (bagType) {
    case "MINI_WALLET":
      return 25;
    case "LIGHT_SLUMBERS_WALLET":
      return 99;
    case "LIGHT_IMPERIAL_WALLET":
      return 500;
    case "EXPLORERS_WALLET":
      return 5000;
    case "HOTEL_STAFF_WALLET":
      return 10000;
    case "PLATINUM_MEMBERSHIP_WALLET":
      return 100000;
    default:
      return 0;
  }
}
