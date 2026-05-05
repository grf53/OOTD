from datetime import datetime, timedelta

import ootd


def test_between_monkeypatch(monkeypatch):
    def fake_between(start, end, locale, use_native_ko_number=False):
        return f"patched:{locale}:{use_native_ko_number}"

    monkeypatch.setattr(ootd, "_between_impl", fake_between)
    assert ootd.between("2024-01-25T11:00:00Z", "2024-01-25T13:31:43Z", "ko", True) == "patched:ko:True"


def test_from_duration_monkeypatch(monkeypatch):
    def fake_from_duration(seconds, is_future=False, locale="en", use_native_ko_number=False):
        return f"patched:{seconds}:{is_future}:{locale}:{use_native_ko_number}"

    monkeypatch.setattr(ootd, "_from_duration_impl", fake_from_duration)
    assert ootd.from_duration(123, True, "ko", True) == "patched:123:True:ko:True"


def test_range_of_monkeypatch(monkeypatch):
    def fake_range_of(expression, locale="en"):
        return {
            "start": -10,
            "end": -1,
            "meta": f"{expression}:{locale}",
        }

    monkeypatch.setattr(ootd, "_range_of_impl", fake_range_of)
    out = ootd.range_of("두 달 전", "ko")
    assert out.to_dict() == {"start": timedelta(seconds=-10), "end": timedelta(seconds=-1)}


def test_resolve_duration_range_monkeypatch(monkeypatch):
    def fake_resolve_duration_range(start_seconds, end_seconds, anchor_rfc3339=None):
        return {
            "start": "2026-01-01T00:00:00+00:00",
            "end": "2026-01-02T00:00:00+00:00",
            "meta": f"{start_seconds}:{end_seconds}:{anchor_rfc3339}",
        }

    monkeypatch.setattr(ootd, "_resolve_duration_range_impl", fake_resolve_duration_range)
    out = ootd.DurationRange(-6047999, -4752000).resolve_at("2026-04-29T12:00:00+09:00")
    assert out["meta"] == "-6047999:-4752000:2026-04-29T12:00:00+09:00"
    assert out.start == datetime.fromisoformat("2026-01-01T00:00:00+00:00")
    assert out.end == datetime.fromisoformat("2026-01-02T00:00:00+00:00")
