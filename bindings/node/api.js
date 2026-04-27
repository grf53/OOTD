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

module.exports = {
  between,
  fromDuration,
}
