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
        String out = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", OotdLocale.EN);
        assertEquals("a month and a half ago", out);
    }

    @Test
    void rendersEarlierTonightForPastSameNight() {
        String out = Ootd.between("2024-01-25T20:30:00Z", "2024-01-25T23:30:00Z", OotdLocale.EN);
        assertEquals("earlier tonight", out);
    }

    @Test
    void rendersNativeKoreanNumberWhenEnabled() {
        String out = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", OotdLocale.KO, true);
        assertEquals("한 달 반 전", out);
    }

    @Test
    void rejectsNegativeDuration() {
        assertThrows(IllegalArgumentException.class, () -> Ootd.fromDuration(-1, false, OotdLocale.EN));
    }

    @Test
    void acceptsOffsetDateTimeInputs() {
        OffsetDateTime start = OffsetDateTime.parse("2024-01-25T01:30:00+09:00");
        OffsetDateTime end = OffsetDateTime.parse("2024-01-25T13:00:00Z");

        String expected = Ootd.between("2024-01-25T01:30:00+09:00", "2024-01-25T13:00:00Z", OotdLocale.EN);
        String out = Ootd.between(start, end, OotdLocale.EN);
        assertEquals(expected, out);
    }

    @Test
    void acceptsZonedDateTimeInputs() {
        ZonedDateTime start = ZonedDateTime.of(2024, 1, 25, 1, 30, 0, 0, ZoneId.of("Asia/Seoul"));
        ZonedDateTime end = ZonedDateTime.of(2024, 1, 25, 13, 0, 0, 0, ZoneId.of("UTC"));

        String expected = Ootd.between("2024-01-25T01:30:00+09:00", "2024-01-25T13:00:00Z", OotdLocale.EN);
        String out = Ootd.between(start, end, OotdLocale.EN);
        assertEquals(expected, out);
    }

    @Test
    void acceptsDurationInput() {
        String expected = Ootd.fromDuration(90 * 60, false, OotdLocale.EN);
        String out = Ootd.fromDuration(Duration.ofMinutes(90), false, OotdLocale.EN);
        assertEquals(expected, out);
    }

    @Test
    void rejectsNegativeDurationObject() {
        assertThrows(IllegalArgumentException.class, () ->
                Ootd.fromDuration(Duration.ofSeconds(-1), false, OotdLocale.EN)
        );
    }
}
