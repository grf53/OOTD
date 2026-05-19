from pathlib import Path


def test_generated_pyi_exists_and_has_signatures():
    stub = Path(__file__).resolve().parents[1] / "ootd" / "__init__.pyi"
    text = stub.read_text(encoding="utf-8")

    assert "def between(" in text
    assert "def from_duration(" in text
    assert "def range_of(" in text
    assert "def extract_expressions(" in text
    assert "def resolve_at(" in text
    assert "use_native_ko_number" in text
    assert 'Locale = Literal["en", "ko"]' in text
    assert "DurationLike = Union[int, timedelta]" in text
    assert "class DurationRangeDict(TypedDict):" in text
    assert "class DurationRange:" in text
    assert "class TimestampRangeDict(TypedDict):" in text
    assert "class TimestampRange:" in text
    assert "start: timedelta" in text
    assert "start: datetime" in text
