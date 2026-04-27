from pathlib import Path


def test_generated_pyi_exists_and_has_signatures():
    stub = Path(__file__).resolve().parents[1] / "ootd" / "__init__.pyi"
    text = stub.read_text(encoding="utf-8")

    assert "def between(" in text
    assert "def from_duration(" in text
    assert "use_native_ko_number" in text
    assert 'Locale = Literal["en", "ko"]' in text
    assert "DurationLike = Union[int, timedelta]" in text
