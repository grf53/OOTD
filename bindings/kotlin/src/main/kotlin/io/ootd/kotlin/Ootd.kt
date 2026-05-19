package io.ootd.kotlin

import io.ootd.Ootd as JavaOotd
import io.ootd.Locale
import java.time.Duration
import java.time.OffsetDateTime
import java.time.ZonedDateTime

object Ootd {
    @JvmStatic
    @JvmOverloads
    fun between(
        startRfc3339: String,
        endRfc3339: String,
        locale: Locale = Locale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = JavaOotd.between(startRfc3339, endRfc3339, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun between(
        start: OffsetDateTime,
        end: OffsetDateTime,
        locale: Locale = Locale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = JavaOotd.between(start, end, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun between(
        start: ZonedDateTime,
        end: ZonedDateTime,
        locale: Locale = Locale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = JavaOotd.between(start, end, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun fromDuration(
        seconds: Long,
        isFuture: Boolean = false,
        locale: Locale = Locale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = JavaOotd.fromDuration(seconds, isFuture, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun fromDuration(
        duration: Duration,
        isFuture: Boolean = false,
        locale: Locale = Locale.EN,
        useNativeKoNumber: Boolean = false,
    ): String = JavaOotd.fromDuration(duration, isFuture, locale, useNativeKoNumber)

    @JvmStatic
    @JvmOverloads
    fun rangeOf(
        expression: String,
        locale: Locale = Locale.EN,
    ): JavaOotd.DurationRange = JavaOotd.rangeOf(expression, locale)

    @JvmStatic
    @JvmOverloads
    fun extractExpressions(
        input: String,
        locale: Locale = Locale.EN,
    ): List<JavaOotd.ExpressionCandidate> = JavaOotd.extractExpressions(input, locale)
}
