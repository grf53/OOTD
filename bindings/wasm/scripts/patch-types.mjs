import { readFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'

const dtsPath = resolve(process.cwd(), 'pkg', 'ootd_wasm.d.ts')
const source = readFileSync(dtsPath, 'utf-8')

let updated = source

if (!updated.includes('export type Locale = "en" | "ko";')) {
  updated = `export type Locale = "en" | "ko";\n${updated}`
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

if (updated !== source) {
  writeFileSync(dtsPath, updated)
}
