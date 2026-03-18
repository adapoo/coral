import { colorCodeToVar } from "@/lib/utils/general/colors";
import { formatInt, getNumber } from "@/lib/utils/general";
import {
  DUELS_MODES,
  DUELS_CATEGORY_KEYS,
  getDuelsModeStats,
  getDuelsDivision,
} from "./index";

export type DuelsModeStats = NonNullable<ReturnType<typeof getDuelsModeStats>>;

export function buildModesByCategory(stats: Record<string, unknown>) {
  const modesByCategory: Record<string, DuelsModeStats[]> = {};
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    const modeStats = getDuelsModeStats(stats, modeKey);
    if (
      modeStats &&
      (modeStats.wins > 0 ||
        modeStats.losses > 0 ||
        modeStats.kills > 0 ||
        modeStats.deaths > 0)
    ) {
      const category = modeStats.category;
      if (!modesByCategory[category]) {
        modesByCategory[category] = [];
      }
      modesByCategory[category].push(modeStats);
    }
  });
  return modesByCategory;
}

export function renderDivision(wins: number, compact = true) {
  const { color, numeral, title } = getDuelsDivision(wins, compact);
  const cssColor = colorCodeToVar(color);
  const displayText = compact ? numeral : `${title} ${numeral}`.trim();
  return <span style={{ color: cssColor }}>{displayText || "-"}</span>;
}

export function formatWinstreak(
  winstreak: number | null | undefined,
  isHidden = false,
) {
  if (isHidden || winstreak === null || winstreak === undefined) {
    return <span style={{ color: "var(--color-c)" }}>?</span>;
    }
  return formatInt(winstreak);
}

export function getMeleeHitRate(
  stats: Record<string, unknown>,
  modeKey: string,
) {
  const hits = Number(stats[`${modeKey}_melee_hits`] || 0);
  const swings = Number(stats[`${modeKey}_melee_swings`] || 0);
  if (swings === 0) return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((hits / swings) * 100).toFixed(1) + "%";
}

export function getArrowHitRate(
  stats: Record<string, unknown>,
  modeKey: string,
) {
  const hits = Number(stats[`${modeKey}_bow_hits`] || 0);
  const shots = Number(stats[`${modeKey}_bow_shots`] || 0);
  if (shots === 0) return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((hits / shots) * 100).toFixed(1) + "%";
}

export function getGoals(stats: Record<string, unknown>, modeKey: string) {
  const goals = Number(stats[`${modeKey}_goals`] || 0);
  return goals > 0 ? (
    formatInt(goals)
  ) : (
    <span style={{ color: "var(--color-7)" }}>-</span>
  );
}

export function getBridgeKills(stats: Record<string, unknown>, modeKey: string) {
  if (modeKey.startsWith("bridge_")) {
    return Number(stats[`${modeKey}_bridge_kills`] || 0);
  }
  return Number(stats[`${modeKey}_kills`] || 0);
}

export function getBridgeDeaths(
  stats: Record<string, unknown>,
  modeKey: string,
) {
  if (modeKey.startsWith("bridge_")) {
    return Number(stats[`${modeKey}_bridge_deaths`] || 0);
  }
  return Number(stats[`${modeKey}_deaths`] || 0);
}

export function getOverallMeleeHitRate(stats: Record<string, unknown>) {
  let totalHits = 0;
  let totalSwings = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalHits += Number(stats[`${modeKey}_melee_hits`] || 0);
    totalSwings += Number(stats[`${modeKey}_melee_swings`] || 0);
  });
  if (totalSwings === 0)
    return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((totalHits / totalSwings) * 100).toFixed(1) + "%";
}

export function getOverallArrowHitRate(stats: Record<string, unknown>) {
  let totalHits = 0;
  let totalShots = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalHits += Number(stats[`${modeKey}_bow_hits`] || 0);
    totalShots += Number(stats[`${modeKey}_bow_shots`] || 0);
  });
  if (totalShots === 0)
    return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((totalHits / totalShots) * 100).toFixed(1) + "%";
}

export function getOverallMeleeSwings(stats: Record<string, unknown>) {
  let totalSwings = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalSwings += Number(stats[`${modeKey}_melee_swings`] || 0);
  });
  return totalSwings;
}

export function getOverallMeleeHits(stats: Record<string, unknown>) {
  let totalHits = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalHits += Number(stats[`${modeKey}_melee_hits`] || 0);
  });
  return totalHits;
}

export function getOverallArrowShots(stats: Record<string, unknown>) {
  let totalShots = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalShots += Number(stats[`${modeKey}_bow_shots`] || 0);
  });
  return totalShots;
}

export function getOverallArrowHits(stats: Record<string, unknown>) {
  let totalHits = 0;
  Object.keys(DUELS_MODES).forEach((modeKey) => {
    totalHits += Number(stats[`${modeKey}_bow_hits`] || 0);
  });
  return totalHits;
}

export function getCategoryMeleeHitRate(
  stats: Record<string, unknown>,
  modes: DuelsModeStats[],
) {
  let totalHits = 0;
  let totalSwings = 0;
  modes.forEach((mode) => {
    const modeKey =
      Object.keys(DUELS_MODES).find(
        (key) => DUELS_MODES[key as keyof typeof DUELS_MODES].label === mode.mode,
      ) || "";
    totalHits += Number(stats[`${modeKey}_melee_hits`] || 0);
    totalSwings += Number(stats[`${modeKey}_melee_swings`] || 0);
  });
  if (totalSwings === 0)
    return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((totalHits / totalSwings) * 100).toFixed(1) + "%";
}

export function getCategoryArrowHitRate(
  stats: Record<string, unknown>,
  modes: DuelsModeStats[],
) {
  let totalHits = 0;
  let totalShots = 0;
  modes.forEach((mode) => {
    const modeKey =
      Object.keys(DUELS_MODES).find(
        (key) => DUELS_MODES[key as keyof typeof DUELS_MODES].label === mode.mode,
      ) || "";
    totalHits += Number(stats[`${modeKey}_bow_hits`] || 0);
    totalShots += Number(stats[`${modeKey}_bow_shots`] || 0);
  });
  if (totalShots === 0)
    return <span style={{ color: "var(--color-7)" }}>-</span>;
  return ((totalHits / totalShots) * 100).toFixed(1) + "%";
}

export function getCategoryWinstreak(
  stats: Record<string, unknown>,
  category: string,
  type: "current" | "best",
) {
  const categoryKey = DUELS_CATEGORY_KEYS[category];
  if (!categoryKey) {
    return <span style={{ color: "var(--color-c)" }}>?</span>;
  }
  const winstreakKey = `${type}_${categoryKey}_winstreak`;
  const winstreak = stats[winstreakKey];
  if (winstreak === undefined || winstreak === null) {
    return <span style={{ color: "var(--color-c)" }}>?</span>;
  }
  const numWinstreak = Number(winstreak);
  if (numWinstreak === 0) {
    return <span style={{ color: "var(--color-7)" }}>-</span>;
  }
  return formatInt(numWinstreak);
}

export function getTokens(stats: Record<string, unknown>) {
  return getNumber(stats, "coins");
}

