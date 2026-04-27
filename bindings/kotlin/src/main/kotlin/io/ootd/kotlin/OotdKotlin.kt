package io.ootd.kotlin

import io.ootd.Ootd
import io.ootd.OotdLocale
import java.time.Duration
import java.time.OffsetDateTime
import java.time.ZonedDateTime

object OotdKotlin {
    @JvmStatic
    @JvmOverloads
    fun between(
        startRfc3339: String,
        endRfc3339: String,
        locale: OotdLocale = OotdLocale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = Ootd.between(startRfc3339, endRfc3339, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun between(
        start: OffsetDateTime,
        end: OffsetDateTime,
        locale: OotdLocale = OotdLocale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = Ootd.between(start, end, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun between(
        start: ZonedDateTime,
        end: ZonedDateTime,
        locale: OotdLocale = OotdLocale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = Ootd.between(start, end, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun fromDuration(
        seconds: Long,
        isFuture: Boolean = false,
        locale: OotdLocale = OotdLocale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = Ootd.fromDuration(seconds, isFuture, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun fromDuration(
        duration: Duration,
        isFuture: Boolean = false,
        locale: OotdLocale = OotdLocale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = Ootd.fromDuration(duration, isFuture, locale, useNativeKoNumber)
}
