use chrono::{DateTime, FixedOffset, Timelike};

use crate::locale::{render_daypart_en, render_daypart_ko};
use crate::types::{Direction, Locale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DayPart {
    Dawn,
    Morning,
    Afternoon,
    Evening,
    Night,
}

pub(crate) fn between_daypart(
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    locale: Locale,
    direction: Direction,
) -> Option<String> {
    // Policy: daypart labels are anchored to the `end` timezone.
    // This keeps wording consistent with the "now" perspective even when
    // start/end offsets are mixed (e.g. +09:00 vs Z).
    let anchor_offset = *end.offset();
    let start_in_anchor_tz = start.with_timezone(&anchor_offset);
    let end_in_anchor_tz = end.with_timezone(&anchor_offset);

    let day_diff = start_in_anchor_tz
        .date_naive()
        .signed_duration_since(end_in_anchor_tz.date_naive())
        .num_days();
    if !(-1..=1).contains(&day_diff) {
        return None;
    }

    let daypart = DayPart::from_hour(start_in_anchor_tz.hour() as i64)?;
    Some(match locale {
        Locale::Ko => render_daypart_ko(day_diff, daypart, direction)?,
        Locale::En => render_daypart_en(day_diff, daypart, direction)?,
    })
}

impl DayPart {
    fn from_hour(hour: i64) -> Option<Self> {
        match hour {
            0..=4 => Some(Self::Dawn),
            5..=10 => Some(Self::Morning),
            11..=16 => Some(Self::Afternoon),
            17..=19 => Some(Self::Evening),
            20..=23 => Some(Self::Night),
            _ => None,
        }
    }
}
