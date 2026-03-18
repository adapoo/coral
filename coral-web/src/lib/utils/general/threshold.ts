export type ThresholdItem = { req: number };

export function findThresholdIndex<T extends ThresholdItem>(
  items: T[],
  score = 0
): number {
  return items.findIndex(
    ({ req }, index, arr) =>
      score >= req &&
      ((arr[index + 1] && score < arr[index + 1].req) || !arr[index + 1])
  );
}

export function findThreshold<T extends ThresholdItem>(
  items: T[],
  score = 0
): T {
  return items[findThresholdIndex(items, score)];
}

export function findThresholdLinear<T extends ThresholdItem>(
  items: T[],
  score: number
): T {
  let best = items[0];
  for (const item of items) {
    if (score >= item.req) best = item;
    else break;
  }
  return best;
}
