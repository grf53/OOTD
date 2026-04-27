use crate::locale::{render_duration_en, render_duration_ko};
use crate::types::{Direction, Locale, RenderOptions, UNITS};

pub(crate) fn render_duration_nonnegative(
    seconds: i64,
    locale: Locale,
    direction: Direction,
    options: RenderOptions,
) -> String {
    const HOUR_SECONDS: i64 = 60 * 60;
    const HOUR_ROUND_UP_THRESHOLD_SECONDS: i64 = 50 * 60;

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

    let has_half = !rounded_hour
        && selected.allow_half
        && rem * selected.half_threshold_den >= selected.seconds * selected.half_threshold_num;

    match locale {
        Locale::En => render_duration_en(base, has_half, selected, direction),
        Locale::Ko => render_duration_ko(base, has_half, selected, direction, options),
    }
}
