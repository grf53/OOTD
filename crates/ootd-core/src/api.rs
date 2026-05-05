use chrono::{DateTime, Duration, FixedOffset};

use crate::daypart::between_daypart;
use crate::expression::range_of_impl;
use crate::render::render_duration_nonnegative;
use crate::types::{Direction, DurationRange, Locale, OotdError, RenderOptions};

pub fn between(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>, locale: Locale) -> String {
    between_with_options(start, end, locale, RenderOptions::default())
}

pub fn between_with_options(
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    locale: Locale,
    options: RenderOptions,
) -> String {
    let delta = end.signed_duration_since(start);
    let abs_seconds = delta.num_seconds().unsigned_abs() as i64;
    let direction = if delta >= Duration::zero() {
        Direction::Past
    } else {
        Direction::Future
    };

    if (3 * 60 * 60..=24 * 60 * 60).contains(&abs_seconds) {
        if let Some(special) = between_daypart(start, end, locale, direction) {
            return special;
        }
    }

    match from_duration_with_options(abs_seconds, locale, direction, options) {
        Ok(v) => v,
        Err(_) => unreachable!("between passes non-negative duration"),
    }
}

pub fn between_rfc3339(start: &str, end: &str, locale: Locale) -> Result<String, OotdError> {
    between_rfc3339_with_options(start, end, locale, RenderOptions::default())
}

pub fn between_rfc3339_with_options(
    start: &str,
    end: &str,
    locale: Locale,
    options: RenderOptions,
) -> Result<String, OotdError> {
    let start_dt = DateTime::parse_from_rfc3339(start)
        .map_err(|_| OotdError::InvalidDatetime(start.to_string()))?;
    let end_dt = DateTime::parse_from_rfc3339(end)
        .map_err(|_| OotdError::InvalidDatetime(end.to_string()))?;

    Ok(between_with_options(start_dt, end_dt, locale, options))
}

pub fn from_duration(
    seconds: i64,
    locale: Locale,
    direction: Direction,
) -> Result<String, OotdError> {
    from_duration_with_options(seconds, locale, direction, RenderOptions::default())
}

pub fn from_duration_with_options(
    seconds: i64,
    locale: Locale,
    direction: Direction,
    options: RenderOptions,
) -> Result<String, OotdError> {
    if seconds < 0 {
        return Err(OotdError::NegativeDuration(seconds));
    }
    Ok(render_duration_nonnegative(
        seconds, locale, direction, options,
    ))
}

pub fn range_of(expression: &str, locale: Locale) -> Result<DurationRange, OotdError> {
    range_of_impl(expression, locale, None)
}

pub fn range_of_at(
    expression: &str,
    locale: Locale,
    anchor: DateTime<FixedOffset>,
) -> Result<DurationRange, OotdError> {
    range_of_impl(expression, locale, Some(anchor))
}

pub fn range_of_at_rfc3339(
    expression: &str,
    locale: Locale,
    anchor_rfc3339: &str,
) -> Result<DurationRange, OotdError> {
    let anchor = DateTime::parse_from_rfc3339(anchor_rfc3339)
        .map_err(|_| OotdError::InvalidDatetime(anchor_rfc3339.to_string()))?;
    range_of_at(expression, locale, anchor)
}
