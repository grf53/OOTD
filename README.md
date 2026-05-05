# Outstandingly Obvious Time Delta (OOTD)

[![Rust](https://img.shields.io/badge/Rust-stable-f46623?logo=rust&logoColor=white)](#rust) [![Python](https://img.shields.io/badge/Python-%3E%3D3.8-3776AB?logo=python&logoColor=white)](#python) [![TypeScript](https://img.shields.io/badge/TypeScript-Node-3178C6?logo=typescript&logoColor=white)](#typescript-node) [![WebAssembly](https://img.shields.io/badge/WebAssembly-browser-654FF0?logo=webassembly&logoColor=white)](#typescript-browser-webassembly)  
[![Java](https://img.shields.io/badge/Java-JDK_%3E%3D22-ED8B00?logo=java&logoColor=white)](#java) [![Kotlin](https://img.shields.io/badge/Kotlin-JVM_%3E%3D22-7F52FF?logo=kotlin&logoColor=white)](#kotlin) [![Swift](https://img.shields.io/badge/Swift-%3E%3D5.9-F05138?logo=swift&logoColor=white)](#swift)

OOTD supports bidirectional conversion between relative-time expressions and time ranges.
- OOTD renders time deltas as glanceable, localized phrases for feeds, notifications, timelines, and logs.
- OOTD parses natural phrases back into query-ready time ranges.


## Quick start

```python
import ootd

# duration/timestamps -> expression
print(ootd.between("2026-03-09T18:21:29Z", "2026-05-03T19:31:43Z"))
# 2 months ago

# expression -> range -> concrete timestamps (query-ready)
r = ootd.range_of("2 months ago")
ts = r.resolve_at("2026-04-29T12:00:00Z")
print(f"Between {ts.start} and {ts.end}.")
# Between 2026-02-18 12:00:01Z and 2026-03-05 12:00:00Z.

# locale support
print(ootd.between("2026-03-09T18:21:29Z", "2026-05-03T19:31:43Z",
                   locale="ko", use_native_ko_number=True))
# 두 달 전
```

Same interval, different rendering:
| Case | Expression |
| --- | --- |
| Actual `datetime.timedelta` | `55 days, 1:10:14` |
| Common site display | `1 month ago` |
| **OOTD** | **`2 months ago`** |

You know a 55-day gap is actually two months.  
But no site says that until the calendar-month delta rolls over to `2`.

OOTD also works in the reverse direction: parse phrases like `두 달 전`,
`yesterday afternoon`, then resolve to absolute timestamp ranges for DB queries.

See the same idea in whatever stack you ship:

- [![Rust](https://img.shields.io/badge/Rust-stable-f46623?logo=rust&logoColor=white)](#rust)
- [![Python](https://img.shields.io/badge/Python-%3E%3D3.8-3776AB?logo=python&logoColor=white)](#python)
- [![TypeScript](https://img.shields.io/badge/TypeScript-Node-3178C6?logo=typescript&logoColor=white)](#typescript-node)
- [![WebAssembly](https://img.shields.io/badge/WebAssembly-browser-654FF0?logo=webassembly&logoColor=white)](#typescript-browser-webassembly)
- [![Java](https://img.shields.io/badge/Java-JDK_%3E%3D22-ED8B00?logo=java&logoColor=white)](#java)
- [![Kotlin](https://img.shields.io/badge/Kotlin-JVM_%3E%3D22-7F52FF?logo=kotlin&logoColor=white)](#kotlin)
- [![Swift](https://img.shields.io/badge/Swift-%3E%3D5.9-F05138?logo=swift&logoColor=white)](#swift)

## Behavior By Example

OOTD gives people the phrase they understand at a glance. (`ootd.between(Start, End)`):

| Start | End | English | Korean(`locale="ko"`) |
| --- | --- | --- | --- |
| `2023-11-03` | `2026-05-03` | `2 years and a half ago` | `2년 반 전` |
| `03-09` | `05-03` | `2 months ago` | `두 달 전` |
| `03-24` | `05-03` | `a month and a half ago` | `한 달 반 전` |
| `06-12` | `05-03` | `a month and a half later` | `한 달 반 후` |
| `04-23` | `05-03` | `a week ago` | `1주 전` |
| `05-10` | `05-03` | `a week later` | `1주 후` |
| `05-02 13:30` | `05-03 12:00` | `yesterday afternoon` | `어제 낮` |
| `05-04 13:30` | `05-03 20:30` | `tomorrow afternoon` | `내일 낮` |
| `05-04 08:00` | `05-03 20:00` | `tomorrow morning` | `내일 아침` |
| `20:30` | `23:30` | `earlier tonight` | `오늘 밤` |
| `09:07` | `10:42` | `an hour and a half ago` | `한 시간 반 전` |
| `10:42` | `09:07` | `an hour and a half later` | `한 시간 반 후` |

Range interpretation check (`ootd.range_of(Expression).resolve_at(Anchor)`):

| Target | Anchor | Expression | Resolved range | `Target` in range? |
| --- | --- | --- | --- | --- |
| `2023-11-03` | `2026-05-03` | `2 years and a half ago` | `2023-07-18 ~ 2024-01-14` | Yes |
| `03-09` | `05-03` | `2 months ago` | `02-22 00:00:01 ~ 03-09 00:00:00` | Yes |
| `05-10` | `05-03` | `a week later` | `05-10 ~ 05-13` | Yes |
| `01-24 16:30` | `01-25 13:00` | `yesterday afternoon` | `01-24 11:00:00 ~ 16:59:59` | Yes |

## Languages

### Rust

```rust
use ootd_core::{between_rfc3339, between_rfc3339_with_options, Locale, RenderOptions};

let phrase = between_rfc3339(
    "2026-03-09T18:21:29Z",
    "2026-05-03T19:31:43Z",
    Locale::En,
)?;
assert_eq!(phrase, "2 months ago");

let ko = between_rfc3339_with_options(
    "2026-03-09T18:21:29Z",
    "2026-05-03T19:31:43Z",
    Locale::Ko,
    RenderOptions {
        ko_native_numerals: true,
    },
)?;
assert_eq!(ko, "두 달 전");
```

### Python

The Python API accepts RFC3339 strings or timezone-aware `datetime` objects for
`between`. Naive datetimes are rejected so the output cannot silently depend on
the machine timezone.

```python
import ootd

print(ootd.between(
    "2026-03-09T18:21:29Z",
    "2026-05-03T19:31:43Z",
))
# 2 months ago

print(ootd.between(
    "2026-03-09T18:21:29Z",
    "2026-05-03T19:31:43Z",
    locale="ko",
    use_native_ko_number=True,
))
# 두 달 전

r = ootd.range_of("두 달 전", "ko")
ts = r.resolve_at("2026-04-29T12:00:00+09:00")
# ts.start, ts.end are timezone-aware datetime
```

### TypeScript Node

```ts
import { between, rangeOf } from '@ootd/node'

console.log(between('2026-03-09T18:21:29Z', '2026-05-03T19:31:43Z', 'en'))
// 2 months ago

console.log(between('2026-03-09T18:21:29Z', '2026-05-03T19:31:43Z', 'ko', true))
// 두 달 전

const r = rangeOf('두 달 전', 'ko')
const ts = r.resolveAt('2026-04-29T12:00:00+09:00')
// ts.start, ts.end are Date
```

### TypeScript Browser WebAssembly

```ts
import { between, rangeOf } from '@ootd/wasm'

console.log(between('2026-03-09T18:21:29Z', '2026-05-03T19:31:43Z', 'en'))
// 2 months ago

console.log(between('2026-03-09T18:21:29Z', '2026-05-03T19:31:43Z', 'ko', true))
// 두 달 전

const r = rangeOf('두 달 전', 'ko')
const ts = r.resolveAt('2026-04-29T12:00:00+09:00')
// ts.start, ts.end are Date
```

### Java

```java
import io.ootd.Ootd;
import io.ootd.Locale;

String phrase = Ootd.between(
        "2026-03-09T18:21:29Z",
        "2026-05-03T19:31:43Z",
        Locale.EN
);
// 2 months ago

String ko = Ootd.between(
        "2026-03-09T18:21:29Z",
        "2026-05-03T19:31:43Z",
        Locale.KO,
        true
);
// 두 달 전

var r = Ootd.rangeOf("두 달 전", Locale.KO);
var ts = r.resolveAt("2026-04-29T12:00:00+09:00");
// ts.start(), ts.end() are OffsetDateTime
```

### Kotlin

```kotlin
import io.ootd.Locale
import io.ootd.kotlin.Ootd

println(Ootd.between("2026-03-09T18:21:29Z", "2026-05-03T19:31:43Z", Locale.EN))
// 2 months ago

println(Ootd.between("2026-03-09T18:21:29Z", "2026-05-03T19:31:43Z", Locale.KO, true))
// 두 달 전

val r = Ootd.rangeOf("두 달 전", Locale.KO)
val ts = r.resolveAt("2026-04-29T12:00:00+09:00")
// ts.start(), ts.end()
```

### Swift

```swift
import OOTD

let phrase = try OOTD.between(
    startRFC3339: "2026-03-09T18:21:29Z",
    endRFC3339: "2026-05-03T19:31:43Z",
    locale: .en
)
// 2 months ago

let ko = try OOTD.between(
    startRFC3339: "2026-03-09T18:21:29Z",
    endRFC3339: "2026-05-03T19:31:43Z",
    locale: .ko,
    useNativeKoNumber: true
)
// 두 달 전

let r = try OOTD.rangeOf(expression: "두 달 전", locale: .ko)
let ts = try r.resolveAt("2026-04-29T12:00:00+09:00")
// ts.start, ts.end are Date
```

## API Shape

Core operations are the same across bindings and form a bidirectional flow:

| Operation | Use when | Direction |
| --- | --- | --- |
| `between(start, end, locale, options)` | You have two timestamp instants. | `end - start` decides past/future. |
| `from_duration(seconds, is_future, locale, options)` | You already have an elapsed duration. | `is_future=False` renders past, `True` renders future. |
| `range_of(expression, locale)` | You have a relative phrase like `두 달 전`, `yesterday afternoon`. | Returns a duration range relative to an anchor. |
| `duration_range.resolve_at(anchor)` | You want concrete query timestamps. | Resolves to absolute timestamp range usable directly in queries. |

Supported locales:

- `en`
- `ko`

Korean native counters can be enabled for `시간` and `달` units:

```python
ootd.from_duration(90 * 60, False, "ko", True)
# 한 시간 반 전
```

## Bindings

This is a Rust-first multi-binding repository.

| Binding | Location | Build or test |
| --- | --- | --- |
| Rust core | `crates/ootd-core` | `cargo test -p ootd-core` |
| C FFI | `crates/ootd-ffi-c` | `cargo build -p ootd-ffi-c` |
| Python | `bindings/python` | `maturin develop && pytest tests` |
| Node | `bindings/node` | `npm ci && npm run build && node test/parity.test.mjs` |
| WebAssembly | `bindings/wasm` | `npm ci && npm run build && npm run test:parity` |
| Java | `bindings/java` | `gradle test --no-daemon` |
| Kotlin | `bindings/kotlin` | `gradle test --no-daemon` |
| Swift | `bindings/swift` | `swift run ootd-parity` |

## Input Contract

- `between` requires RFC3339 timestamps or timezone-aware datetime objects.
- Naive datetime values are rejected by design.
- Mixed offsets are allowed.
- Delta magnitude is computed by comparing absolute instants.
- Daypart labels are based on the `start` time converted to the `end` timezone
  offset.
- `from_duration` accepts non-negative durations.

## Tooling

- C header generation: `cbindgen` with `cbindgen.toml`
- Java binding generation: `jextract` from `include/ootd.h`
- Shared parity fixtures: `tests/parity_cases.json`
- CI: GitHub Actions validates Rust and all maintained bindings

## License

`LGPL-3.0` (see `LICENSE.txt`)
