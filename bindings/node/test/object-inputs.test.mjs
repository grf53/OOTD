import { between, fromDuration, rangeOf, TimestampRange } from '../api.js'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

function assertThrows(fn, pattern) {
  let threw = false
  try {
    fn()
  } catch (error) {
    threw = pattern.test(String(error?.message ?? error))
  }
  if (!threw) {
    throw new Error(`expected error matching ${pattern}`)
  }
}

const start = '2024-01-25T01:30:00+09:00'
const end = '2024-01-25T13:00:00Z'

const dts = readFileSync(resolve(process.cwd(), 'api.d.ts'), 'utf-8')
if (!dts.includes('export type DateLike = string | Date | { toISOString(): string }')) {
  throw new Error('Node type declaration must expose DateLike')
}
if (!dts.includes('export type DurationLike =')) {
  throw new Error('Node type declaration must expose DurationLike')
}
if (!dts.includes('export declare class TimestampRange')) {
  throw new Error('Node type declaration must expose TimestampRange class')
}
if (!dts.includes('export declare function rangeOf(')) {
  throw new Error('Node type declaration must expose rangeOf')
}
if (!dts.includes('export declare class DurationRange')) {
  throw new Error('Node type declaration must expose DurationRange class')
}

const expectedBetween = between(start, end, 'en')

const fromDateObjects = between(new Date(start), new Date(end), 'en')
if (fromDateObjects !== expectedBetween) {
  throw new Error(`between(Date, Date) mismatch: ${fromDateObjects} != ${expectedBetween}`)
}

const fromIsoProviders = between(
  { toISOString: () => start },
  { toISOString: () => end },
  'en'
)
if (fromIsoProviders !== expectedBetween) {
  throw new Error(`between(toISOString providers) mismatch: ${fromIsoProviders} != ${expectedBetween}`)
}

const expectedDuration = fromDuration(3599, false, 'en')

const fromFloat = fromDuration(3599.9, false, 'en')
if (fromFloat !== expectedDuration) {
  throw new Error(`fromDuration(float) mismatch: ${fromFloat} != ${expectedDuration}`)
}

const fromBigInt = fromDuration(3599n, false, 'en')
if (fromBigInt !== expectedDuration) {
  throw new Error(`fromDuration(bigint) mismatch: ${fromBigInt} != ${expectedDuration}`)
}

const fromTemporalLike = fromDuration(
  { total: ({ unit }) => (unit === 'seconds' ? 3599.9 : 0) },
  false,
  'en'
)
if (fromTemporalLike !== expectedDuration) {
  throw new Error(`fromDuration(total) mismatch: ${fromTemporalLike} != ${expectedDuration}`)
}

const fromAsSeconds = fromDuration({ asSeconds: () => 3599.9 }, false, 'en')
if (fromAsSeconds !== expectedDuration) {
  throw new Error(`fromDuration(asSeconds) mismatch: ${fromAsSeconds} != ${expectedDuration}`)
}

const fromToMillis = fromDuration({ toMillis: () => 3_599_900 }, false, 'en')
if (fromToMillis !== expectedDuration) {
  throw new Error(`fromDuration(toMillis) mismatch: ${fromToMillis} != ${expectedDuration}`)
}

const range = rangeOf('두 달 전', 'ko')
if (range.start !== -6_047_999 || range.end !== -4_752_000) {
  throw new Error(`rangeOf mismatch: ${JSON.stringify(range)}`)
}

const timestampRange = range.resolveAt('2026-04-29T12:00:00+09:00')
if (!(timestampRange instanceof TimestampRange)) {
  throw new Error('range.resolveAt must return TimestampRange class instance')
}
if (
  timestampRange.start.toISOString() !== '2026-02-18T03:00:01.000Z' ||
  timestampRange.end.toISOString() !== '2026-03-05T03:00:00.000Z'
) {
  throw new Error(`range.resolveAt mismatch: ${JSON.stringify(timestampRange)}`)
}

const daypartRange = rangeOf('어제 밤', 'ko').resolveAt('2024-01-25T23:30:00+09:00')
if (!(daypartRange instanceof TimestampRange)) {
  throw new Error('range.resolveAt(daypart) must return TimestampRange class instance')
}
if (
  daypartRange.start.toISOString() !== '2024-01-24T11:00:00.000Z' ||
  daypartRange.end.toISOString() !== '2024-01-24T14:59:59.000Z'
) {
  throw new Error(`range.resolveAt(daypart) mismatch: ${JSON.stringify(daypartRange)}`)
}

assertThrows(
  () => between(new Date('not-a-date'), new Date(end), 'en'),
  /valid Date/
)

assertThrows(
  () => fromDuration({}, false, 'en'),
  /duration-like object/
)

assertThrows(
  () => fromDuration(Number.POSITIVE_INFINITY, false, 'en'),
  /finite number/
)

assertThrows(
  () => rangeOf(123, 'ko'),
  /expression must be a string/
)

assertThrows(
  () => range.resolveAt(123),
  /anchorRfc3339 must be an RFC3339 string/
)
