use chrono::DateTime;
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

    let before_four_weeks =
        from_duration((25 * DAY_SECONDS) - 1, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(before_four_weeks, "3 weeks ago");

    let at_four_weeks =
        from_duration(25 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_four_weeks, "4 weeks ago");

    let at_29_days =
        from_duration(29 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(at_29_days, "4주 전");

    let at_month =
        from_duration(30 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_month, "a month ago");

    let before_half =
        from_duration((40 * DAY_SECONDS) - 1, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(before_half, "1달 전");

    let at_half =
        from_duration(40 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(at_half, "1달 반 전");

    let at_next_month =
        from_duration(55 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_next_month, "2 months ago");
}

#[test]
fn rounds_up_weeks_from_four_days() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    let ten_days =
        from_duration(10 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(ten_days, "1주 전");

    let eleven_days =
        from_duration(11 * DAY_SECONDS, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(eleven_days, "2주 전");

    let seventeen_days =
        from_duration(17 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(seventeen_days, "2 weeks ago");

    let eighteen_days =
        from_duration(18 * DAY_SECONDS, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(eighteen_days, "3 weeks ago");
}

#[test]
fn rounds_up_years_from_ten_months() {
    const MONTH_SECONDS: i64 = 30 * 24 * 60 * 60;
    const DAY_SECONDS: i64 = 24 * 60 * 60;
    let ten_months = 10 * MONTH_SECONDS;
    let eleven_half_months = (11 * MONTH_SECONDS) + (15 * DAY_SECONDS);
    let first_year_start = 350 * DAY_SECONDS;
    let sixteen_months = 16 * MONTH_SECONDS;
    let thirty_four_months = 34 * MONTH_SECONDS;

    let at_ten_months =
        from_duration(ten_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_ten_months, "10 months ago");

    let at_eleven_half_months =
        from_duration(eleven_half_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_eleven_half_months, "11 months and a half ago");

    let at_first_year_start =
        from_duration(first_year_start, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_first_year_start, "a year ago");

    let before_first_year_start =
        from_duration(first_year_start - 1, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(before_first_year_start, "11 months and a half ago");

    let at_first_year_start_ko =
        from_duration(first_year_start, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(at_first_year_start_ko, "1년 전");

    let before_first_year_start_ko =
        from_duration(first_year_start - 1, Locale::Ko, Direction::Past).expect("valid duration");
    assert_eq!(before_first_year_start_ko, "11달 반 전");

    let at_sixteen_months =
        from_duration(sixteen_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_sixteen_months, "a year and a half ago");

    let before_thirty_four =
        from_duration(thirty_four_months - 1, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(before_thirty_four, "2 years and a half ago");

    let at_thirty_four =
        from_duration(thirty_four_months, Locale::En, Direction::Past).expect("valid duration");
    assert_eq!(at_thirty_four, "3 years ago");

    let at_thirty_four_ko =
        from_duration(thirty_four_months, Locale::Ko, Direction::Past).expect("valid duration");
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

#[test]
fn range_of_supports_korean_numeric_and_native_month_labels() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    // Literal/lenient semantics: "N 달" -> [N * 30d, (N+1) * 30d - 1].
    let numeric = range_of("2달 전", Locale::Ko).expect("valid expression");
    assert_eq!(numeric.start_seconds, -((90 * DAY_SECONDS) - 1));
    assert_eq!(numeric.end_seconds, -(60 * DAY_SECONDS));

    let native = range_of("두 달 전", Locale::Ko).expect("valid expression");
    assert_eq!(native, numeric);
}

#[test]
fn range_of_supports_english_half_expression() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    let range = range_of("a month and a half ago", Locale::En).expect("valid expression");
    assert_eq!(range.start_seconds, -((60 * DAY_SECONDS) - 1));
    assert_eq!(range.end_seconds, -(45 * DAY_SECONDS));
}

#[test]
fn range_of_uses_literal_bucket_for_year() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    // 1 year bucket = 12 * 30 = 360 days. Lenient semantics ignore the render-side
    // 350-day first-label policy.
    let before_year = range_of("11 months and a half ago", Locale::En).expect("valid expression");
    assert_eq!(before_year.start_seconds, -((360 * DAY_SECONDS) - 1));
    assert_eq!(before_year.end_seconds, -(345 * DAY_SECONDS));

    let at_year = range_of("a year ago", Locale::En).expect("valid expression");
    assert_eq!(at_year.start_seconds, -((720 * DAY_SECONDS) - 1));
    assert_eq!(at_year.end_seconds, -(360 * DAY_SECONDS));
}

#[test]
fn range_of_supports_future_direction() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    let range = range_of("2달 후", Locale::Ko).expect("valid expression");
    assert_eq!(range.start_seconds, 60 * DAY_SECONDS);
    assert_eq!(range.end_seconds, (90 * DAY_SECONDS) - 1);
}

#[test]
fn range_of_accepts_units_outside_render_policy() {
    const DAY_SECONDS: i64 = 24 * 60 * 60;

    // "24일 전" — Day base above the render-side selection cap is now accepted.
    let twenty_four_days = range_of("24일 전", Locale::Ko).expect("lenient day accepted");
    assert_eq!(twenty_four_days.start_seconds, -((25 * DAY_SECONDS) - 1));
    assert_eq!(twenty_four_days.end_seconds, -(24 * DAY_SECONDS));

    // "60시간 전" — Hour base above selection cap.
    let sixty_hours = range_of("60 hours ago", Locale::En).expect("lenient hour accepted");
    assert_eq!(sixty_hours.start_seconds, -((61 * 3600) - 1));
    assert_eq!(sixty_hours.end_seconds, -(60 * 3600));

    // "2주 반 전" — half on a unit that has no render-side half threshold.
    let two_and_half_weeks = range_of("2주 반 전", Locale::Ko).expect("half on weeks accepted");
    assert_eq!(
        two_and_half_weeks.start_seconds,
        -((3 * 7 * DAY_SECONDS) - 1)
    );
    // 2 weeks + half week = 14d + 3d12h
    assert_eq!(
        two_and_half_weeks.end_seconds,
        -(2 * 7 * DAY_SECONDS + (7 * DAY_SECONDS) / 2)
    );
}

#[test]
fn range_of_rejects_unsupported_or_invalid_expression() {
    // base < 1 is invalid for every unit.
    let zero_base = range_of("0 days ago", Locale::En).expect_err("must reject base < 1");
    assert!(matches!(zero_base, OotdError::InvalidExpression(_)));

    // Half on seconds collapses (bucket / 2 == 0), so it is rejected.
    let half_second =
        range_of("2 seconds and a half ago", Locale::En).expect_err("must reject half-second");
    assert!(matches!(half_second, OotdError::InvalidExpression(_)));

    // Pure garbage stays rejected.
    let garbage = range_of("not a time expression", Locale::En).expect_err("must reject garbage");
    assert!(matches!(garbage, OotdError::InvalidExpression(_)));
}

#[test]
fn range_of_supports_daypart_expression_with_anchor() {
    let anchor = "2024-01-25T23:30:00+09:00";
    let range = range_of_at_rfc3339("어제 밤", Locale::Ko, anchor)
        .expect("daypart expression should parse");
    let resolved = range.resolve_at_rfc3339(anchor).expect("must resolve");
    let (start, end) = resolved.to_rfc3339_pair();

    assert_eq!(start, "2024-01-24T20:00:00+09:00");
    assert_eq!(end, "2024-01-24T23:59:59+09:00");
}

#[test]
fn range_of_supports_english_daypart_alias_with_anchor() {
    let anchor = "2024-01-25T23:30:00+09:00";
    let range = range_of_at_rfc3339("earlier tonight", Locale::En, anchor)
        .expect("daypart expression should parse");
    let resolved = range.resolve_at_rfc3339(anchor).expect("must resolve");
    let (start, end) = resolved.to_rfc3339_pair();

    assert_eq!(start, "2024-01-25T20:00:00+09:00");
    assert_eq!(end, "2024-01-25T23:59:59+09:00");
}

#[test]
fn range_of_contains_exact_bucket_boundary_for_each_unit() {
    // Lenient range_of is no longer the exact inverse of render. It always covers
    // `[N*bucket, (N+1)*bucket - 1]` for `N <unit>`, so the lower boundary must be
    // contained in the parsed range.
    let cases: &[(&str, Locale, i64)] = &[
        ("1년 전", Locale::Ko, -(360 * 24 * 60 * 60)),
        ("1달 전", Locale::Ko, -(30 * 24 * 60 * 60)),
        ("1주 전", Locale::Ko, -(7 * 24 * 60 * 60)),
        ("1일 전", Locale::Ko, -(24 * 60 * 60)),
        ("1시간 전", Locale::Ko, -3600),
        ("1분 전", Locale::Ko, -60),
        ("1초 전", Locale::Ko, -1),
    ];

    for (expression, locale, signed_boundary) in cases {
        let range = range_of(expression, *locale).expect("must parse expression");
        assert!(
            range.start_seconds <= *signed_boundary && *signed_boundary <= range.end_seconds,
            "expr={expression}, boundary={signed_boundary}, range=({}, {})",
            range.start_seconds,
            range.end_seconds
        );
    }
}

#[test]
fn duration_range_resolve_at_uses_given_anchor_datetime() {
    let range = range_of("두 달 전", Locale::Ko).expect("valid expression");
    let anchor =
        DateTime::parse_from_rfc3339("2026-04-29T12:00:00+09:00").expect("valid anchor datetime");

    let resolved = range.resolve_at(anchor).expect("must resolve");
    let (start, end) = resolved.to_rfc3339_pair();

    // Lenient "두 달 전" = [60d, 90d-1s] past, anchored at 2026-04-29 12:00 +09:00.
    assert_eq!(start, "2026-01-29T12:00:01+09:00");
    assert_eq!(end, "2026-02-28T12:00:00+09:00");
}

#[test]
fn duration_range_resolve_at_rfc3339_matches_resolve_at() {
    let range = range_of("두 달 전", Locale::Ko).expect("valid expression");
    let anchor =
        DateTime::parse_from_rfc3339("2026-04-29T12:00:00+09:00").expect("valid anchor datetime");

    let from_dt = range.resolve_at(anchor).expect("must resolve");
    let from_str = range
        .resolve_at_rfc3339("2026-04-29T12:00:00+09:00")
        .expect("must resolve");

    assert_eq!(from_dt, from_str);
}

#[test]
fn duration_range_rejects_invalid_bounds() {
    let invalid = DurationRange {
        start_seconds: 10,
        end_seconds: 9,
    };
    let anchor =
        DateTime::parse_from_rfc3339("2026-04-29T12:00:00+09:00").expect("valid anchor datetime");

    let err = invalid.resolve_at(anchor).expect_err("must fail");
    assert!(matches!(
        err,
        OotdError::InvalidDurationRangeBounds {
            start_seconds: 10,
            end_seconds: 9
        }
    ));
}

#[test]
fn duration_range_resolve_now_preserves_span() {
    let range = range_of("두 달 전", Locale::Ko).expect("valid expression");
    let resolved = range.resolve_now().expect("must resolve now");
    let span_seconds = resolved
        .end
        .signed_duration_since(resolved.start)
        .num_seconds();

    assert_eq!(span_seconds, range.end_seconds - range.start_seconds);
}

#[test]
fn extract_expressions_finds_korean_patterns() {
    let q = "지난 두 달 전 로그랑 어제 낮 결제 내역 보여줘";
    let found = extract_expressions(q, Locale::Ko);
    let texts = found.iter().map(|it| it.text.as_str()).collect::<Vec<_>>();

    assert_eq!(texts, vec!["두 달 전", "어제 낮"]);
}

#[test]
fn extract_expressions_finds_english_patterns() {
    let q = "show errors from 2 months ago and yesterday afternoon";
    let found = extract_expressions(q, Locale::En);
    let texts = found.iter().map(|it| it.text.as_str()).collect::<Vec<_>>();

    assert_eq!(texts, vec!["2 months ago", "yesterday afternoon"]);
}

#[test]
fn extract_expressions_filters_invalid_matches() {
    // Lenient range_of still rejects base < 1 ("0 days ago"), so the detector
    // continues to filter that candidate out while keeping the valid one.
    let q = "0 days ago and a week ago";
    let found = extract_expressions(q, Locale::En);
    let texts = found.iter().map(|it| it.text.as_str()).collect::<Vec<_>>();

    assert_eq!(texts, vec!["a week ago"]);
}

#[test]
fn extract_expressions_now_accepts_half_on_weeks() {
    // Previously "2 weeks and a half ago" was rejected by range_of; under lenient
    // semantics it is a valid candidate alongside "a week ago".
    let q = "2 weeks and a half ago and a week ago";
    let found = extract_expressions(q, Locale::En);
    let texts = found.iter().map(|it| it.text.as_str()).collect::<Vec<_>>();

    assert_eq!(texts, vec!["2 weeks and a half ago", "a week ago"]);
}
