export type Locale = 'en' | 'ko'
export type DateLike = string | Date | { toISOString(): string }
export type DurationLike =
  | number
  | bigint
  | { total(options: { unit: 'seconds' }): number }
  | { asSeconds(): number }
  | { toMillis(): number }
export declare class TimestampRange {
  constructor(start: Date, end: Date)
  start: Date
  end: Date
}

export declare class DurationRange {
  constructor(start: number, end: number)
  start: number
  end: number
  resolveAt(anchorRfc3339?: DateLike): TimestampRange
}
export type ExpressionCandidate = {
  start: number
  end: number
  text: string
}

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

export declare function rangeOf(
  expression: string,
  locale?: Locale
): DurationRange

export declare function extractExpressions(
  input: string,
  locale?: Locale
): ExpressionCandidate[]
