package io.ootd;

import org.junit.jupiter.api.Test;

import java.time.Duration;
import java.time.OffsetDateTime;
import java.time.ZoneId;
import java.time.ZonedDateTime;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

class OotdTest {
    @Test
    void rendersKnownPhrase() {
        String out = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", Locale.EN);
        assertEquals("a month and a half ago", out);
    }

    @Test
    void rendersEarlierTonightForPastSameNight() {
        String out = Ootd.between("2024-01-25T20:30:00Z", "2024-01-25T23:30:00Z", Locale.EN);
        assertEquals("earlier tonight", out);
    }

    @Test
    void rendersNativeKoreanNumberWhenEnabled() {
        String out = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", Locale.KO, true);
        assertEquals("한 달 반 전", out);
    }

    @Test
    void rejectsNegativeDuration() {
        assertThrows(IllegalArgumentException.class, () -> Ootd.fromDuration(-1, false, Locale.EN));
    }

    @Test
    void acceptsOffsetDateTimeInputs() {
        OffsetDateTime start = OffsetDateTime.parse("2024-01-25T01:30:00+09:00");
        OffsetDateTime end = OffsetDateTime.parse("2024-01-25T13:00:00Z");

        String expected = Ootd.between("2024-01-25T01:30:00+09:00", "2024-01-25T13:00:00Z", Locale.EN);
        String out = Ootd.between(start, end, Locale.EN);
        assertEquals(expected, out);
    }

    @Test
    void acceptsZonedDateTimeInputs() {
        ZonedDateTime start = ZonedDateTime.of(2024, 1, 25, 1, 30, 0, 0, ZoneId.of("Asia/Seoul"));
        ZonedDateTime end = ZonedDateTime.of(2024, 1, 25, 13, 0, 0, 0, ZoneId.of("UTC"));

        String expected = Ootd.between("2024-01-25T01:30:00+09:00", "2024-01-25T13:00:00Z", Locale.EN);
        String out = Ootd.between(start, end, Locale.EN);
        assertEquals(expected, out);
    }

    @Test
    void acceptsDurationInput() {
        String expected = Ootd.fromDuration(90 * 60, false, Locale.EN);
        String out = Ootd.fromDuration(Duration.ofMinutes(90), false, Locale.EN);
        assertEquals(expected, out);
    }

    @Test
    void rejectsNegativeDurationObject() {
        assertThrows(IllegalArgumentException.class, () ->
                Ootd.fromDuration(Duration.ofSeconds(-1), false, Locale.EN)
        );
    }

    @Test
    void supportsRangeOf() {
        Ootd.DurationRange range = Ootd.rangeOf("두 달 전", Locale.KO);
        assertEquals(Duration.ofSeconds(-7_775_999L), range.start());
        assertEquals(Duration.ofSeconds(-5_184_000L), range.end());
    }

    @Test
    void durationRangeResolveAt() {
        Ootd.DurationRange range = Ootd.rangeOf("두 달 전", Locale.KO);
        Ootd.TimestampRange resolved = range.resolveAt("2026-04-29T12:00:00+09:00");
        assertEquals(OffsetDateTime.parse("2026-01-29T12:00:01+09:00"), resolved.start());
        assertEquals(OffsetDateTime.parse("2026-02-28T12:00:00+09:00"), resolved.end());
    }

    @Test
    void extractsExpressions() {
        var out = Ootd.extractExpressions("지난 두 달 전 로그랑 어제 낮 결제", Locale.KO);
        assertEquals(2, out.size());
        assertEquals("두 달 전", out.get(0).text());
        assertEquals("어제 낮", out.get(1).text());
    }
}
