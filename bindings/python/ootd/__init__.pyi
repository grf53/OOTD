from datetime import datetime, timedelta
from typing import List, Literal, Optional, TypedDict, Union

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

class TimestampRange:
    start: datetime
    end: datetime
    def __init__(
        self,
        start: DateLike,
        end: DateLike,
    ) -> None: ...
    def to_dict(self) -> TimestampRangeDict: ...
    def __getitem__(self, key: str) -> object: ...
    def get(self, key: str, default: Optional[object] = ...) -> Optional[object]: ...

class DurationRange:
    start: timedelta
    end: timedelta
    def __init__(
        self,
        start: DurationLike,
        end: DurationLike,
        expression: Optional[str] = ...,
        locale: Locale = ...,
    ) -> None: ...
    def resolve_at(self, anchor_rfc3339: Optional[DateLike] = ...) -> TimestampRange: ...
    def to_dict(self) -> DurationRangeDict: ...


def between(
    start_rfc3339: DateLike,
    end_rfc3339: DateLike,
    locale: Locale = ...,
    use_native_ko_number: bool = ...,
) -> str: ...


def from_duration(
    seconds: DurationLike,
    is_future: bool = ...,
    locale: Locale = ...,
    use_native_ko_number: bool = ...,
) -> str: ...


def range_of(
    expression: str,
    locale: Locale = ...,
) -> DurationRange: ...

def extract_expressions(
    input: str,
    locale: Locale = ...,
) -> List[ExpressionCandidateDict]: ...
