import { romanNumeral } from "@/lib/utils/general/format";
import { C, F } from "@/lib/utils/general/colors";
import { findThresholdLinear } from "@/lib/utils/general/threshold";

type TitleDef = {
  req: number;
  inc: number;
  title: string;
  color: string;
  bold?: boolean;
  semi?: boolean;
  max?: number;
};

const baseTitles: TitleDef[] = [
  { req: 0, inc: 0, title: "None", color: C.GRAY },
  { req: 50, inc: 10, title: "Rookie", color: C.DARK_GRAY },
  { req: 100, inc: 30, title: "Iron", color: C.WHITE },
  { req: 250, inc: 50, title: "Gold", color: C.GOLD },
  { req: 500, inc: 100, title: "Diamond", color: C.DARK_AQUA },
  { req: 1000, inc: 200, title: "Master", color: C.DARK_GREEN },
  { req: 2000, inc: 600, title: "Legend", color: C.DARK_RED, bold: true },
  {
    req: 5000,
    inc: 1000,
    title: "Grandmaster",
    color: C.YELLOW,
    bold: true,
  },
  {
    req: 10000,
    inc: 3000,
    title: "Godlike",
    color: C.DARK_PURPLE,
    bold: true,
  },
  { req: 25000, inc: 5000, title: "CELESTIAL", color: C.AQUA, bold: true },
  {
    req: 50000,
    inc: 10000,
    title: "DIVINE",
    color: C.LIGHT_PURPLE,
    bold: true,
  },
  {
    req: 100000,
    inc: 10000,
    max: 50,
    title: "ASCENDED",
    color: C.RED,
    bold: true,
  },
];

const baseTitlesOverall: TitleDef[] = baseTitles.map((t) => ({
  ...t,
  req: t.req * 2,
  inc: t.inc ? t.inc * 2 : t.inc,
}));

export const DUELS_MODES = {
  // Duel Arena
  duel_arena: { label: "Duel Arena", category: "Duel Arena" },

  // Blitz
  blitz_duel: { label: "Blitz Duel", category: "Blitz" },

  // Bow
  bow_duel: { label: "Bow Duel", category: "Bow" },

  // Spleef
  bowspleef_duel: { label: "Bow Spleef Duel", category: "Spleef" },
  spleef_duel: { label: "Spleef Duel", category: "Spleef" },

  // Boxing
  boxing_duel: { label: "Boxing Duel", category: "Boxing" },

  // Classic
  classic_duel: { label: "Classic Duel", category: "Classic" },
  classic_doubles: { label: "Classic Doubles", category: "Classic" },

  // Combo
  combo_duel: { label: "Combo Duel", category: "Combo" },

  // Sumo
  sumo_duel: { label: "Sumo Duel", category: "Sumo" },

  // NoDebuff
  potion_duel: { label: "NoDebuff Duel", category: "NoDebuff" },

  // Parkour
  parkour_eight: { label: "Parkour Duels", category: "Parkour" },

  // Mega Walls
  mw_duel: { label: "Mega Walls Duel", category: "Mega Walls" },
  mw_doubles: { label: "Mega Walls Doubles", category: "Mega Walls" },

  // UHC
  uhc_duel: { label: "UHC Duel", category: "UHC" },
  uhc_doubles: { label: "UHC Doubles", category: "UHC" },
  uhc_four: { label: "UHC Teams", category: "UHC" },
  uhc_meetup: { label: "UHC Deathmatch", category: "UHC" },

  // SkyWars
  sw_duel: { label: "SkyWars Duel", category: "SkyWars" },
  sw_doubles: { label: "SkyWars Doubles", category: "SkyWars" },

  // OP
  op_duel: { label: "OP Duel", category: "OP" },
  op_doubles: { label: "OP Doubles", category: "OP" },

  // The Bridge
  bridge_duel: { label: "Bridge Duel", category: "The Bridge" },
  bridge_doubles: { label: "Bridge Doubles", category: "The Bridge" },
  bridge_threes: { label: "Bridge 3v3", category: "The Bridge" },
  bridge_four: { label: "Bridge 4v4", category: "The Bridge" },
  bridge_2v2v2v2: { label: "Bridge 2v2v2v2", category: "The Bridge" },
  bridge_3v3v3v3: { label: "Bridge 3v3v3v3", category: "The Bridge" },
  capture_threes: { label: "Bridge CTF", category: "The Bridge" },

  // Bed Wars
  bedwars_two_one_duels: { label: "Bed Wars Duel", category: "Bed Wars" },
  bedwars_two_one_duels_rush: { label: "Bed Rush Duel", category: "Bed Wars" },

  // Quakecraft
  quake_duel: { label: "Quakecraft Duel", category: "Quakecraft" },
};

export const DUELS_CATEGORIES = [
  "Duel Arena",
  "Blitz",
  "Bow",
  "Spleef",
  "Boxing",
  "Classic",
  "Combo",
  "Sumo",
  "NoDebuff",
  "Parkour",
  "Mega Walls",
  "UHC",
  "SkyWars",
  "OP",
  "The Bridge",
  "Bed Wars",
  "Quakecraft",
];

export const DUELS_CATEGORY_KEYS: { [key: string]: string } = {
  "Duel Arena": "arena",
  Blitz: "blitz",
  Bow: "bow",
  Spleef: "spleef",
  Boxing: "boxing",
  Classic: "classic",
  Combo: "combo",
  Sumo: "sumo",
  NoDebuff: "no_debuff",
  Parkour: "parkour",
  "Mega Walls": "mega_walls",
  UHC: "uhc",
  SkyWars: "skywars",
  OP: "op",
  "The Bridge": "bridge",
  "Bed Wars": "bedwars",
  Quakecraft: "quake",
};

