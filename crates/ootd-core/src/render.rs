use crate::duration_policy::{resolve_duration_bucket, unit_by_kind};
use crate::locale::{render_duration_en, render_duration_ko};
use crate::types::{Direction, Locale, RenderOptions};

pub(crate) fn render_duration_nonnegative(
    seconds: i64,
    locale: Locale,
    direction: Direction,
    options: RenderOptions,
) -> String {
    let bucket = resolve_duration_bucket(seconds);
    let unit = unit_by_kind(bucket.kind);

    match locale {
        Locale::En => render_duration_en(bucket.base, bucket.has_half, unit, direction),
        Locale::Ko => render_duration_ko(bucket.base, bucket.has_half, unit, direction, options),
    }
}
