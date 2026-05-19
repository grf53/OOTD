package io.ootd.kotlin

import io.ootd.Locale
import java.time.Duration
import java.time.OffsetDateTime
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class OotdKotlinTest {
    @Test
    fun rendersKnownPhrase() {
        val out = Ootd.between(
            "2023-12-09T18:21:29Z",
            "2024-01-25T13:31:43Z",
            Locale.EN,
        )
        assertEquals("a month and a half ago", out)
    }

    @Test
    fun rendersNativeKoreanNumberWhenEnabled() {
        val out = Ootd.between(
            "2023-12-09T18:21:29Z",
            "2024-01-25T13:31:43Z",
            Locale.KO,
            true,
        )
        assertEquals("한 달 반 전", out)
    }

    @Test
    fun acceptsDurationInput() {
        val out = Ootd.fromDuration(Duration.ofMinutes(90), false, Locale.EN)
        assertEquals("an hour and a half ago", out)
    }

    @Test
    fun rejectsNegativeDuration() {
        assertFailsWith<IllegalArgumentException> {
            Ootd.fromDuration(-1, false, Locale.EN)
        }
    }

    @Test
    fun supportsRangeOf() {
        val range = Ootd.rangeOf("두 달 전", Locale.KO)
        assertEquals(Duration.ofSeconds(-7_775_999L), range.start())
        assertEquals(Duration.ofSeconds(-5_184_000L), range.end())
    }

    @Test
    fun supportsDurationRangeResolveAt() {
        val range = Ootd.rangeOf("두 달 전", Locale.KO)
        val resolved = range.resolveAt("2026-04-29T12:00:00+09:00")
        assertEquals(OffsetDateTime.parse("2026-01-29T12:00:01+09:00"), resolved.start())
        assertEquals(OffsetDateTime.parse("2026-02-28T12:00:00+09:00"), resolved.end())
    }

    @Test
    fun extractsExpressions() {
        val out = Ootd.extractExpressions("지난 두 달 전 로그랑 어제 낮 결제", Locale.KO)
        assertEquals(2, out.size)
        assertEquals("두 달 전", out[0].text())
        assertEquals("어제 낮", out[1].text())
    }
}
