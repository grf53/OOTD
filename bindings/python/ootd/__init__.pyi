from datetime import datetime, timedelta
from typing import Literal, Union

DateLike = Union[str, datetime]
DurationLike = Union[int, timedelta]
Locale = Literal["en", "ko"]


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
