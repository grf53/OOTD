use crate::daypart::DayPart;
use crate::types::{Direction, Unit};

pub(crate) fn render_duration_en(
    base: i64,
    has_half: bool,
    unit: &Unit,
    direction: Direction,
) -> String {
    let amount = if base == 1 {
        format!(
            "{} {}",
            indefinite_article(unit.name_en_singular),
            unit.name_en_singular
        )
    } else {
        format!("{} {}", base, unit.name_en_plural)
    };

    let body = if has_half {
        format!("{} and a half", amount)
    } else {
        amount
    };

    match direction {
        Direction::Past => format!("{} ago", body),
        Direction::Future => format!("{} later", body),
    }
}

fn indefinite_article(singular: &str) -> &'static str {
    match singular {
        "hour" => "an",
        _ => "a",
    }
}

pub(crate) fn render_daypart_en(
    day_diff: i64,
    daypart: DayPart,
    direction: Direction,
) -> Option<String> {
    let text = match (direction, day_diff, daypart) {
        (Direction::Past, -1, DayPart::Night) => "last night".to_string(),
        (Direction::Past, -1, p) => format!("yesterday {}", daypart_en(p)),
        (Direction::Past, 0, DayPart::Night) => "earlier tonight".to_string(),
        (Direction::Past, 0, p) => format!("this {}", daypart_en(p)),
        (Direction::Future, 0, DayPart::Night) => "tonight".to_string(),
        (Direction::Future, 0, p) => format!("this {}", daypart_en(p)),
        (Direction::Future, 1, p) => format!("tomorrow {}", daypart_en(p)),
        _ => return None,
    };

    Some(text)
}

fn daypart_en(daypart: DayPart) -> &'static str {
    match daypart {
        DayPart::Dawn => "dawn",
        DayPart::Morning => "morning",
        DayPart::Afternoon => "afternoon",
        DayPart::Evening => "evening",
        DayPart::Night => "night",
    }
}
