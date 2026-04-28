use crate::types::{Fraction, RoundUpMode, Unit, UnitKind, UnitPolicy, UNITS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DurationBucket {
    pub(crate) kind: UnitKind,
    pub(crate) base: i64,
    pub(crate) has_half: bool,
}

pub(crate) fn resolve_duration_bucket(seconds: i64) -> DurationBucket {
    debug_assert!(seconds >= 0, "duration must be non-negative");

    resolve_year_bucket(seconds)
        .or_else(|| resolve_month_bucket(seconds))
        .unwrap_or_else(|| resolve_standard_bucket(seconds))
}

pub(crate) fn unit_by_kind(kind: UnitKind) -> &'static Unit {
    UNITS
        .iter()
        .find(|unit| unit.kind == kind)
        .expect("all unit kinds must exist in UNITS")
}

fn resolve_year_bucket(seconds: i64) -> Option<DurationBucket> {
    let unit = unit_by_kind(UnitKind::Year);
    let policy = unit.policy;
    if seconds < policy.first_label_start_seconds {
        return None;
    }

    if seconds < unit.seconds {
        return Some(DurationBucket {
            kind: unit.kind,
            base: 1,
            has_half: false,
        });
    }

    let (mut base, remainder) = split_bucket(seconds, policy.bucket_seconds);
    let rounded = meets_threshold(remainder, policy.round_up_threshold, policy.bucket_seconds);
    if rounded {
        base += 1;
    }

    Some(DurationBucket {
        kind: unit.kind,
        base: base.max(1),
        has_half: !rounded
            && meets_threshold(remainder, policy.half_threshold, policy.bucket_seconds),
    })
}

fn resolve_month_bucket(seconds: i64) -> Option<DurationBucket> {
    let unit = unit_by_kind(UnitKind::Month);
    let policy = unit.policy;
    if seconds < policy.first_label_start_seconds {
        return None;
    }

    // Month uses early-shift semantics so that 5/6 round-up starts labels 1/6 early.
    let shifted = seconds.saturating_add(early_shift_seconds(policy)?);
    let (base, remainder) = split_bucket(shifted, policy.bucket_seconds);

    Some(DurationBucket {
        kind: unit.kind,
        base: base.max(1),
        has_half: meets_threshold(remainder, policy.half_threshold, policy.bucket_seconds),
    })
}

fn resolve_standard_bucket(seconds: i64) -> DurationBucket {
    let unit = select_unit(seconds);
    let policy = unit.policy;

    let (mut base, remainder) = split_bucket(seconds, policy.bucket_seconds);
    let rounded = should_round_up(policy, remainder);
    if rounded {
        base += 1;
    }

    DurationBucket {
        kind: unit.kind,
        base,
        has_half: !rounded
            && meets_threshold(remainder, policy.half_threshold, policy.bucket_seconds),
    }
}

fn select_unit(seconds: i64) -> &'static Unit {
    UNITS
        .iter()
        .find(|unit| seconds >= unit.seconds)
        .unwrap_or_else(|| unit_by_kind(UnitKind::Second))
}

fn split_bucket(total: i64, bucket_seconds: i64) -> (i64, i64) {
    if total < bucket_seconds {
        return (1, 0);
    }

    let base = (total / bucket_seconds).max(1);
    let remainder = total % bucket_seconds;
    (base, remainder)
}

fn threshold_seconds(ratio: Option<Fraction>, bucket_seconds: i64) -> Option<i64> {
    ratio.map(|f| f.of(bucket_seconds))
}

fn meets_threshold(remainder: i64, ratio: Option<Fraction>, bucket_seconds: i64) -> bool {
    if remainder <= 0 {
        return false;
    }

    threshold_seconds(ratio, bucket_seconds)
        .map(|threshold| remainder >= threshold)
        .unwrap_or(false)
}

fn should_round_up(policy: UnitPolicy, remainder: i64) -> bool {
    match policy.round_up_mode {
        RoundUpMode::None => false,
        RoundUpMode::RemainderThreshold => {
            meets_threshold(remainder, policy.round_up_threshold, policy.bucket_seconds)
        }
        // Early-shift policy is applied only in resolve_month_bucket.
        RoundUpMode::EarlyShiftByRoundUp => false,
    }
}

fn early_shift_seconds(policy: UnitPolicy) -> Option<i64> {
    if policy.round_up_mode != RoundUpMode::EarlyShiftByRoundUp {
        return Some(0);
    }

    threshold_seconds(policy.round_up_threshold, policy.bucket_seconds)
        .map(|threshold| policy.bucket_seconds.saturating_sub(threshold).max(0))
}

#[cfg(test)]
mod policy_tests {
    use super::*;

    #[test]
    fn policy_thresholds_are_valid() {
        for unit in &UNITS {
            let policy = unit.policy;

            if let Some(half) = policy.half_threshold {
                assert!(
                    half.num > 0 && half.den > 0,
                    "half threshold must be positive"
                );
                assert!(half.num < half.den, "half threshold must be < 1.0");
            }

            if let Some(round_up) = policy.round_up_threshold {
                assert!(
                    round_up.num > 0 && round_up.den > 0,
                    "round-up threshold must be positive"
                );
                assert!(
                    round_up.num < round_up.den,
                    "round-up threshold must be < 1.0"
                );
            }

            if let (Some(half), Some(round_up)) = (policy.half_threshold, policy.round_up_threshold)
            {
                assert!(
                    half.of(policy.bucket_seconds) <= round_up.of(policy.bucket_seconds),
                    "half threshold should not exceed round-up threshold for {:?}",
                    unit.kind
                );
            }

            if unit.kind == UnitKind::Year {
                assert!(
                    policy.first_label_start_seconds <= unit.seconds,
                    "year first label start should be before or at one year"
                );
            } else {
                assert!(
                    policy.first_label_start_seconds >= unit.seconds,
                    "first label start must be >= selection threshold for {:?}",
                    unit.kind
                );
            }
        }
    }
}
