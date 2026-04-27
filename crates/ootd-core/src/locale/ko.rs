use crate::daypart::DayPart;
use crate::types::{Direction, RenderOptions, Unit, UnitKind};

pub(crate) fn render_duration_ko(
    base: i64,
    has_half: bool,
    unit: &Unit,
    direction: Direction,
    options: RenderOptions,
) -> String {
    let mut body = format_korean_count(base, unit, options);
    if has_half {
        body.push_str(" 반");
    }

    match direction {
        Direction::Past => format!("{} 전", body),
        Direction::Future => format!("{} 후", body),
    }
}

pub(crate) fn render_daypart_ko(
    day_diff: i64,
    daypart: DayPart,
    direction: Direction,
) -> Option<String> {
    let day_label = match (direction, day_diff) {
        (Direction::Past, -1) => "어제",
        (Direction::Past, 0) => "오늘",
        (Direction::Future, 0) => "오늘",
        (Direction::Future, 1) => "내일",
        _ => return None,
    };

    let part = match daypart {
        DayPart::Dawn => "새벽",
        DayPart::Morning => "아침",
        DayPart::Afternoon => "낮",
        DayPart::Evening => "저녁",
        DayPart::Night => "밤",
    };

    Some(format!("{} {}", day_label, part))
}

fn format_korean_count(base: i64, unit: &Unit, options: RenderOptions) -> String {
    if options.ko_native_numerals && matches!(unit.kind, UnitKind::Hour | UnitKind::Month) {
        if let Some(native) = korean_native_counter_number(base) {
            return format!("{} {}", native, unit.name_ko);
        }
    }

    format!("{}{}", base, unit.name_ko)
}

fn korean_native_counter_number(value: i64) -> Option<String> {
    if !(1..=99).contains(&value) {
        return None;
    }

    if value < 10 {
        return korean_native_single(value).map(str::to_string);
    }

    if value < 20 {
        if value == 10 {
            return Some("열".to_string());
        }
        let one = korean_native_single(value - 10)?;
        return Some(format!("열{}", one));
    }

    let tens = value / 10;
    let ones = value % 10;
    let tens_word = match tens {
        2 if ones == 0 => "스무",
        2 => "스물",
        3 => "서른",
        4 => "마흔",
        5 => "쉰",
        6 => "예순",
        7 => "일흔",
        8 => "여든",
        9 => "아흔",
        _ => return None,
    };

    if ones == 0 {
        return Some(tens_word.to_string());
    }

    let one = korean_native_single(ones)?;
    Some(format!("{}{}", tens_word, one))
}

fn korean_native_single(value: i64) -> Option<&'static str> {
    match value {
        1 => Some("한"),
        2 => Some("두"),
        3 => Some("세"),
        4 => Some("네"),
        5 => Some("다섯"),
        6 => Some("여섯"),
        7 => Some("일곱"),
        8 => Some("여덟"),
        9 => Some("아홉"),
        _ => None,
    }
}
