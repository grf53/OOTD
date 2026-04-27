from datetime import timedelta

import pytest

import ootd


def test_from_duration_accepts_float_by_truncating_to_int():
    expected = ootd.from_duration(3599, False, "en")
    out = ootd.from_duration(3599.9, False, "en")
    assert out == expected


def test_from_duration_accepts_timedelta_by_total_seconds():
    expected = ootd.from_duration(3599, False, "en")
    out = ootd.from_duration(timedelta(seconds=3599.9), False, "en")
    assert out == expected


def test_from_duration_rejects_non_finite_float():
    with pytest.raises(ValueError, match="finite"):
        ootd.from_duration(float("inf"), False, "en")


def test_from_duration_negative_timedelta_still_errors():
    with pytest.raises(ValueError, match="negative duration"):
        ootd.from_duration(timedelta(seconds=-1), False, "en")
