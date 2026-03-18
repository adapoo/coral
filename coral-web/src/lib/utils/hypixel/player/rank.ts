import { C } from "@/lib/utils/general/colors";

export const rankMap: Record<string, (plusColorCode: string) => string> = {
  "MVP+": (plus) => `${C.AQUA}[MVP${plus}+${C.AQUA}]`,
  "MVP++": (plus) => `${C.GOLD}[MVP${plus}++${C.GOLD}]`,
  "bMVP++": (plus) => `${C.AQUA}[MVP${plus}++${C.AQUA}]`,
  MVP: () => `${C.AQUA}[MVP]`,
  "VIP+": () => `${C.GREEN}[VIP${C.GOLD}+${C.GREEN}]`,
  VIP: () => `${C.GREEN}[VIP]`,
  YOUTUBE: () => `${C.RED}[${C.WHITE}YOUTUBE${C.RED}]`,
  "PIG+++": () => `${C.LIGHT_PURPLE}[PIG${C.AQUA}+++${C.LIGHT_PURPLE}]`,
  INNIT: () => `${C.LIGHT_PURPLE}[INNIT]`,
  GM: () => `${C.DARK_GREEN}[GM]`,
  ADMIN: () => `${C.RED}[ADMIN]`,
  OWNER: () => `${C.RED}[OWNER]`,
  STAFF: () => `${C.RED}[${C.GOLD}ዞ${C.RED}]`,
  MOJANG: () => `${C.GOLD}[MOJANG]`,
  EVENTS: () => `${C.GOLD}[EVENTS]`,
  DEFAULT: () => C.GRAY,
};

function replaceRank(rank: string): string {
  return rank
    .replace("SUPERSTAR", "MVP++")
    .replace("VIP_PLUS", "VIP+")
    .replace("MVP_PLUS", "MVP+")
    .replace("MODERATOR", "MOD")
    .replace("GAME_MASTER", "GM")
    .replace("YOUTUBER", "YOUTUBE")
    .replace("NONE", "");
}

export function getRank(data: any): string {
  let rank = "DEFAULT";
  const monthlyPackageRank = data?.monthlyPackageRank as string | undefined;
  const packageRank = data?.packageRank as string | undefined;
  const newPackageRank = data?.newPackageRank as string | undefined;
  const monthlyRankColor = data?.monthlyRankColor as string | undefined;

  if (monthlyPackageRank || packageRank || newPackageRank) {
    if (monthlyPackageRank === "SUPERSTAR") {
      rank = monthlyPackageRank;
      if (monthlyRankColor && monthlyRankColor !== "GOLD") rank = "bMVP++";
    } else {
      rank =
        packageRank && newPackageRank
          ? newPackageRank!
          : packageRank || newPackageRank || "DEFAULT";
    }
  }
  const rankOverride = data?.rank as string | undefined;
  if (rankOverride && rankOverride !== "NORMAL") rank = rankOverride;
  const prefix = data?.prefix as string | undefined;
  if (prefix) rank = prefix.replace(/§.|\[|]/g, "");
  rank = replaceRank(rank);
  return rank.length === 0 ? "DEFAULT" : rank;
}

export function getPlusColor(rank: string, plusColorName?: string): string {
  const defaults: Record<string, string> = {
    "MVP+": C.RED,
    "MVP++": C.RED,
    "bMVP++": C.RED,
    MVP: C.AQUA,
    VIP: C.GREEN,
    "VIP+": C.GOLD,
    "PIG+++": C.AQUA,
  };
  if (!plusColorName || rank === "PIG+++" || rank === "VIP")
    return defaults[rank] ?? C.GRAY;
  const key = plusColorName.toUpperCase();
  return C[key as keyof typeof C] ?? C.GRAY;
}

export function getDisplayName(
  username: string,
  rank: string,
  plusColorCode: string
): string {
  const fmt = rankMap[rank] ?? rankMap.DEFAULT;
  const rankStr = fmt(plusColorCode);
  return `${rankStr}${rankStr === C.GRAY ? "" : " "}${username}`;
}
