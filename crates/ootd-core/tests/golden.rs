use ootd_core::{between_rfc3339_with_options, Locale, RenderOptions};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct BetweenCase {
    name: String,
    locale: String,
    start: String,
    end: String,
    expected: String,
    #[serde(default)]
    use_native_ko_number: bool,
}

fn load_cases() -> Vec<BetweenCase> {
    let raw = include_str!("../../../tests/between_rfc3339_cases.json");
    serde_json::from_str(raw).expect("between_rfc3339_cases.json must be valid")
}

#[test]
fn golden_between_rfc3339_cases() {
    for case in load_cases() {
        let locale = Locale::from_str(&case.locale).expect("case locale must be valid");
        let options = RenderOptions {
            ko_native_numerals: case.use_native_ko_number,
        };
        let output = between_rfc3339_with_options(&case.start, &case.end, locale, options)
            .expect("must parse");
        assert_eq!(
            output, case.expected,
            "case failed: {} ({} -> {}, locale={}, use_native_ko_number={})",
            case.name, case.start, case.end, case.locale, case.use_native_ko_number
        );
    }
}
