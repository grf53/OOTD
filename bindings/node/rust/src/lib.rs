use napi::bindgen_prelude::Error;
use napi_derive::napi;
use ootd_core::{
    between_rfc3339_with_options, from_duration_with_options, range_of, range_of_at_rfc3339,
    Direction, DurationRange as CoreDurationRange, Locale, RenderOptions,
};
use std::str::FromStr;

#[napi(object)]
pub struct DurationRange {
    pub start: i64,
    pub end: i64,
}

#[napi(object)]
pub struct TimestampRange {
    pub start: String,
    pub end: String,
}

#[napi]
pub fn between(
    start_rfc3339: String,
    end_rfc3339: String,
    locale: Option<String>,
    use_native_ko_number: Option<bool>,
) -> napi::Result<String> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| Error::from_reason(e.to_string()))?;
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number.unwrap_or(false),
    };

    between_rfc3339_with_options(&start_rfc3339, &end_rfc3339, locale, options)
        .map_err(|e| Error::from_reason(e.to_string()))
}

#[napi(js_name = "fromDuration")]
pub fn from_duration_ts(
    seconds: i64,
    is_future: Option<bool>,
    locale: Option<String>,
    use_native_ko_number: Option<bool>,
) -> napi::Result<String> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| Error::from_reason(e.to_string()))?;
    let direction = if is_future.unwrap_or(false) {
        Direction::Future
    } else {
        Direction::Past
    };
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number.unwrap_or(false),
    };

    from_duration_with_options(seconds, locale, direction, options)
        .map_err(|e| Error::from_reason(e.to_string()))
}

#[napi(js_name = "rangeOf")]
pub fn range_of_ts(expression: String, locale: Option<String>) -> napi::Result<DurationRange> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| Error::from_reason(e.to_string()))?;
    let range = range_of(&expression, locale).map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(DurationRange {
        start: range.start_seconds,
        end: range.end_seconds,
    })
}

#[napi(js_name = "resolveDurationRange")]
pub fn resolve_duration_range_ts(
    start: i64,
    end: i64,
    anchor_rfc3339: Option<String>,
) -> napi::Result<TimestampRange> {
    let range = CoreDurationRange {
        start_seconds: start,
        end_seconds: end,
    };
    let resolved = match anchor_rfc3339 {
        Some(anchor) => range.resolve_at_rfc3339(&anchor),
        None => range.resolve_now(),
    }
    .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(TimestampRange {
        start: resolved.start.to_rfc3339(),
        end: resolved.end.to_rfc3339(),
    })
}

#[napi(js_name = "rangeOfTimestamps")]
pub fn range_of_timestamps_ts(
    expression: String,
    locale: Option<String>,
    anchor_rfc3339: Option<String>,
) -> napi::Result<TimestampRange> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| Error::from_reason(e.to_string()))?;
    let resolved = match anchor_rfc3339 {
        Some(anchor) => {
            let range = range_of_at_rfc3339(&expression, locale, &anchor)
                .map_err(|e| Error::from_reason(e.to_string()))?;
            range
                .resolve_at_rfc3339(&anchor)
                .map_err(|e| Error::from_reason(e.to_string()))?
        }
        None => {
            let range =
                range_of(&expression, locale).map_err(|e| Error::from_reason(e.to_string()))?;
            range
                .resolve_now()
                .map_err(|e| Error::from_reason(e.to_string()))?
        }
    };

    Ok(TimestampRange {
        start: resolved.start.to_rfc3339(),
        end: resolved.end.to_rfc3339(),
    })
}
