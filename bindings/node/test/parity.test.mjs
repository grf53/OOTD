import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { between, fromDuration } from '../api.js'

const dts = readFileSync(resolve(process.cwd(), 'api.d.ts'), 'utf-8')
if (!dts.includes("export type Locale = 'en' | 'ko'")) {
  throw new Error('Node type declaration must expose Locale as "en" | "ko"')
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
