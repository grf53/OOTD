package io.ootd;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

class OotdParityTest {
    @Test
    void parityBetweenCases() {
        ParityFixture fixture = ParityFixture.load();
        for (ParityFixture.BetweenCase c : fixture.betweenCases) {
            Locale locale = "ko".equals(c.locale()) ? Locale.KO : Locale.EN;
            String out = Ootd.between(c.start(), c.end(), locale, c.useNativeKoNumber());
            assertEquals(c.expected(), out, "between parity mismatch: " + c.name());
        }
    }

    @Test
    void parityDurationCases() {
        ParityFixture fixture = ParityFixture.load();
        for (ParityFixture.DurationCase c : fixture.durationCases) {
            Locale locale = "ko".equals(c.locale()) ? Locale.KO : Locale.EN;

            if (c.expectedError() != null) {
                try {
                    Ootd.fromDuration(c.seconds(), c.isFuture(), locale, c.useNativeKoNumber());
                    fail("duration error case must fail: " + c.name());
                } catch (IllegalArgumentException e) {
                    assertTrue(
                            e.getMessage().contains(c.expectedError()),
                            "duration error mismatch: " + c.name()
                    );
                }
                continue;
            }

            String out = Ootd.fromDuration(c.seconds(), c.isFuture(), locale, c.useNativeKoNumber());
            assertEquals(c.expected(), out, "duration parity mismatch: " + c.name());
        }
    }

    @Test
    void parityRangeCases() {
        Ootd.DurationRange range = Ootd.rangeOf("두 달 전", Locale.KO);
        assertEquals(java.time.Duration.ofSeconds(-7_775_999L), range.start(), "range start mismatch");
        assertEquals(java.time.Duration.ofSeconds(-5_184_000L), range.end(), "range end mismatch");

        Ootd.TimestampRange resolved = range.resolveAt("2026-04-29T12:00:00+09:00");
        assertEquals(java.time.OffsetDateTime.parse("2026-01-29T12:00:01+09:00"), resolved.start(), "resolved start mismatch");
        assertEquals(java.time.OffsetDateTime.parse("2026-02-28T12:00:00+09:00"), resolved.end(), "resolved end mismatch");
    }
}
