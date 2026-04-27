"""Python-friendly OOTD wrapper module.

Public functions are pure Python callables delegating to the native extension,
which allows straightforward monkeypatching in tests.
"""

from datetime import datetime, timedelta
from typing import Callable, Literal, Union

from . import _native

__all__ = ["between", "from_duration"]


DateLike = Union[str, datetime]
DurationLike = Union[int, timedelta]
Locale = Literal["en", "ko"]
BetweenImpl = Callable[[DateLike, DateLike, str, bool], str]
FromDurationImpl = Callable[[DurationLike, bool, str, bool], str]

# Exposed indirection points for monkeypatch/testing.
_between_impl: BetweenImpl = _native.between
_from_duration_impl: FromDurationImpl = _native.from_duration


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