export function getDuelsDivision(
  wins: number,
  compact: boolean = false
): {
  formatted: string;
  raw: string;
  color: string;
  title: string;
  numeral: string;
} {
  const {
    req,
    inc,
    title,
    color,
    bold = false,
    semi = false,
    max,
  } = findThresholdLinear(baseTitlesOverall, wins);
  const remaining = wins - req;
  let index = (inc ? Math.floor(remaining / inc) : inc) + 1;
  index = max ? Math.min(index, max) : index;
  const numeral = romanNumeral(index);

  if (compact) {
    const formatted = `${color}${numeral}${F.RESET}`;
    const raw = numeral;
    return { formatted, raw, color, title, numeral };
  } else {
    const formatted = `${bold ? F.BOLD : ""}${color}${
      semi ? F.BOLD : ""
    }${title}${numeral ? ` ${numeral}` : ""}${F.RESET}`;
    const raw = `${title}${numeral ? ` ${numeral}` : ""}`;
    return { formatted, raw, color, title, numeral };
  }
}

export function getMostPlayedDuelsMode(stats: Record<string, unknown>): string {
  let bestKey = "";
  let bestWins = -1;
  for (const [prefix, mode] of Object.entries(DUELS_MODES)) {
    const winsKey = `${prefix}_wins`;
    const wins = Number((stats as any)[winsKey] ?? 0);
    if (wins > bestWins) {
      bestWins = wins;
      bestKey = mode.label;
    }
  }
  return bestWins > 0 ? bestKey : "Overall";
}

export function getDuelsWinstreak(
  mode: string,
  stats: Record<string, unknown>
): number | null {
  const individualKey = `current_winstreak_mode_${mode}`;
  if (stats[individualKey] !== undefined) {
    return Number(stats[individualKey]);
  }

  return null;
}

export function getDuelsBestWinstreak(
  mode: string,
  stats: Record<string, unknown>
): number | null {
  const individualKey = `best_winstreak_mode_${mode}`;
  if (stats[individualKey] !== undefined) {
    return Number(stats[individualKey]);
  }

  return null;
}

export function getDuelsOverallWinstreak(
  stats: Record<string, unknown>,
  type: "current" | "best"
): number | null {
  const winstreakKey = `${type}_all_modes_winstreak`;
  const winstreak = stats[winstreakKey];
  return winstreak !== undefined ? Number(winstreak) : null;
}

export function getDuelsModeStats(
  stats: Record<string, unknown>,
  modeKey: string
) {
  const mode = DUELS_MODES[modeKey as keyof typeof DUELS_MODES];
  if (!mode) return null;

  const wins = Number(stats[`${modeKey}_wins`] || 0);
  const losses = Number(stats[`${modeKey}_losses`] || 0);
  const kills = Number(stats[`${modeKey}_kills`] || 0);
  const deaths = Number(stats[`${modeKey}_deaths`] || 0);
  const currentWinstreakKey = `current_winstreak_mode_${modeKey}`;
  const bestWinstreakKey = `best_winstreak_mode_${modeKey}`;

  const currentWinstreak =
    stats[currentWinstreakKey] !== undefined
      ? Number(stats[currentWinstreakKey])
      : null;
  const bestWinstreak =
    stats[bestWinstreakKey] !== undefined
      ? Number(stats[bestWinstreakKey])
      : null;

  return {
    mode: mode.label,
    category: mode.category,
    wins,
    losses,
    kills,
    deaths,
    currentWinstreak: currentWinstreak as number | null,
    bestWinstreak: bestWinstreak as number | null,
  };
}

export function getDuelsCategoryStats(
  stats: Record<string, unknown>,
  category: string
) {
  const categoryModes = Object.entries(DUELS_MODES)
    .filter(([_, mode]) => mode.category === category)
    .map(([key, _]) => key);

  let totalWins = 0;
  let totalLosses = 0;
  let totalKills = 0;
  let totalDeaths = 0;
  let totalCurrentWinstreak = 0;
  let totalBestWinstreak = 0;

  for (const modeKey of categoryModes) {
    const modeStats = getDuelsModeStats(stats, modeKey);
    if (modeStats) {
      totalWins += modeStats.wins;
      totalLosses += modeStats.losses;
      totalKills += modeStats.kills;
      totalDeaths += modeStats.deaths;
      totalCurrentWinstreak += modeStats.currentWinstreak || 0;
      totalBestWinstreak += modeStats.bestWinstreak || 0;
    }
  }

  return {
    category,
    wins: totalWins,
    losses: totalLosses,
    kills: totalKills,
    deaths: totalDeaths,
    currentWinstreak: totalCurrentWinstreak,
    bestWinstreak: totalBestWinstreak,
  };
}

export function getAllDuelsStats(stats: Record<string, unknown>) {
  let totalWins = 0;
  let totalLosses = 0;
  let totalKills = 0;
  let totalDeaths = 0;
  let totalCurrentWinstreak = 0;
  let totalBestWinstreak = 0;

  for (const modeKey of Object.keys(DUELS_MODES)) {
    const modeStats = getDuelsModeStats(stats, modeKey);
    if (modeStats) {
      totalWins += modeStats.wins;
      totalLosses += modeStats.losses;
      totalKills += modeStats.kills;
      totalDeaths += modeStats.deaths;
      totalCurrentWinstreak += modeStats.currentWinstreak || 0;
      totalBestWinstreak += modeStats.bestWinstreak || 0;
    }
  }

  return {
    category: "Overall",
    wins: totalWins,
    losses: totalLosses,
    kills: totalKills,
    deaths: totalDeaths,
    currentWinstreak: totalCurrentWinstreak,
    bestWinstreak: totalBestWinstreak,
  };
}
