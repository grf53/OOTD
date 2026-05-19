import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import {
  between,
  DurationRange,
  extractExpressions,
  fromDuration,
  rangeOf,
  TimestampRange,
} from '../api.js'

const dts = readFileSync(resolve(process.cwd(), 'api.d.ts'), 'utf-8')
if (!dts.includes("export type Locale = 'en' | 'ko'")) {
  throw new Error("WASM type declaration must expose Locale as 'en' | 'ko'")
}
if (!dts.includes('export declare class DurationRange')) {
  throw new Error('WASM type declaration must expose DurationRange class')
}
if (!dts.includes('export declare class TimestampRange')) {
  throw new Error('WASM type declaration must expose TimestampRange class')
}
if (!dts.includes('export declare function extractExpressions(')) {
  throw new Error('WASM type declaration must expose extractExpressions')
}

const fixture = JSON.parse(
  readFileSync(resolve(process.cwd(), '../../tests/parity_cases.json'), 'utf-8')
)

for (const c of fixture.between_cases) {
  const out = between(c.start, c.end, c.locale, c.use_native_ko_number ?? false)
  if (out !== c.expected) {
    throw new Error(`Between case ${c.name} failed: ${out} != ${c.expected}`)
  }
}

for (const c of fixture.duration_cases) {
  if (c.expected_error) {
    let failed = false
    try {
      fromDuration(c.seconds, c.is_future, c.locale, c.use_native_ko_number ?? false)
    } catch (e) {
      failed = String(e?.message ?? e).includes(c.expected_error)
    }

    if (!failed) {
      throw new Error(`Duration error case ${c.name} failed`)
    }
    continue
  }

  const out = fromDuration(c.seconds, c.is_future, c.locale, c.use_native_ko_number ?? false)
  if (out !== c.expected) {
    throw new Error(`Duration case ${c.name} failed: ${out} != ${c.expected}`)
  }
}

const range = rangeOf('두 달 전', 'ko')
if (!(range instanceof DurationRange)) {
  throw new Error('rangeOf must return DurationRange class instance')
}
if (range.start !== -7_775_999 || range.end !== -5_184_000) {
  throw new Error(`rangeOf case failed: ${JSON.stringify(range)}`)
}

const tsRange = range.resolveAt('2026-04-29T12:00:00+09:00')
if (!(tsRange instanceof TimestampRange)) {
  throw new Error('range.resolveAt must return TimestampRange class instance')
}
if (
  tsRange.start.toISOString() !== '2026-01-29T03:00:01.000Z' ||
  tsRange.end.toISOString() !== '2026-02-28T03:00:00.000Z'
) {
  throw new Error(`range.resolveAt case failed: ${JSON.stringify(tsRange)}`)
}

const daypartTsRange = rangeOf('어제 밤', 'ko').resolveAt('2024-01-25T23:30:00+09:00')
if (!(daypartTsRange instanceof TimestampRange)) {
  throw new Error('range.resolveAt(daypart) must return TimestampRange class instance')
}
if (
  daypartTsRange.start.toISOString() !== '2024-01-24T11:00:00.000Z' ||
  daypartTsRange.end.toISOString() !== '2024-01-24T14:59:59.000Z'
) {
  throw new Error(`range.resolveAt(daypart) case failed: ${JSON.stringify(daypartTsRange)}`)
}

const extracted = extractExpressions('지난 두 달 전 로그랑 어제 낮 결제', 'ko')
if (
  extracted.length !== 2 ||
  extracted[0].text !== '두 달 전' ||
  extracted[1].text !== '어제 낮'
) {
  throw new Error(`extractExpressions mismatch: ${JSON.stringify(extracted)}`)
}
