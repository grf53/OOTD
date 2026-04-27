use ootd_core::{
    between_rfc3339_with_options, from_duration_with_options, Direction, Locale, RenderOptions,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::str::FromStr;

#[pyfunction]
#[pyo3(signature = (start_rfc3339, end_rfc3339, locale="en", use_native_ko_number=false))]
fn between(
    start_rfc3339: &Bound<'_, PyAny>,
    end_rfc3339: &Bound<'_, PyAny>,
    locale: &str,
    use_native_ko_number: bool,
) -> PyResult<String> {
    let locale = Locale::from_str(locale).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let start = coerce_to_rfc3339(start_rfc3339)?;
    let end = coerce_to_rfc3339(end_rfc3339)?;
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number,
    };

    between_rfc3339_with_options(&start, &end, locale, options)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction(name = "from_duration")]
#[pyo3(signature = (seconds, is_future=false, locale="en", use_native_ko_number=false))]
fn from_duration_py(
    seconds: &Bound<'_, PyAny>,
    is_future: bool,
    locale: &str,
    use_native_ko_number: bool,
) -> PyResult<String> {
    let seconds = coerce_to_seconds(seconds)?;
    let locale = Locale::from_str(locale).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let direction = if is_future {
        Direction::Future
    } else {
        Direction::Past
    };
    let options = RenderOptions {
        ko_native_numerals: use_native_ko_number,
    };

    from_duration_with_options(seconds, locale, direction, options)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

fn coerce_to_seconds(value: &Bound<'_, PyAny>) -> PyResult<i64> {
    if let Ok(seconds) = value.extract::<i64>() {
        return Ok(seconds);
    }

    if let Ok(seconds) = value.extract::<f64>() {
        return coerce_float_seconds(seconds);
    }

    if let Ok(total_seconds) = value.call_method0("total_seconds") {
        let total_seconds = total_seconds.extract::<f64>().map_err(|_| {
            PyValueError::new_err("timedelta-like total_seconds() must return a number")
        })?;
        return coerce_float_seconds(total_seconds);
    }

    Err(PyValueError::new_err(
        "expected seconds as int/float or timedelta-like object with total_seconds()",
    ))
}

fn coerce_float_seconds(value: f64) -> PyResult<i64> {
    if !value.is_finite() {
        return Err(PyValueError::new_err("seconds must be a finite number"));
    }

    let truncated = value.trunc();
    if truncated < i64::MIN as f64 || truncated > i64::MAX as f64 {
        return Err(PyValueError::new_err("seconds is out of range for i64"));
    }

    Ok(truncated as i64)
}

fn coerce_to_rfc3339(value: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(s) = value.extract::<String>() {
        return Ok(s);
    }

    let utcoffset = value.call_method0("utcoffset").map_err(|_| {
        PyValueError::new_err("expected RFC3339 string or datetime-like object with utcoffset()")
    })?;
    if utcoffset.is_none() {
        return Err(PyValueError::new_err(
            "naive datetime is not supported; pass timezone-aware datetime",
        ));
    }

    value.call_method0("isoformat")?.extract::<String>()
}

#[pymodule]
fn _native(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(between, module)?)?;
    module.add_function(wrap_pyfunction!(from_duration_py, module)?)?;
    Ok(())
}
