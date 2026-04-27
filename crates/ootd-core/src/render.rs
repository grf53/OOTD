use crate::locale::{render_duration_en, render_duration_ko};
use crate::types::{Direction, Locale, RenderOptions, UNITS};

pub(crate) fn render_duration_nonnegative(
    seconds: i64,
    locale: Locale,
    direction: Direction,
    options: RenderOptions,
) -> String {
    const DAY_SECONDS: i64 = 24 * 60 * 60;
    const WEEK_SECONDS: i64 = 7 * DAY_SECONDS;
    const WEEK_ROUND_UP_THRESHOLD_SECONDS: i64 = 4 * DAY_SECONDS;
    const MONTH_SECONDS: i64 = 30 * 24 * 60 * 60;
    const FIRST_MONTH_START_SECONDS: i64 = MONTH_SECONDS;
    const MONTH_EARLY_SHIFT_SECONDS: i64 = 5 * DAY_SECONDS;
    const MONTH_HALF_THRESHOLD_SECONDS: i64 = 15 * DAY_SECONDS;
    const YEAR_HALF_THRESHOLD_MONTHS: i64 = 4;
    const YEAR_ROUND_UP_THRESHOLD_MONTHS: i64 = 10;
    const FIRST_YEAR_START_SECONDS: i64 = (11 * MONTH_SECONDS) + (15 * DAY_SECONDS);
    const HOUR_SECONDS: i64 = 60 * 60;
    const HOUR_HALF_THRESHOLD_SECONDS: i64 = 20 * 60;
    const HOUR_ROUND_UP_THRESHOLD_SECONDS: i64 = 50 * 60;

    // Year expressions are computed with month buckets.
    // First-year exception: start "a year" at 11m15d.
    if seconds >= FIRST_YEAR_START_SECONDS {
        let total_months = seconds / MONTH_SECONDS;
        if total_months < 12 {
            let year_unit = &UNITS[0];
            return match locale {
                Locale::En => render_duration_en(1, false, year_unit, direction),
                Locale::Ko => render_duration_ko(1, false, year_unit, direction, options),
            };
        }
        let mut base = total_months / 12;
        let rem_months = total_months % 12;
        let has_half = if rem_months >= YEAR_ROUND_UP_THRESHOLD_MONTHS {
            base += 1;
            false
        } else {
            rem_months >= YEAR_HALF_THRESHOLD_MONTHS
        };
        let base = base.max(1);
        let year_unit = &UNITS[0];
        return match locale {
            Locale::En => render_duration_en(base, has_half, year_unit, direction),
            Locale::Ko => render_duration_ko(base, has_half, year_unit, direction, options),
        };
    }

    // Month expressions with a 5-day early shift.
    // First month still starts at 30d, so 25~29d remains week-level ("4 weeks").
    // - 1 month:       [30d, 39d]
    // - 1 month half:  [40d, 54d]
    // - n months:      [30n-5d, 30n+9d]       for n >= 2
    // - n months half: [30n+10d, 30n+24d]     for n >= 2
    // - (n+1) months:  [30n+25d, 30n+39d]     for n >= 2
    let shifted_month_seconds = seconds + MONTH_EARLY_SHIFT_SECONDS;
    if seconds >= FIRST_MONTH_START_SECONDS && shifted_month_seconds >= MONTH_SECONDS {
        let base = (shifted_month_seconds / MONTH_SECONDS).max(1);
        let rem_shifted = shifted_month_seconds % MONTH_SECONDS;
        let has_half = rem_shifted >= MONTH_HALF_THRESHOLD_SECONDS;
        let month_unit = &UNITS[1];
        return match locale {
            Locale::En => render_duration_en(base, has_half, month_unit, direction),
            Locale::Ko => render_duration_ko(base, has_half, month_unit, direction, options),
        };
    }

    let mut selected = &UNITS[UNITS.len() - 1];
    for unit in &UNITS {
        if seconds >= unit.seconds {
            selected = unit;
            break;
        }
    }

    let mut base = (seconds / selected.seconds).max(1);
    let mut rem = seconds - (base * selected.seconds);
    let mut rounded_hour = false;

    // For hour-level expressions, round up at 2h50m+ (and similarly Nh50m+).
    if selected.seconds == HOUR_SECONDS && rem >= HOUR_ROUND_UP_THRESHOLD_SECONDS {
        base += 1;
        rem = 0;
        rounded_hour = true;
    }

    // For week-level expressions, round up at 1w4d+ (and similarly Nw4d+).
    // This yields 1w for 7~10 days, 2w for 11~17 days, 3w for 18~24 days, etc.
    if selected.seconds == WEEK_SECONDS && rem >= WEEK_ROUND_UP_THRESHOLD_SECONDS {
        base += 1;
        rem = 0;
    }

    let has_half = if rounded_hour || !selected.allow_half {
        false
    } else if selected.seconds == HOUR_SECONDS {
        rem >= HOUR_HALF_THRESHOLD_SECONDS
    } else {
        rem * selected.half_threshold_den >= selected.seconds * selected.half_threshold_num
    };

    match locale {
        Locale::En => render_duration_en(base, has_half, selected, direction),
        Locale::Ko => render_duration_ko(base, has_half, selected, direction, options),
    }
}
