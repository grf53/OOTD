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
