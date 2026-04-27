export type Locale = 'en' | 'ko'
export type DateLike = string | Date | { toISOString(): string }
export type DurationLike =
  | number
  | bigint
  | { total(options: { unit: 'seconds' }): number }
  | { asSeconds(): number }
  | { toMillis(): number }

export declare function between(
  startRfc3339: DateLike,
  endRfc3339: DateLike,
  locale?: Locale,
  useNativeKoNumber?: boolean
): string

export declare function fromDuration(
  seconds: DurationLike,
  isFuture?: boolean,
  locale?: Locale,
  useNativeKoNumber?: boolean
): string
