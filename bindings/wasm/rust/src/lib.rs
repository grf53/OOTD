use ootd_core::{
    between_rfc3339_with_options, from_duration_with_options, Direction, Locale, RenderOptions,
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
