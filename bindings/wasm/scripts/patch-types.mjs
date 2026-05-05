import { readFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'

const dtsPath = resolve(process.cwd(), 'pkg', 'ootd_wasm.d.ts')
const source = readFileSync(dtsPath, 'utf-8')

let updated = source

if (!updated.includes('export type Locale = "en" | "ko";')) {
  updated = `export type Locale = "en" | "ko";\n${updated}`
}
if (!updated.includes('export type DurationRange = { start: number; end: number };')) {
  updated = `export type DurationRange = { start: number; end: number };\n${updated}`
}
if (
  !updated.includes(
    'export type TimestampRange = { start: string; end: string };'
  )
) {
  updated = `export type TimestampRange = { start: string; end: string };\n${updated}`
}

updated = updated
  .replace(
    /locale\?: string \| undefined/g,
    'locale?: Locale | undefined'
  )
  .replace(
    /locale\?: string \| null/g,
    'locale?: Locale | null'
  )
  .replace(
    /locale\?: string/g,
    'locale?: Locale'
  )
  .replace(
    /export function rangeOf\(([^)]*)\): any;/g,
    'export function rangeOf($1): DurationRange;'
  )
  .replace(
    /export function resolveDurationRange\(([^)]*)\): any;/g,
    'export function resolveDurationRange($1): TimestampRange;'
  )
  .replace(
    /export function rangeOfTimestamps\(([^)]*)\): any;/g,
    'export function rangeOfTimestamps($1): TimestampRange;'
  )

if (updated !== source) {
  writeFileSync(dtsPath, updated)
}
