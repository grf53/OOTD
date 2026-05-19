const native = require('./index.js')

const MIN_SAFE_BIGINT = BigInt(Number.MIN_SAFE_INTEGER)
const MAX_SAFE_BIGINT = BigInt(Number.MAX_SAFE_INTEGER)

function toRfc3339(value, fieldName) {
  if (typeof value === 'string') {
    return value
  }

  if (value instanceof Date) {
    if (Number.isNaN(value.getTime())) {
      throw new TypeError(`${fieldName} must be a valid Date`)
    }
    return value.toISOString()
  }

  if (value && typeof value.toISOString === 'function') {
    const iso = value.toISOString()
    if (typeof iso === 'string') {
      return iso
    }
  }

  throw new TypeError(
    `${fieldName} must be an RFC3339 string, Date, or object with toISOString()`
  )
}

function toSafeIntegerSeconds(value, sourceName) {
  if (!Number.isFinite(value)) {
    throw new TypeError(`${sourceName} must be a finite number`)
  }

  const seconds = Math.trunc(value)
  if (!Number.isSafeInteger(seconds)) {
    throw new TypeError(`${sourceName} must be within JS safe integer range`)
  }

  return seconds
}

function toSeconds(value) {
  if (typeof value === 'number') {
    return toSafeIntegerSeconds(value, 'seconds')
  }

  if (typeof value === 'bigint') {
    if (value < MIN_SAFE_BIGINT || value > MAX_SAFE_BIGINT) {
      throw new TypeError('seconds bigint must be within JS safe integer range')
    }
    return Number(value)
  }

  if (value && typeof value === 'object') {
    if (typeof value.total === 'function') {
      return toSafeIntegerSeconds(value.total({ unit: 'seconds' }), 'duration.total({ unit: "seconds" })')
    }

    if (typeof value.asSeconds === 'function') {
      return toSafeIntegerSeconds(value.asSeconds(), 'duration.asSeconds()')
    }

    if (typeof value.toMillis === 'function') {
      return toSafeIntegerSeconds(value.toMillis() / 1000, 'duration.toMillis()')
    }
  }

  throw new TypeError(
    'seconds must be a number, bigint, or duration-like object (total/asSeconds/toMillis)'
  )
}

function between(startRfc3339, endRfc3339, locale = 'en', useNativeKoNumber = false) {
  return native.between(
    toRfc3339(startRfc3339, 'startRfc3339'),
    toRfc3339(endRfc3339, 'endRfc3339'),
    locale,
    useNativeKoNumber
  )
}

function fromDuration(seconds, isFuture = false, locale = 'en', useNativeKoNumber = false) {
  return native.fromDuration(toSeconds(seconds), isFuture, locale, useNativeKoNumber)
}

function toTimestampRange(raw, sourceName) {
  const start = raw?.start ?? raw?.start_rfc3339 ?? raw?.startRfc3339
  const end = raw?.end ?? raw?.end_rfc3339 ?? raw?.endRfc3339

  if (typeof start !== 'string' || typeof end !== 'string') {
    throw new TypeError(`${sourceName} returned invalid timestamp fields`)
  }

  return new TimestampRange(new Date(start), new Date(end), raw)
}

class TimestampRange {
  constructor(start, end, raw = null) {
    if (!(start instanceof Date) || Number.isNaN(start.getTime())) {
      throw new TypeError('start must be a valid Date')
    }
    if (!(end instanceof Date) || Number.isNaN(end.getTime())) {
      throw new TypeError('end must be a valid Date')
    }
    this.start = start
    this.end = end
    this._raw = raw ?? {
      start: start.toISOString(),
      end: end.toISOString(),
    }
  }

  toJSON() {
    return {
      start: this.start,
      end: this.end,
    }
  }
}

class DurationRange {
  constructor(startSeconds, endSeconds, expression = null, locale = 'en') {
    this.start = startSeconds
    this.end = endSeconds
    this._expression = expression
    this._locale = locale
  }

  resolveAt(anchorRfc3339 = undefined) {
    const anchor =
      anchorRfc3339 == null ? undefined : toRfc3339(anchorRfc3339, 'anchorRfc3339')
    if (typeof this._expression === 'string') {
      const raw = native.rangeOfTimestamps(this._expression, this._locale, anchor)
      return toTimestampRange(raw, 'native rangeOfTimestamps')
    }
    const raw = native.resolveDurationRange(this.start, this.end, anchor)
    return toTimestampRange(raw, 'native resolveDurationRange')
  }

  toJSON() {
    return {
      start: this.start,
      end: this.end,
    }
  }
}

function rangeOf(expression, locale = 'en') {
  if (typeof expression !== 'string') {
    throw new TypeError('expression must be a string')
  }

  const raw = native.rangeOf(expression, locale)
  const start = raw?.start ?? raw?.start_seconds ?? raw?.startSeconds
  const end = raw?.end ?? raw?.end_seconds ?? raw?.endSeconds

  if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end)) {
    throw new TypeError('native rangeOf returned invalid range object')
  }

  return new DurationRange(start, end, expression, locale)
}

function extractExpressions(input, locale = 'en') {
  if (typeof input !== 'string') {
    throw new TypeError('input must be a string')
  }

  const raw = native.extractExpressions(input, locale)
  if (!Array.isArray(raw)) {
    throw new TypeError('native extractExpressions returned invalid value')
  }

  return raw.map((it) => {
    if (
      typeof it?.start !== 'number' ||
      typeof it?.end !== 'number' ||
      typeof it?.text !== 'string'
    ) {
      throw new TypeError('native extractExpressions returned invalid candidate')
    }
    return {
      start: it.start,
      end: it.end,
      text: it.text,
    }
  })
}

module.exports = {
  between,
  fromDuration,
  rangeOf,
  extractExpressions,
  DurationRange,
  TimestampRange,
}
