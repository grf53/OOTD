# Outstandingly Obvious Time Delta (OOTD)

OOTD renders time deltas in human phrases that feel more intuitive than strict numeric time strings.

This repository is implemented as a Rust-first multi-binding project.

## Architecture

- `crates/ootd-core`: pure Rust domain logic (`between`, `from_duration`)
- `crates/ootd-ffi-c`: C ABI layer for low-level interop (`cbindgen`, Java/Swift FFI path)
- `bindings/python/rust`: Python native extension crate (`PyO3`)
- `bindings/node/rust`: Node.js native addon (`napi-rs`)
- `bindings/wasm/rust`: Browser/WebAssembly binding (`wasm-bindgen`)
- `bindings/python`: Python package layout and tests (`maturin` project)
- `bindings/java`: Java FFM wrapper and Gradle project
- `bindings/kotlin`: Kotlin/JVM wrapper over Java FFM layer
- `bindings/node`: Node package scaffolding
- `bindings/wasm`: wasm package scaffolding
- `bindings/swift`: Swift package over C FFI (`dlopen`/`dlsym`)

## Supported Locale (v1)

- `en`
- `ko`

## Time Input Contract

- `between` requires timezone-aware RFC3339 timestamps (`Z` or `+/-hh:mm` offsets)
- Naive datetime values are rejected by design
- Delta magnitude is computed on absolute instants (UTC-equivalent comparison)
- Daypart labels (`dawn/morning/...`, `새벽/아침/...`) are anchored to the `end` timestamp timezone offset
- Mixed offsets are allowed; `start` is converted to the `end` offset before daypart labeling
- `ko` only: optionally enable native Korean numerals for `시간` and `달` units (`1 -> 한`, `2 -> 두`, ...)
- Duration humanization uses policy buckets (not calendar-precise month/year lengths):
  - `month` bucket basis: `30d`
  - `year` bucket basis: `12 * 30d = 360d` (policy consistency with month buckets)
  - first `1 year` label starts at `350d` (`350d 00:00:00` and later)

## Core Rust API

```rust
use ootd_core::{
    between_rfc3339, between_rfc3339_with_options, from_duration, Direction, Locale, RenderOptions,
};

let phrase = between_rfc3339(
    "2023-12-09T18:21:29Z",
    "2024-01-25T13:31:43Z",
    Locale::En,
)?;
assert_eq!(phrase, "a month and a half ago");

let mixed_en = between_rfc3339(
    "2024-01-25T01:30:00+09:00",
    "2024-01-25T13:00:00Z",
    Locale::En,
)?;
assert_eq!(mixed_en, "yesterday afternoon");

let mixed_ko = between_rfc3339(
    "2024-01-25T01:30:00+09:00",
    "2024-01-25T13:00:00Z",
    Locale::Ko,
)?;
assert_eq!(mixed_ko, "어제 낮");

let native_ko = between_rfc3339_with_options(
    "2023-12-09T18:21:29Z",
    "2024-01-25T13:31:43Z",
    Locale::Ko,
    RenderOptions {
        ko_native_numerals: true,
    },
)?;
assert_eq!(native_ko, "한 달 반 전");

let phrase = from_duration(90 * 60, Locale::Ko, Direction::Past)?;
assert_eq!(phrase, "1시간 반 전");

let err = from_duration(-1, Locale::En, Direction::Past);
assert!(err.is_err());
```

## Python (PyO3)

Build/install locally:

```bash
cd bindings/python
maturin develop
```

Usage:

```python
import ootd
from datetime import datetime, timezone, timedelta

start = datetime.now(timezone.utc) - timedelta(days=48)
end = datetime.now(timezone.utc)
print(ootd.between(start, end, "en"))
print(ootd.from_duration(90 * 60, False, "ko"))
print(ootd.from_duration(timedelta(minutes=90), False, "ko"))  # timedelta 입력 허용
print(ootd.from_duration(90 * 60 + 0.9, False, "ko"))  # float은 내부에서 int로 변환
print(ootd.from_duration(90 * 60, False, "ko", True))  # 한 시간 반 전
# raises ValueError: negative duration is not allowed: -1
# ootd.from_duration(-1, False, "en")
```

Notes:

- `bindings/python/rust/build.rs` auto-generates `bindings/python/ootd/__init__.pyi` during build.
- `ootd` is a pure-Python wrapper over `ootd._native`, so monkeypatching is straightforward (`ootd.between`, `ootd._between_impl`, etc.).

