export function ratio(
  numerator: number,
  denominator: number,
  precision?: number
): string {
  const num = Number(numerator) || 0;
  const den = Number(denominator) || 0;
  const ratio = den === 0 ? num : num / den;
  return ratio.toFixed(precision ?? 2);
}

export function duration(totalMinutes: number): string {
  const minutes = Math.max(0, Math.floor(Number(totalMinutes) || 0));
  if (minutes <= 0) return "0m";
  const days = Math.floor(minutes / (60 * 24));
  const hours = Math.floor((minutes % (60 * 24)) / 60);
  const mins = Math.floor(minutes % 60);
  if (days > 0) return `${days}d ${hours}h`;
  if (hours > 0) return `${hours}h ${mins}m`;
  return `${mins}m`;
}

export function relativeTime(
  input?: string | number | Date | null
): string | undefined {
  if (input === undefined || input === null) return undefined;
  const date = input instanceof Date ? input : new Date(input);
  if (isNaN(date.getTime())) return undefined;
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const sec = Math.floor(diffMs / 1000);
  const min = Math.floor(sec / 60);
  const hr = Math.floor(min / 60);
  const day = Math.floor(hr / 24);
  if (day >= 7) return date.toLocaleString();
  if (day >= 1) return `${day} day${day === 1 ? "" : "s"} ago`;
  if (hr >= 1) return `${hr} hour${hr === 1 ? "" : "s"} ago`;
  if (min >= 1) return `${min} minute${min === 1 ? "" : "s"} ago`;
  return `${sec} second${sec === 1 ? "" : "s"} ago`;
}

export function round(num: number, precision?: number): number {
  return (
    Math.round((num + Number.EPSILON) * (precision ?? 100)) / (precision ?? 100)
  );
}

export function truncate(num: number, precision: number = 2): number {
  const factor = Math.pow(10, precision);
  return Math.trunc(num * factor) / factor;
}

export function romanNumeral(n: number): string {
  const map: [number, string][] = [
    [1000, "M"],
    [900, "CM"],
    [500, "D"],
    [400, "CD"],
    [100, "C"],
    [90, "XC"],
    [50, "L"],
    [40, "XL"],
    [10, "X"],
    [9, "IX"],
    [5, "V"],
    [4, "IV"],
    [1, "I"],
  ];
  let out = "";
  for (const [v, sym] of map)
    while (n >= v) {
      out += sym;
      n -= v;
    }
  return out;
}

export function formatCompactWhole(value: number): string {
  if (value >= 1_000_000_000) return `${Math.floor(value / 1_000_000_000)}B`;
  if (value >= 1_000_000) return `${Math.floor(value / 1_000_000)}M`;
  if (value >= 1_000) return `${Math.floor(value / 1_000)}K`;
  return String(value);
}

export function formatInt(value: number): string {
  return Number(value || 0).toLocaleString();
}
