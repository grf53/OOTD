use js_sys::{Object, Reflect};
use ootd_core::{
    between_rfc3339_with_options, extract_expressions, from_duration_with_options, range_of,
    range_of_at_rfc3339, Direction, DurationRange as CoreDurationRange, Locale, RenderOptions,
};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn between(
    start_rfc3339: &str,
    end_rfc3339: &str,
    locale: Option<String>,
    use_native_ko_number: Option<bool>,
) -> Result<String, JsValue> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number.unwrap_or(false),
    };

    between_rfc3339_with_options(start_rfc3339, end_rfc3339, locale, options)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = fromDuration)]
pub fn from_duration_wasm(
    seconds: i64,
    is_future: Option<bool>,
    locale: Option<String>,
    use_native_ko_number: Option<bool>,
) -> Result<String, JsValue> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let direction = if is_future.unwrap_or(false) {
        Direction::Future
    } else {
        Direction::Past
    };
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number.unwrap_or(false),
    };

    from_duration_with_options(seconds, locale, direction, options)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = rangeOf)]
pub fn range_of_wasm(expression: &str, locale: Option<String>) -> Result<JsValue, JsValue> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let range = range_of(expression, locale).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let out = Object::new();
    Reflect::set(
        &out,
        &JsValue::from_str("start"),
        &JsValue::from_f64(range.start_seconds as f64),
    )
    .map_err(|e| JsValue::from(e))?;
    Reflect::set(
        &out,
        &JsValue::from_str("end"),
        &JsValue::from_f64(range.end_seconds as f64),
    )
    .map_err(|e| JsValue::from(e))?;

    Ok(out.into())
}

#[wasm_bindgen(js_name = resolveDurationRange)]
pub fn resolve_duration_range_wasm(
    start: f64,
    end: f64,
    anchor_rfc3339: Option<String>,
) -> Result<JsValue, JsValue> {
    let start_seconds = coerce_js_seconds(start)?;
    let end_seconds = coerce_js_seconds(end)?;
    let range = CoreDurationRange {
        start_seconds,
        end_seconds,
    };
    let resolved = match anchor_rfc3339 {
        Some(anchor) => range.resolve_at_rfc3339(&anchor),
        None => range.resolve_now(),
    }
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let out = Object::new();
    Reflect::set(
        &out,
        &JsValue::from_str("start"),
        &JsValue::from_str(&resolved.start.to_rfc3339()),
    )?;
    Reflect::set(
        &out,
        &JsValue::from_str("end"),
        &JsValue::from_str(&resolved.end.to_rfc3339()),
    )?;

    Ok(out.into())
}

#[wasm_bindgen(js_name = rangeOfTimestamps)]
pub fn range_of_timestamps_wasm(
    expression: &str,
    locale: Option<String>,
    anchor_rfc3339: Option<String>,
) -> Result<JsValue, JsValue> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let resolved = match anchor_rfc3339 {
        Some(anchor) => {
            let range = range_of_at_rfc3339(expression, locale, &anchor)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            range
                .resolve_at_rfc3339(&anchor)
                .map_err(|e| JsValue::from_str(&e.to_string()))?
        }
        None => {
            let range =
                range_of(expression, locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
            range
                .resolve_now()
                .map_err(|e| JsValue::from_str(&e.to_string()))?
        }
    };

    let out = Object::new();
    Reflect::set(
        &out,
        &JsValue::from_str("start"),
        &JsValue::from_str(&resolved.start.to_rfc3339()),
    )?;
    Reflect::set(
        &out,
        &JsValue::from_str("end"),
        &JsValue::from_str(&resolved.end.to_rfc3339()),
    )?;

    Ok(out.into())
}

#[wasm_bindgen(js_name = extractExpressions)]
pub fn extract_expressions_wasm(input: &str, locale: Option<String>) -> Result<JsValue, JsValue> {
    let locale = locale.unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let candidates = extract_expressions(input, locale);
    let arr = js_sys::Array::new();
    for c in candidates {
        let out = Object::new();
        Reflect::set(
            &out,
            &JsValue::from_str("start"),
            &JsValue::from_f64(c.start as f64),
        )?;
        Reflect::set(
            &out,
            &JsValue::from_str("end"),
            &JsValue::from_f64(c.end as f64),
        )?;
        Reflect::set(&out, &JsValue::from_str("text"), &JsValue::from_str(&c.text))?;
        arr.push(&out);
    }
    Ok(arr.into())
}

fn coerce_js_seconds(value: f64) -> Result<i64, JsValue> {
    if !value.is_finite() {
        return Err(JsValue::from_str("seconds must be a finite number"));
    }

    let truncated = value.trunc();
    if truncated < i64::MIN as f64 || truncated > i64::MAX as f64 {
        return Err(JsValue::from_str("seconds is out of range for i64"));
    }

    Ok(truncated as i64)
}
