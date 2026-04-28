use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    Ko,
}

impl FromStr for Locale {
    type Err = OotdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "en" | "en_us" | "en-us" | "en_us.utf-8" | "en-us.utf-8" => Ok(Self::En),
            "ko" | "ko_kr" | "ko-kr" | "ko_kr.utf-8" | "ko-kr.utf-8" => Ok(Self::Ko),
            _ => Err(OotdError::UnsupportedLocale(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RenderOptions {
    pub ko_native_numerals: bool,
}

#[derive(Debug, Error)]
pub enum OotdError {
    #[error("unsupported locale: {0}")]
    UnsupportedLocale(String),
    #[error("invalid RFC3339 datetime: {0}")]
    InvalidDatetime(String),
    #[error("negative duration is not allowed: {0}")]
    NegativeDuration(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnitKind {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Fraction {
    pub(crate) num: i64,
    pub(crate) den: i64,
}

impl Fraction {
    pub(crate) const fn of(self, total: i64) -> i64 {
        total * self.num / self.den
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RoundUpMode {
    None,
    RemainderThreshold,
    EarlyShiftByRoundUp,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct UnitPolicy {
    // Unit granularity used to split base/remainder.
    pub(crate) bucket_seconds: i64,
    // Lower bound where this unit becomes eligible as the main label.
    pub(crate) first_label_start_seconds: i64,
    // "N and a half" threshold.
    pub(crate) half_threshold: Option<Fraction>,
    // N -> N+1 threshold.
    pub(crate) round_up_threshold: Option<Fraction>,
    pub(crate) round_up_mode: RoundUpMode,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Unit {
    pub(crate) kind: UnitKind,
    pub(crate) name_en_singular: &'static str,
    pub(crate) name_en_plural: &'static str,
    pub(crate) name_ko: &'static str,
    // Unit selection threshold.
    pub(crate) seconds: i64,
    pub(crate) policy: UnitPolicy,
}

const SECOND: i64 = 1;
const MINUTE: i64 = 60 * SECOND;
const HOUR: i64 = 60 * MINUTE;
const DAY: i64 = 24 * HOUR;
const WEEK: i64 = 7 * DAY;
const MONTH: i64 = 30 * DAY;
// Policy consistency: year uses the same 30-day month bucket basis (12 * 30 = 360 days),
// not calendar-year length.
const YEAR_BUCKET: i64 = 12 * MONTH;

const ONE_THIRD: Fraction = Fraction { num: 1, den: 3 };
const ONE_HALF: Fraction = Fraction { num: 1, den: 2 };
const FIVE_SIXTHS: Fraction = Fraction { num: 5, den: 6 };
const FOUR_SEVENTHS: Fraction = Fraction { num: 4, den: 7 };

pub(crate) const UNITS: [Unit; 7] = [
    Unit {
        kind: UnitKind::Year,
        name_en_singular: "year",
        name_en_plural: "years",
        name_ko: "년",
        seconds: YEAR_BUCKET,
        // Year policy:
        // - first year starts at 350 days
        // - half from +4 months
        // - round-up from +10 months
        policy: UnitPolicy {
            bucket_seconds: YEAR_BUCKET,
            first_label_start_seconds: 350 * DAY,
            half_threshold: Some(ONE_THIRD),
            round_up_threshold: Some(FIVE_SIXTHS),
            round_up_mode: RoundUpMode::RemainderThreshold,
        },
    },
    Unit {
        kind: UnitKind::Month,
        name_en_singular: "month",
        name_en_plural: "months",
        name_ko: "달",
        seconds: MONTH,
        // Month policy:
        // - first month starts at 30 days (25~29 days remains week-level)
        // - half from +15 days
        // - round-up from +25 days (5/6), encoded as early-shift.
        policy: UnitPolicy {
            bucket_seconds: MONTH,
            first_label_start_seconds: MONTH,
            half_threshold: Some(ONE_HALF),
            round_up_threshold: Some(FIVE_SIXTHS),
            round_up_mode: RoundUpMode::EarlyShiftByRoundUp,
        },
    },
    Unit {
        kind: UnitKind::Week,
        name_en_singular: "week",
        name_en_plural: "weeks",
        name_ko: "주",
        seconds: WEEK,
        // Week policy: no half, round-up at +4 days (4/7).
        policy: UnitPolicy {
            bucket_seconds: WEEK,
            first_label_start_seconds: WEEK,
            half_threshold: None,
            round_up_threshold: Some(FOUR_SEVENTHS),
            round_up_mode: RoundUpMode::RemainderThreshold,
        },
    },
    Unit {
        kind: UnitKind::Day,
        name_en_singular: "day",
        name_en_plural: "days",
        name_ko: "일",
        seconds: DAY,
        policy: UnitPolicy {
            bucket_seconds: DAY,
            first_label_start_seconds: DAY,
            half_threshold: None,
            round_up_threshold: None,
            round_up_mode: RoundUpMode::None,
        },
    },
    Unit {
        kind: UnitKind::Hour,
        name_en_singular: "hour",
        name_en_plural: "hours",
        name_ko: "시간",
        seconds: HOUR,
        // Hour policy: half from +20m (1/3), round-up from +50m (5/6).
        policy: UnitPolicy {
            bucket_seconds: HOUR,
            first_label_start_seconds: HOUR,
            half_threshold: Some(ONE_THIRD),
            round_up_threshold: Some(FIVE_SIXTHS),
            round_up_mode: RoundUpMode::RemainderThreshold,
        },
    },
    Unit {
        kind: UnitKind::Minute,
        name_en_singular: "minute",
        name_en_plural: "minutes",
        name_ko: "분",
        seconds: MINUTE,
        policy: UnitPolicy {
            bucket_seconds: MINUTE,
            first_label_start_seconds: MINUTE,
            half_threshold: None,
            round_up_threshold: None,
            round_up_mode: RoundUpMode::None,
        },
    },
    Unit {
        kind: UnitKind::Second,
        name_en_singular: "second",
        name_en_plural: "seconds",
        name_ko: "초",
        seconds: SECOND,
        policy: UnitPolicy {
            bucket_seconds: SECOND,
            first_label_start_seconds: SECOND,
            half_threshold: None,
            round_up_threshold: None,
            round_up_mode: RoundUpMode::None,
        },
    },
];
