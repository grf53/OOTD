use ootd_core::{
    between_rfc3339_with_options, from_duration_with_options, range_of, range_of_at_rfc3339,
    Direction, DurationRange, Locale, RenderOptions,
};
use std::ffi::{c_char, CStr, CString};
use std::str::FromStr;

#[repr(C)]
pub struct OotdDurationInput {
    pub seconds: i64,
    pub is_future: bool,
    pub locale: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn ootd_between_rfc3339(
    start: *const c_char,
    end: *const c_char,
    locale: *const c_char,
) -> *mut c_char {
    ootd_between_rfc3339_with_options(start, end, locale, false)
}

#[no_mangle]
pub unsafe extern "C" fn ootd_between_rfc3339_with_options(
    start: *const c_char,
    end: *const c_char,
    locale: *const c_char,
    use_native_ko_number: bool,
) -> *mut c_char {
    let start = match cstr_to_string(start) {
        Some(value) => value,
        None => return std::ptr::null_mut(),
    };

    let end = match cstr_to_string(end) {
        Some(value) => value,
        None => return std::ptr::null_mut(),
    };

    let locale = match cstr_to_locale(locale) {
        Some(value) => value,
        None => return std::ptr::null_mut(),
    };
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number,
    };

    match between_rfc3339_with_options(&start, &end, locale, options) {
        Ok(value) => into_raw_string(value),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ootd_from_duration(input: OotdDurationInput) -> *mut c_char {
    ootd_from_duration_parts_with_options(input.seconds, input.is_future, input.locale, false)
}

#[no_mangle]
pub unsafe extern "C" fn ootd_from_duration_parts(
    seconds: i64,
    is_future: bool,
    locale: *const c_char,
) -> *mut c_char {
    ootd_from_duration_parts_with_options(seconds, is_future, locale, false)
}

#[no_mangle]
pub unsafe extern "C" fn ootd_from_duration_parts_with_options(
    seconds: i64,
    is_future: bool,
    locale: *const c_char,
    use_native_ko_number: bool,
) -> *mut c_char {
    let locale = match cstr_to_locale(locale) {
        Some(value) => value,
        None => return std::ptr::null_mut(),
    };
    let direction = if is_future {
        Direction::Future
    } else {
        Direction::Past
    };
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number,
    };
    match from_duration_with_options(seconds, locale, direction, options) {
        Ok(rendered) => into_raw_string(rendered),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ootd_range_of(
    expression: *const c_char,
    locale: *const c_char,
    out_start_seconds: *mut i64,
    out_end_seconds: *mut i64,
) -> bool {
    if out_start_seconds.is_null() || out_end_seconds.is_null() {
        return false;
    }

    let expression = match cstr_to_string(expression) {
        Some(value) => value,
        None => return false,
    };
    let locale = match cstr_to_locale(locale) {
        Some(value) => value,
        None => return false,
    };

    match range_of(&expression, locale) {
        Ok(range) => {
            *out_start_seconds = range.start_seconds;
            *out_end_seconds = range.end_seconds;
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ootd_range_of_at_rfc3339(
    expression: *const c_char,
    locale: *const c_char,
    anchor_rfc3339: *const c_char,
    out_start_seconds: *mut i64,
    out_end_seconds: *mut i64,
) -> bool {
    if out_start_seconds.is_null() || out_end_seconds.is_null() {
        return false;
    }

    let expression = match cstr_to_string(expression) {
        Some(value) => value,
        None => return false,
    };
    let locale = match cstr_to_locale(locale) {
        Some(value) => value,
        None => return false,
    };
    let anchor_rfc3339 = match cstr_to_string(anchor_rfc3339) {
        Some(value) => value,
        None => return false,
    };

    match range_of_at_rfc3339(&expression, locale, &anchor_rfc3339) {
        Ok(range) => {
            *out_start_seconds = range.start_seconds;
            *out_end_seconds = range.end_seconds;
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ootd_duration_range_resolve_at_rfc3339(
    start_seconds: i64,
    end_seconds: i64,
    anchor_rfc3339: *const c_char,
    out_start_rfc3339: *mut *mut c_char,
    out_end_rfc3339: *mut *mut c_char,
) -> bool {
    if out_start_rfc3339.is_null() || out_end_rfc3339.is_null() {
        return false;
    }

    let anchor_rfc3339 = match cstr_to_string(anchor_rfc3339) {
        Some(value) => value,
        None => return false,
    };

    let range = DurationRange {
        start_seconds,
        end_seconds,
    };
    let resolved = match range.resolve_at_rfc3339(&anchor_rfc3339) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let start_raw = match CString::new(resolved.start.to_rfc3339()) {
        Ok(value) => value.into_raw(),
        Err(_) => return false,
    };
    let end_raw = match CString::new(resolved.end.to_rfc3339()) {
        Ok(value) => value.into_raw(),
        Err(_) => {
            let _ = CString::from_raw(start_raw);
            return false;
        }
    };

    *out_start_rfc3339 = start_raw;
    *out_end_rfc3339 = end_raw;
    true
}

#[no_mangle]
pub unsafe extern "C" fn ootd_free_string(raw: *mut c_char) {
    if raw.is_null() {
        return;
    }

    let _ = CString::from_raw(raw);
}

fn into_raw_string(value: String) -> *mut c_char {
    match CString::new(value) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

unsafe fn cstr_to_string(raw: *const c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    CStr::from_ptr(raw).to_str().ok().map(ToString::to_string)
}

unsafe fn cstr_to_locale(raw: *const c_char) -> Option<Locale> {
    let value = cstr_to_string(raw)?;
    Locale::from_str(&value).ok()
}
