import { truncate } from "@/lib/utils/general/format";

export function getNetworkExp(networkLevel = 1): number {
  return (Math.pow((networkLevel + 2.5) * 50, 2) - 30625) / 2;
}

export function getNetworkLevel(networkExp = 0): number {
  if (!networkExp) return 1;
  const lvl = Math.sqrt(networkExp * 2 + 30625) / 50 - 2.5;
  return truncate(lvl, 2);
}
