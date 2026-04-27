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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Unit {
    pub(crate) kind: UnitKind,
    pub(crate) name_en_singular: &'static str,
    pub(crate) name_en_plural: &'static str,
    pub(crate) name_ko: &'static str,
    pub(crate) seconds: i64,
    pub(crate) allow_half: bool,
    pub(crate) half_threshold_num: i64,
    pub(crate) half_threshold_den: i64,
}

pub(crate) const UNITS: [Unit; 7] = [
    Unit {
        kind: UnitKind::Year,
        name_en_singular: "year",
        name_en_plural: "years",
        name_ko: "년",
        seconds: 365 * 24 * 60 * 60,
        allow_half: true,
        half_threshold_num: 1,
        half_threshold_den: 3,
    },
    Unit {
        kind: UnitKind::Month,
        name_en_singular: "month",
        name_en_plural: "months",
        name_ko: "달",
        seconds: 30 * 24 * 60 * 60,
        allow_half: true,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
    Unit {
        kind: UnitKind::Week,
        name_en_singular: "week",
        name_en_plural: "weeks",
        name_ko: "주",
        seconds: 7 * 24 * 60 * 60,
        allow_half: false,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
    Unit {
        kind: UnitKind::Day,
        name_en_singular: "day",
        name_en_plural: "days",
        name_ko: "일",
        seconds: 24 * 60 * 60,
        allow_half: false,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
    Unit {
        kind: UnitKind::Hour,
        name_en_singular: "hour",
        name_en_plural: "hours",
        name_ko: "시간",
        seconds: 60 * 60,
        allow_half: true,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
    Unit {
        kind: UnitKind::Minute,
        name_en_singular: "minute",
        name_en_plural: "minutes",
        name_ko: "분",
        seconds: 60,
        allow_half: false,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
    Unit {
        kind: UnitKind::Second,
        name_en_singular: "second",
        name_en_plural: "seconds",
        name_ko: "초",
        seconds: 1,
        allow_half: false,
        half_threshold_num: 1,
        half_threshold_den: 2,
    },
];