## TypeScript Node (napi-rs)

```bash
cd bindings/node
npm install
npm run build
```

```ts
import { between, fromDuration } from '@ootd/node'
// locale type: "en" | "ko"
// between input: RFC3339 string | Date | object with toISOString()
// fromDuration input: number | bigint | duration-like object(total/asSeconds/toMillis)

console.log(between('2023-12-09T18:21:29Z', '2024-01-25T13:31:43Z', 'en'))
console.log(between(new Date('2023-12-09T18:21:29Z'), new Date('2024-01-25T13:31:43Z'), 'en'))
console.log(fromDuration(90 * 60, false, 'ko'))
console.log(fromDuration({ asSeconds: () => 90 * 60 }, false, 'ko'))
console.log(fromDuration(90 * 60, false, 'ko', true)) // 한 시간 반 전
// throws Error: negative duration is not allowed: -1
// fromDuration(-1, false, 'en')
```

Note: `Date` inputs are normalized via `toISOString()` (UTC `Z`). If you need a specific offset anchor for daypart labeling, pass explicit RFC3339 strings with that offset.

## TypeScript Browser (wasm-bindgen)

```bash
cd bindings/wasm
npm install
npm run build
```

```ts
import { between, fromDuration } from '@ootd/wasm/pkg/ootd_wasm'
// locale type: "en" | "ko" (generated d.ts is patched after wasm build)

console.log(between('2023-12-09T18:21:29Z', '2024-01-25T13:31:43Z', 'en'))
console.log(fromDuration(90n * 60n, false, 'ko'))
console.log(fromDuration(90n * 60n, false, 'ko', true)) // 한 시간 반 전
// throws Error: negative duration is not allowed: -1
// fromDuration(-1n, false, 'en')
```

## Java (Project Panama / FFM)

Requires JDK `22+` (FFM/Panama target baseline).

Generate C header and optional Panama bindings:

```bash
./scripts/gen-c-header.sh
./scripts/gen-java-bindings.sh
```

Build Java wrapper:

```bash
cd bindings/java
gradle test --no-daemon
```

Usage:

```java
import java.time.Duration;
import java.time.OffsetDateTime;

String phrase = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", OotdLocale.EN);
String phraseFromDateTime = Ootd.between(
        OffsetDateTime.parse("2023-12-09T18:21:29Z"),
        OffsetDateTime.parse("2024-01-25T13:31:43Z"),
        OotdLocale.EN
);
String nativeKo = Ootd.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", OotdLocale.KO, true);
String fromDurationObject = Ootd.fromDuration(Duration.ofMinutes(90), false, OotdLocale.EN);
// throws IllegalArgumentException for negative duration
// Ootd.fromDuration(-1, false, OotdLocale.EN);
```

## Kotlin (JVM)

Requires JDK `22+` (toolchain/jvmTarget baseline).

Build/test:

```bash
cd bindings/kotlin
gradle test --no-daemon
```

Usage:

```kotlin
import io.ootd.OotdLocale
import io.ootd.kotlin.OotdKotlin
import java.time.Duration

println(OotdKotlin.between("2023-12-09T18:21:29Z", "2024-01-25T13:31:43Z", OotdLocale.EN))
println(OotdKotlin.fromDuration(Duration.ofMinutes(90), false, OotdLocale.KO, true))
```

## Swift

Build native library first:

```bash
cargo build -p ootd-ffi-c
cd bindings/swift
swift run ootd-parity
```

Usage:

```swift
import OOTD

let phrase = try OOTD.between(
    startRFC3339: "2023-12-09T18:21:29Z",
    endRFC3339: "2024-01-25T13:31:43Z",
    locale: .en
)

let ko = try OOTD.fromDuration(
    seconds: 90 * 60,
    isFuture: false,
    locale: .ko,
    useNativeKoNumber: true
)
```

## Tooling

- C header generation: `cbindgen` (`cbindgen.toml`)
- Java binding generation: `jextract` (from generated `include/ootd.h`)
- Shared parity fixtures: `tests/parity_cases.json` (`between_cases`, `duration_cases`)

## CI

GitHub Actions runs Rust checks/tests and validates multi-binding build commands.

## License

`LGPL-3.0` (see `LICENSE.txt`)
