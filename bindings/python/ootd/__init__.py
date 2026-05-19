"""Python-friendly OOTD wrapper module.

Public functions are pure Python callables delegating to the native extension,
which allows straightforward monkeypatching in tests.
"""

from datetime import datetime, timedelta, timezone
from typing import Any, Callable, List, Literal, Mapping, Optional, TypedDict, Union

from . import _native

__all__ = [
    "between",
    "from_duration",
    "range_of",
    "extract_expressions",
    "DurationRange",
    "TimestampRange",
]


DateLike = Union[str, datetime]
DurationLike = Union[int, timedelta]
Locale = Literal["en", "ko"]


class DurationRangeDict(TypedDict):
    start: timedelta
    end: timedelta


class TimestampRangeDict(TypedDict):
    start: datetime
    end: datetime


class ExpressionCandidateDict(TypedDict):
    start: int
    end: int
    text: str


BetweenImpl = Callable[[DateLike, DateLike, str, bool], str]
FromDurationImpl = Callable[[DurationLike, bool, str, bool], str]
RangeOfImpl = Callable[[str, str], Mapping[str, Any]]
ExtractExpressionsImpl = Callable[[str, str], List[Mapping[str, Any]]]
ResolveDurationRangeImpl = Callable[[int, int, Optional[str]], Mapping[str, Any]]
RangeOfTimestampsImpl = Callable[[str, str, Optional[str]], Mapping[str, Any]]

# Exposed indirection points for monkeypatch/testing.
_between_impl: BetweenImpl = _native.between
_from_duration_impl: FromDurationImpl = _native.from_duration
_range_of_impl: RangeOfImpl = _native.range_of
_extract_expressions_impl: ExtractExpressionsImpl = _native.extract_expressions
_resolve_duration_range_impl: ResolveDurationRangeImpl = _native.resolve_duration_range
_range_of_timestamps_impl: RangeOfTimestampsImpl = _native.range_of_timestamps


class TimestampRange:
    def __init__(
        self,
        start: DateLike,
        end: DateLike,
        raw: Optional[Mapping[str, Any]] = None,
    ):
        self.start = _coerce_to_datetime(start)
        self.end = _coerce_to_datetime(end)
        if raw is None:
            self._raw = {
                "start": self.start,
                "end": self.end,
            }
        else:
            self._raw = dict(raw)
            self._raw["start"] = self.start
            self._raw["end"] = self.end

    @classmethod
    def from_mapping(cls, out: Mapping[str, Any]) -> "TimestampRange":
        start = out.get("start", out.get("start_rfc3339"))
        end = out.get("end", out.get("end_rfc3339"))
        if not isinstance(start, (str, datetime)) or not isinstance(end, (str, datetime)):
            raise ValueError("timestamp range must include datetime-like start/end")
        return cls(
            start,
            end,
            out,
        )

    def to_dict(self) -> TimestampRangeDict:
        return {
            "start": self.start,
            "end": self.end,
        }

    def __getitem__(self, key: str) -> Any:
        return self._raw[key]

    def get(self, key: str, default: Optional[Any] = None) -> Any:
        return self._raw.get(key, default)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, TimestampRange):
            return self.to_dict() == other.to_dict()
        if isinstance(other, dict):
            return other == self._raw or other == self.to_dict()
        return False

    def __repr__(self) -> str:
        return (
            "TimestampRange("
            f"start={self.start.isoformat()!r}, "
            f"end={self.end.isoformat()!r}"
            ")"
        )


class DurationRange:
    def __init__(
        self,
        start: DurationLike,
        end: DurationLike,
        expression: Optional[str] = None,
        locale: Locale = "en",
    ):
        self.start = _coerce_to_timedelta(start)
        self.end = _coerce_to_timedelta(end)
        self._start_seconds = int(self.start.total_seconds())
        self._end_seconds = int(self.end.total_seconds())
        self._expression = expression
        self._locale = locale

    def resolve_at(self, anchor_rfc3339: Optional[DateLike] = None) -> TimestampRange:
        anchor = _coerce_to_rfc3339(anchor_rfc3339) if anchor_rfc3339 is not None else None
        if self._expression is not None:
            out = _range_of_timestamps_impl(self._expression, self._locale, anchor)
            return TimestampRange.from_mapping(out)

        out = _resolve_duration_range_impl(self._start_seconds, self._end_seconds, anchor)
        return TimestampRange.from_mapping(out)

    def to_dict(self) -> DurationRangeDict:
        return {
            "start": self.start,
            "end": self.end,
        }

    def __eq__(self, other: object) -> bool:
        if isinstance(other, DurationRange):
            return self.start == other.start and self.end == other.end
        if isinstance(other, dict):
            return other == self.to_dict()
        return False

    def __repr__(self) -> str:
        return f"DurationRange(start={self.start}, end={self.end})"


def between(
    start_rfc3339: DateLike,
    end_rfc3339: DateLike,
    locale: Locale = "en",
    use_native_ko_number: bool = False,
) -> str:
    return _between_impl(start_rfc3339, end_rfc3339, locale, use_native_ko_number)


def from_duration(
    seconds: DurationLike,
    is_future: bool = False,
    locale: Locale = "en",
    use_native_ko_number: bool = False,
) -> str:
    return _from_duration_impl(seconds, is_future, locale, use_native_ko_number)


def range_of(
    expression: str,
    locale: Locale = "en",
) -> DurationRange:
    out = _range_of_impl(expression, locale)
    start = out.get("start", out.get("start_seconds"))
    end = out.get("end", out.get("end_seconds"))
    if not isinstance(start, int) or not isinstance(end, int):
        raise ValueError("native range_of returned invalid range object")
    return DurationRange(start, end, expression, locale)


def extract_expressions(input: str, locale: Locale = "en") -> List[ExpressionCandidateDict]:
    out = _extract_expressions_impl(input, locale)
    if not isinstance(out, list):
        raise ValueError("native extract_expressions returned invalid value")

    parsed: List[ExpressionCandidateDict] = []
    for item in out:
        if not isinstance(item, Mapping):
            raise ValueError("native extract_expressions returned invalid candidate")
        start = item.get("start")
        end = item.get("end")
        text = item.get("text")
        if not isinstance(start, int) or not isinstance(end, int) or not isinstance(text, str):
            raise ValueError("native extract_expressions returned invalid candidate")
        parsed.append({"start": start, "end": end, "text": text})
    return parsed


def _coerce_to_rfc3339(value: DateLike) -> str:
    if isinstance(value, str):
        return value
    if value.utcoffset() is None:
        raise ValueError("naive datetime is not supported; pass timezone-aware datetime")
    return value.isoformat()


def _coerce_to_timedelta(value: DurationLike) -> timedelta:
    if isinstance(value, timedelta):
        return value
    return timedelta(seconds=int(value))


def _coerce_to_datetime(value: DateLike) -> datetime:
    if isinstance(value, datetime):
        if value.utcoffset() is None:
            raise ValueError("naive datetime is not supported; pass timezone-aware datetime")
        return value
    text = value.strip()
    if text.endswith("Z"):
        text = f"{text[:-1]}+00:00"
    out = datetime.fromisoformat(text)
    if out.tzinfo is None:
        return out.replace(tzinfo=timezone.utc)
    return out
