package io.ootd.kotlin

import io.ootd.Locale
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Path
import java.time.Duration
import java.time.OffsetDateTime
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

class OotdKotlinParityTest {
    @Test
    fun parityBetweenCases() {
        val fixture = ParityFixture.load()
        fixture.betweenCases.forEach { c ->
            val locale = when (c.locale) {
                "ko" -> Locale.KO
                else -> Locale.EN
            }
            val out = Ootd.between(c.start, c.end, locale, c.useNativeKoNumber)
            assertEquals(c.expected, out, "between parity mismatch: ${c.name}")
        }
    }

    @Test
    fun parityDurationCases() {
        val fixture = ParityFixture.load()
        fixture.durationCases.forEach { c ->
            val locale = when (c.locale) {
                "ko" -> Locale.KO
                else -> Locale.EN
            }

            if (c.expectedError != null) {
                val e = assertFailsWith<IllegalArgumentException>("duration error case must fail: ${c.name}") {
                    Ootd.fromDuration(c.seconds, c.isFuture, locale, c.useNativeKoNumber)
                }
                assertTrue(
                    e.message?.contains(c.expectedError) == true,
                    "duration error mismatch: ${c.name}"
                )
                return@forEach
            }

            val out = Ootd.fromDuration(c.seconds, c.isFuture, locale, c.useNativeKoNumber)
            assertEquals(c.expected, out, "duration parity mismatch: ${c.name}")
        }
    }

    @Test
    fun parityRangeCases() {
        val range = Ootd.rangeOf("두 달 전", Locale.KO)
        assertEquals(Duration.ofSeconds(-6_047_999L), range.start(), "range start mismatch")
        assertEquals(Duration.ofSeconds(-4_752_000L), range.end(), "range end mismatch")

        val resolved = range.resolveAt("2026-04-29T12:00:00+09:00")
        assertEquals(OffsetDateTime.parse("2026-02-18T12:00:01+09:00"), resolved.start(), "resolved start mismatch")
        assertEquals(OffsetDateTime.parse("2026-03-05T12:00:00+09:00"), resolved.end(), "resolved end mismatch")
    }
}

private data class ParityFixture(
    val betweenCases: List<BetweenCase>,
    val durationCases: List<DurationCase>,
) {
    data class BetweenCase(
        val name: String,
        val start: String,
        val end: String,
        val locale: String,
        val useNativeKoNumber: Boolean,
        val expected: String,
    )

    data class DurationCase(
        val name: String,
        val seconds: Long,
        val isFuture: Boolean,
        val locale: String,
        val useNativeKoNumber: Boolean,
        val expected: String?,
        val expectedError: String?,
    )

    companion object {
        fun load(): ParityFixture {
            val path = Path.of("..", "..", "tests", "parity_cases.json")
            val json = Files.readString(path, StandardCharsets.UTF_8)
            return parse(json)
        }

        private fun parse(json: String): ParityFixture {
            val betweenArray = findArray(json, "between_cases")
            val durationArray = findArray(json, "duration_cases")

            val between = splitObjects(betweenArray).map { obj ->
                BetweenCase(
                    name = requiredString(obj, "name"),
                    start = requiredString(obj, "start"),
                    end = requiredString(obj, "end"),
                    locale = requiredString(obj, "locale"),
                    useNativeKoNumber = optionalBoolean(obj, "use_native_ko_number", false),
                    expected = requiredString(obj, "expected"),
                )
            }

            val duration = splitObjects(durationArray).map { obj ->
                DurationCase(
                    name = requiredString(obj, "name"),
                    seconds = requiredLong(obj, "seconds"),
                    isFuture = requiredBoolean(obj, "is_future"),
                    locale = requiredString(obj, "locale"),
                    useNativeKoNumber = optionalBoolean(obj, "use_native_ko_number", false),
                    expected = optionalString(obj, "expected"),
                    expectedError = optionalString(obj, "expected_error"),
                )
            }

            return ParityFixture(between, duration)
        }

        private fun findArray(json: String, key: String): String {
            val keyNeedle = "\"$key\""
            val keyPos = json.indexOf(keyNeedle)
            require(keyPos >= 0) { "Missing key in parity fixture: $key" }

            val start = json.indexOf('[', keyPos)
            require(start >= 0) { "Missing array start for key: $key" }

            var depth = 0
            for (i in start until json.length) {
                when (json[i]) {
                    '[' -> depth++
                    ']' -> {
                        depth--
                        if (depth == 0) {
                            return json.substring(start + 1, i)
                        }
                    }
                }
            }

            error("Unclosed array for key: $key")
        }

        private fun splitObjects(arrayContent: String): List<String> {
            val out = mutableListOf<String>()
            var depth = 0
            var start = -1
            for (i in arrayContent.indices) {
                when (arrayContent[i]) {
                    '{' -> {
                        if (depth == 0) {
                            start = i
                        }
                        depth++
                    }
                    '}' -> {
                        depth--
                        if (depth == 0 && start >= 0) {
                            out += arrayContent.substring(start, i + 1)
                            start = -1
                        }
                    }
                }
            }

            return out
        }

        private fun requiredString(obj: String, key: String): String =
            optionalString(obj, key) ?: error("Missing required string key: $key in $obj")

        private fun optionalString(obj: String, key: String): String? {
            val r = Regex("\"${Regex.escape(key)}\"\\s*:\\s*\"([^\"]*)\"")
            return r.find(obj)?.groupValues?.get(1)
        }

        private fun requiredLong(obj: String, key: String): Long {
            val r = Regex("\"${Regex.escape(key)}\"\\s*:\\s*(-?\\d+)")
            val m = r.find(obj) ?: error("Missing required long key: $key in $obj")
            return m.groupValues[1].toLong()
        }

        private fun requiredBoolean(obj: String, key: String): Boolean {
            val r = Regex("\"${Regex.escape(key)}\"\\s*:\\s*(true|false)")
            val m = r.find(obj) ?: error("Missing required boolean key: $key in $obj")
            return m.groupValues[1].toBooleanStrict()
        }

        private fun optionalBoolean(obj: String, key: String, default: Boolean): Boolean {
            val r = Regex("\"${Regex.escape(key)}\"\\s*:\\s*(true|false)")
            val m = r.find(obj) ?: return default
            return m.groupValues[1].toBooleanStrict()
        }
    }
}
