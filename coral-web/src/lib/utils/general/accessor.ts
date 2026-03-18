export function getNumber(stats: Record<string, unknown>, key: string): number {
  const n = Number((stats as any)[key]);
  return isNaN(n) ? 0 : n;
}

export function getString(stats: Record<string, unknown>, key: string): string {
  const val = (stats as any)[key];
  if (val === undefined || val === null) return "";
  return String(val);
}

export function getOptionalNumber(
  stats: Record<string, unknown>,
  key: string
): number | null {
  const val = (stats as any)[key];
  if (val === undefined || val === null) return null;
  const n = Number(val);
  return isNaN(n) ? null : n;
}

export function getOptionalString(
  stats: Record<string, unknown>,
  key: string
): string | null {
  const val = (stats as any)[key];
  if (val === undefined || val === null) return null;
  return String(val);
}

export function sumNumbers(
  stats: Record<string, unknown>,
  ...keys: string[]
): number {
  let total = 0;
  for (const key of keys) {
    const n = Number((stats as any)[key] ?? 0);
    if (!isNaN(n)) total += n;
  }
  return total;
}
