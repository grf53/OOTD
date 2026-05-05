use chrono::{DateTime, Duration, FixedOffset, TimeZone};

use crate::duration_policy::resolve_bucket_range;
use crate::types::{Direction, DurationRange, Locale, OotdError, UnitKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParsedExpression {
    direction: Direction,
    kind: UnitKind,
    base: i64,
    has_half: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsedDaypart {
    Dawn,
    Morning,
    Afternoon,
    Evening,
    Night,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParsedDaypartExpression {
    day_offset: i64,
    daypart: ParsedDaypart,
}

pub(crate) fn range_of_impl(
    expression: &str,
    locale: Locale,
    anchor: Option<DateTime<FixedOffset>>,
) -> Result<DurationRange, OotdError> {
    if let Some(daypart_range) = resolve_daypart_expression_range(expression, locale, anchor) {
        return Ok(daypart_range);
    }

    let parsed = match locale {
        Locale::En => parse_en_expression(expression),
        Locale::Ko => parse_ko_expression(expression),
    }
    .ok_or_else(|| OotdError::InvalidExpression(expression.to_string()))?;

    let abs = resolve_bucket_range(parsed.kind, parsed.base, parsed.has_half)
        .ok_or_else(|| OotdError::InvalidExpression(expression.to_string()))?;

    let (start_seconds, end_seconds) = match parsed.direction {
        Direction::Past => (abs.max_seconds.checked_neg(), abs.min_seconds.checked_neg()),
        Direction::Future => (Some(abs.min_seconds), Some(abs.max_seconds)),
    };

    Ok(DurationRange {
        start_seconds: start_seconds
            .ok_or_else(|| OotdError::InvalidExpression(expression.to_string()))?,
        end_seconds: end_seconds
            .ok_or_else(|| OotdError::InvalidExpression(expression.to_string()))?,
    })
}

fn resolve_daypart_expression_range(
    expression: &str,
    locale: Locale,
    anchor: Option<DateTime<FixedOffset>>,
) -> Option<DurationRange> {
    let parsed = match locale {
        Locale::En => parse_en_daypart_expression(expression),
        Locale::Ko => parse_ko_daypart_expression(expression),
    }?;

    let anchor = anchor.unwrap_or_else(default_anchor_now);
    daypart_expression_to_range(parsed, anchor)
}

fn parse_en_expression(expression: &str) -> Option<ParsedExpression> {
    let normalized = expression.trim().to_ascii_lowercase();
    let (body, direction) = if let Some(body) = normalized.strip_suffix(" ago") {
        (body.trim(), Direction::Past)
    } else if let Some(body) = normalized.strip_suffix(" later") {
        (body.trim(), Direction::Future)
    } else {
        return None;
    };

    let (body, has_half) = if let Some(body) = body.strip_suffix(" and a half") {
        (body.trim(), true)
    } else {
        (body, false)
    };

    if let Some(unit_text) = body.strip_prefix("a ").or_else(|| body.strip_prefix("an ")) {
        return Some(ParsedExpression {
            direction,
            kind: unit_kind_from_en(unit_text.trim())?,
            base: 1,
            has_half,
        });
    }

    let mut parts = body.split_whitespace();
    let base = parts.next()?.parse::<i64>().ok()?;
    let unit_text = parts.collect::<Vec<_>>().join(" ");
    if unit_text.is_empty() {
        return None;
    }

    Some(ParsedExpression {
        direction,
        kind: unit_kind_from_en(&unit_text)?,
        base,
        has_half,
    })
}

fn parse_ko_expression(expression: &str) -> Option<ParsedExpression> {
    let normalized = expression.trim();
    let (body, direction) = if let Some(body) = normalized.strip_suffix(" 전") {
        (body.trim(), Direction::Past)
    } else if let Some(body) = normalized.strip_suffix(" 후") {
        (body.trim(), Direction::Future)
    } else {
        return None;
    };

    let (body, has_half) = if let Some(body) = body.strip_suffix(" 반") {
        (body.trim(), true)
    } else {
        (body, false)
    };

    let (kind, unit_text) = unit_kind_from_ko_suffix(body)?;
    let amount_text = body.strip_suffix(unit_text)?.trim();
    let base = if amount_text.chars().all(|ch| ch.is_ascii_digit()) {
        amount_text.parse::<i64>().ok()?
    } else {
        parse_korean_native_counter_number(amount_text)?
    };

    Some(ParsedExpression {
        direction,
        kind,
        base,
        has_half,
    })
}

fn parse_en_daypart_expression(expression: &str) -> Option<ParsedDaypartExpression> {
    let normalized = normalize_spaces(expression).to_ascii_lowercase();

    if normalized == "last night" {
        return Some(ParsedDaypartExpression {
            day_offset: -1,
            daypart: ParsedDaypart::Night,
        });
    }

    if normalized == "earlier tonight" || normalized == "tonight" {
        return Some(ParsedDaypartExpression {
            day_offset: 0,
            daypart: ParsedDaypart::Night,
        });
    }

    if let Some(part) = normalized.strip_prefix("yesterday ") {
        return Some(ParsedDaypartExpression {
            day_offset: -1,
            daypart: parsed_daypart_from_en(part)?,
        });
    }

    if let Some(part) = normalized.strip_prefix("this ") {
        return Some(ParsedDaypartExpression {
            day_offset: 0,
            daypart: parsed_daypart_from_en(part)?,
        });
    }

    if let Some(part) = normalized.strip_prefix("tomorrow ") {
        return Some(ParsedDaypartExpression {
            day_offset: 1,
            daypart: parsed_daypart_from_en(part)?,
        });
    }

    None
}

fn parse_ko_daypart_expression(expression: &str) -> Option<ParsedDaypartExpression> {
    let normalized = normalize_spaces(expression);
    let mut parts = normalized.split(' ');
    let day_text = parts.next()?;
    let part_text = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    let day_offset = match day_text {
        "어제" => -1,
        "오늘" => 0,
        "내일" => 1,
        _ => return None,
    };

    Some(ParsedDaypartExpression {
        day_offset,
        daypart: parsed_daypart_from_ko(part_text)?,
    })
}

fn parsed_daypart_from_en(part: &str) -> Option<ParsedDaypart> {
    match part {
        "dawn" => Some(ParsedDaypart::Dawn),
        "morning" => Some(ParsedDaypart::Morning),
        "afternoon" => Some(ParsedDaypart::Afternoon),
        "evening" => Some(ParsedDaypart::Evening),
        "night" => Some(ParsedDaypart::Night),
        _ => None,
    }
}

fn parsed_daypart_from_ko(part: &str) -> Option<ParsedDaypart> {
    match part {
        "새벽" => Some(ParsedDaypart::Dawn),
        "아침" => Some(ParsedDaypart::Morning),
        "낮" => Some(ParsedDaypart::Afternoon),
        "저녁" => Some(ParsedDaypart::Evening),
        "밤" => Some(ParsedDaypart::Night),
        _ => None,
    }
}

fn daypart_expression_to_range(
    parsed: ParsedDaypartExpression,
    anchor: DateTime<FixedOffset>,
) -> Option<DurationRange> {
    let offset = *anchor.offset();
    let local_anchor = anchor.with_timezone(&offset);
    let date = local_anchor.date_naive();
    let target_date = date.checked_add_signed(Duration::try_days(parsed.day_offset)?)?;
    let (start_hour, end_hour) = daypart_hour_bounds(parsed.daypart);

    let start_local = target_date.and_hms_opt(start_hour, 0, 0)?;
    let end_local = target_date.and_hms_opt(end_hour, 59, 59)?;
    let start = offset.from_local_datetime(&start_local).single()?;
    let end = offset.from_local_datetime(&end_local).single()?;

    Some(DurationRange {
        start_seconds: start.signed_duration_since(anchor).num_seconds(),
        end_seconds: end.signed_duration_since(anchor).num_seconds(),
    })
}

fn daypart_hour_bounds(daypart: ParsedDaypart) -> (u32, u32) {
    match daypart {
        ParsedDaypart::Dawn => (0, 4),
        ParsedDaypart::Morning => (5, 10),
        ParsedDaypart::Afternoon => (11, 16),
        ParsedDaypart::Evening => (17, 19),
        ParsedDaypart::Night => (20, 23),
    }
}

fn normalize_spaces(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn unit_kind_from_en(unit: &str) -> Option<UnitKind> {
    match unit {
        "year" | "years" => Some(UnitKind::Year),
        "month" | "months" => Some(UnitKind::Month),
        "week" | "weeks" => Some(UnitKind::Week),
        "day" | "days" => Some(UnitKind::Day),
        "hour" | "hours" => Some(UnitKind::Hour),
        "minute" | "minutes" => Some(UnitKind::Minute),
        "second" | "seconds" => Some(UnitKind::Second),
        _ => None,
    }
}

fn unit_kind_from_ko_suffix(body: &str) -> Option<(UnitKind, &'static str)> {
    for (kind, unit_text) in [
        (UnitKind::Hour, "시간"),
        (UnitKind::Year, "년"),
        (UnitKind::Month, "달"),
        (UnitKind::Week, "주"),
        (UnitKind::Day, "일"),
        (UnitKind::Minute, "분"),
        (UnitKind::Second, "초"),
    ] {
        if body.ends_with(unit_text) {
            return Some((kind, unit_text));
        }
    }

    None
}

fn parse_korean_native_counter_number(text: &str) -> Option<i64> {
    (1..=99).find(|value| korean_native_counter_number(*value).as_deref() == Some(text))
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

#[cfg(target_arch = "wasm32")]
fn default_anchor_now() -> DateTime<FixedOffset> {
    chrono::Utc::now().fixed_offset()
}

#[cfg(not(target_arch = "wasm32"))]
fn default_anchor_now() -> DateTime<FixedOffset> {
    chrono::Local::now().fixed_offset()
}
