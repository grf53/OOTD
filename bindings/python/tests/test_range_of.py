from datetime import datetime, timedelta

import ootd


def test_range_of_duration_expression():
    out = ootd.range_of("두 달 전", "ko")
    assert out.to_dict() == {
        "start": timedelta(seconds=-7_775_999),
        "end": timedelta(seconds=-5_184_000),
    }
    assert out.start == timedelta(seconds=-7_775_999)
    assert out.end == timedelta(seconds=-5_184_000)


def test_duration_range_resolve_at_supports_daypart_anchor():
    out = ootd.range_of("어제 밤", "ko").resolve_at("2024-01-25T23:30:00+09:00")
    assert isinstance(out, ootd.TimestampRange)
    assert out.start == datetime.fromisoformat("2024-01-24T20:00:00+09:00")
    assert out.end == datetime.fromisoformat("2024-01-24T23:59:59+09:00")


def test_duration_range_resolve_at_with_anchor():
    out = ootd.range_of("두 달 전", "ko").resolve_at("2026-04-29T12:00:00+09:00")
    assert isinstance(out, ootd.TimestampRange)
    assert out.start == datetime.fromisoformat("2026-01-29T12:00:01+09:00")
    assert out.end == datetime.fromisoformat("2026-02-28T12:00:00+09:00")


def test_extract_expressions():
    out = ootd.extract_expressions("지난 두 달 전 로그랑 어제 낮 결제", "ko")
    assert [x["text"] for x in out] == ["두 달 전", "어제 낮"]
