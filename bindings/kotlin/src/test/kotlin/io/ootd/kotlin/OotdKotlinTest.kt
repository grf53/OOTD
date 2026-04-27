package io.ootd.kotlin

import io.ootd.OotdLocale
import java.time.Duration
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class OotdKotlinTest {
    @Test
    fun rendersKnownPhrase() {
        val out = OotdKotlin.between(
            "2023-12-09T18:21:29Z",
            "2024-01-25T13:31:43Z",
            OotdLocale.EN,
        )
        assertEquals("a month and a half ago", out)
    }

    @Test
    fun rendersNativeKoreanNumberWhenEnabled() {
        val out = OotdKotlin.between(
            "2023-12-09T18:21:29Z",
            "2024-01-25T13:31:43Z",
            OotdLocale.KO,
            true,
        )
        assertEquals("한 달 반 전", out)
    }

    @Test
    fun acceptsDurationInput() {
        val out = OotdKotlin.fromDuration(Duration.ofMinutes(90), false, OotdLocale.EN)
        assertEquals("an hour and a half ago", out)
    }

    @Test
    fun rejectsNegativeDuration() {
        assertFailsWith<IllegalArgumentException> {
            OotdKotlin.fromDuration(-1, false, OotdLocale.EN)
        }
    }
}
