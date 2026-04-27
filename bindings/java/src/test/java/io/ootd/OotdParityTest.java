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
            OotdLocale locale = "ko".equals(c.locale()) ? OotdLocale.KO : OotdLocale.EN;
            String out = Ootd.between(c.start(), c.end(), locale, c.useNativeKoNumber());
            assertEquals(c.expected(), out, "between parity mismatch: " + c.name());
        }
    }

    @Test
    void parityDurationCases() {
        ParityFixture fixture = ParityFixture.load();
        for (ParityFixture.DurationCase c : fixture.durationCases) {
            OotdLocale locale = "ko".equals(c.locale()) ? OotdLocale.KO : OotdLocale.EN;

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
}
