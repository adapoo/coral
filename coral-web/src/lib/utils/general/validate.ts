const UUID_LIKE_REGEX =
  /^(?:[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12})$/;

export function isUuidLike(value: string | null | undefined): boolean {
  if (!value) return false;
  return UUID_LIKE_REGEX.test(value);
}
