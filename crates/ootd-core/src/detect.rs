use regex::Regex;
use chrono::{FixedOffset, TimeZone};

use crate::expression::range_of_impl;
use crate::types::Locale;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionCandidate {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

pub(crate) fn extract_expressions_impl(input: &str, locale: Locale) -> Vec<ExpressionCandidate> {
    let mut out = Vec::new();
    for mat in locale_patterns(locale)
        .iter()
        .flat_map(|re| re.find_iter(input))
        .collect::<Vec<_>>()
    {
        let text = mat.as_str().trim().to_string();
        if text.is_empty() {
            continue;
        }
        if range_of_impl(&text, locale, Some(detection_anchor())).is_ok() {
            out.push(ExpressionCandidate {
                start: mat.start(),
                end: mat.end(),
                text,
            });
        }
    }

    // Prefer longer matches first, then stable left-to-right selection.
    out.sort_by(|a, b| {
        let a_len = a.end - a.start;
        let b_len = b.end - b.start;
        b_len.cmp(&a_len).then(a.start.cmp(&b.start))
    });

    let mut selected: Vec<ExpressionCandidate> = Vec::new();
    for candidate in out {
        if selected
            .iter()
            .any(|it| overlaps((it.start, it.end), (candidate.start, candidate.end)))
        {
            continue;
        }
        selected.push(candidate);
    }

    selected.sort_by_key(|it| it.start);
    selected
}

fn detection_anchor() -> chrono::DateTime<FixedOffset> {
    FixedOffset::east_opt(0)
        .expect("zero offset must be valid")
        .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
        .single()
        .expect("fixed anchor datetime must be valid")
}

fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn locale_patterns(locale: Locale) -> Vec<Regex> {
    match locale {
        Locale::Ko => vec![
            Regex::new(r"(?u)(?:[0-9]{1,3}|[가-힣]{1,8})\s*(?:년|달|주|일|시간|분|초)(?:\s*반)?\s*(?:전|후)")
                .expect("valid ko duration regex"),
            Regex::new(r"(?u)(?:어제|오늘|내일)\s*(?:새벽|아침|낮|저녁|밤)")
                .expect("valid ko daypart regex"),
        ],
        Locale::En => vec![
            Regex::new(
                r"(?i)\b(?:a|an|\d+)\s+(?:year|years|month|months|week|weeks|day|days|hour|hours|minute|minutes|second|seconds)(?:\s+and\s+a\s+half)?\s+(?:ago|later)\b",
            )
            .expect("valid en duration regex"),
            Regex::new(
                r"(?i)\b(?:yesterday|this|tomorrow)\s+(?:dawn|morning|afternoon|evening|night)\b",
            )
            .expect("valid en daypart regex"),
            Regex::new(r"(?i)\b(?:last night|earlier tonight|tonight)\b")
                .expect("valid en alias regex"),
        ],
    }
}
