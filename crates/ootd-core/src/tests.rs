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
fn uses_half_for_hours_from_twenty_minutes() {
    let before = from_duration((2 * 60 * 60) + (19 * 60) + 59, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(before, "2 hours ago");

    let at = from_duration((2 * 60 * 60) + (20 * 60), Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(at, "2 hours and a half ago");

    let at_ko = from_duration((2 * 60 * 60) + (20 * 60), Locale::Ko, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_ko, "2시간 반 전");
}

#[test]
fn starts_month_labels_five_days_early_with_first_month_exception() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    let before_four_weeks = from_duration((25 * DAY_SECONDS) - 1, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(before_four_weeks, "3 weeks ago");

    let at_four_weeks = from_duration(25 * DAY_SECONDS, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_four_weeks, "4 weeks ago");

    let at_29_days = from_duration(29 * DAY_SECONDS, Locale::Ko, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_29_days, "4주 전");

    let at_month = from_duration(30 * DAY_SECONDS, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_month, "a month ago");

    let before_half = from_duration((40 * DAY_SECONDS) - 1, Locale::Ko, Direction::Past)
        .expect("valid duration");
    assert_eq!(before_half, "1달 전");

    let at_half = from_duration(40 * DAY_SECONDS, Locale::Ko, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_half, "1달 반 전");

    let at_next_month = from_duration(55 * DAY_SECONDS, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_next_month, "2 months ago");
}

#[test]
fn rounds_up_weeks_from_four_days() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    let ten_days = from_duration(10 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(ten_days, "1주 전");

    let eleven_days = from_duration(11 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(eleven_days, "2주 전");

    let seventeen_days = from_duration(17 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(seventeen_days, "2 weeks ago");

    let eighteen_days = from_duration(18 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(eighteen_days, "3 weeks ago");
}

#[test]
fn rounds_up_years_from_ten_months() {
    const MONTH_SECONDS: i64 = 30 * 24 * 60 * 60;
    const DAY_SECONDS: i64 = 24 * 60 * 60;
    let ten_months = 10 * MONTH_SECONDS;
    let eleven_half_months = (11 * MONTH_SECONDS) + (15 * DAY_SECONDS);
    let sixteen_months = 16 * MONTH_SECONDS;
    let thirty_four_months = 34 * MONTH_SECONDS;

    let at_ten_months = from_duration(ten_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_ten_months, "10 months ago");

    let at_eleven_half_months =
        from_duration(eleven_half_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_eleven_half_months, "a year ago");

    let at_sixteen_months = from_duration(sixteen_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_sixteen_months, "a year and a half ago");

    let before_thirty_four = from_duration(thirty_four_months - 1, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(before_thirty_four, "2 years and a half ago");

    let at_thirty_four = from_duration(thirty_four_months, Locale::En, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_thirty_four, "3 years ago");

    let at_thirty_four_ko = from_duration(thirty_four_months, Locale::Ko, Direction::Past)
        .expect("valid duration");
    assert_eq!(at_thirty_four_ko, "3년 전");
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
