use std::str::FromStr;

use super::*;

#[test]
fn supports_rfc3339_between() {
    let value = between_rfc3339("2021-04-30T11:57:16Z", "2024-01-25T13:31:43Z", Locale::En)
        .expect("valid datetime");
    assert_eq!(value, "2 years and a half ago");
}

#[test]
fn supports_korean_half_phrase() {
    let value = from_duration(90 * 60, Locale::Ko, Direction::Past).expect("must succeed");
    assert_eq!(value, "1시간 반 전");
}

#[test]
fn rejects_unknown_locale() {
    let err = Locale::from_str("fr_FR").expect_err("must fail");
    assert!(matches!(err, OotdError::UnsupportedLocale(_)));
}

#[test]
fn uses_daypart_in_three_to_twenty_four_hours() {
    let out = between_rfc3339("2024-01-24T20:29:54Z", "2024-01-25T13:31:43Z", Locale::Ko)
        .expect("valid datetime");
    assert_eq!(out, "어제 밤");
}

#[test]
fn keeps_numeric_for_sub_three_hours() {
    let out = between_rfc3339("2024-01-25T11:31:43Z", "2024-01-25T13:31:43Z", Locale::Ko)
        .expect("valid datetime");
    assert_eq!(out, "2시간 전");
}

#[test]
fn rejects_negative_duration() {
    let err = from_duration(-1, Locale::En, Direction::Past).expect_err("must fail");
    assert!(matches!(err, OotdError::NegativeDuration(-1)));
}

#[test]
fn exact_twenty_four_hours_keeps_daypart() {
    let out = between_rfc3339("2024-01-24T13:31:43Z", "2024-01-25T13:31:43Z", Locale::En)
        .expect("valid datetime");
    assert_eq!(out, "yesterday afternoon");
}

#[test]
fn same_day_past_night_is_earlier_tonight() {
    let out = between_rfc3339("2024-01-25T20:30:00Z", "2024-01-25T23:30:00Z", Locale::En)
        .expect("valid datetime");
    assert_eq!(out, "earlier tonight");
}

#[test]
fn rounds_up_hours_from_fifty_minutes() {
    let past = from_duration((2 * 60 * 60) + (50 * 60), Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(past, "3 hours ago");

    let future = from_duration((2 * 60 * 60) + (50 * 60), Locale::En, Direction::Future)
        .expect("valid duration");
    assert_eq!(future, "3 hours later");
}

#[test]
fn does_not_use_half_for_minutes() {
    let en = from_duration(90, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(en, "a minute ago");

    let ko = from_duration(90, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(ko, "1분 전");
}

#[test]
fn uses_an_for_single_hour_in_english() {
    let out = from_duration(90 * 60, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(out, "an hour and a half ago");
}

#[test]
fn supports_native_korean_numbers_for_hour_and_month_when_enabled() {
    let options = RenderOptions {
        ko_native_numerals: true,
    };

    let hour = from_duration_with_options(90 * 60, Locale::Ko, Direction::Past, options)
        .expect("valid duration");
    assert_eq!(hour, "한 시간 반 전");

    let month = from_duration_with_options(46 * 24 * 60 * 60, Locale::Ko, Direction::Past, options)
        .expect("valid duration");
    assert_eq!(month, "한 달 반 전");

    let week = from_duration_with_options(14 * 24 * 60 * 60, Locale::Ko, Direction::Past, options)
        .expect("valid duration");
    assert_eq!(week, "2주 전");
}
