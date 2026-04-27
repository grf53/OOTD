use std::fs;
use std::path::PathBuf;

const STUB_CONTENT: &str = r#"from datetime import datetime, timedelta
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
"#;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let workspace_dir = crate_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("workspace root");
    let stub_path = workspace_dir.join("bindings/python/ootd/__init__.pyi");

    if let Some(parent) = stub_path.parent() {
        fs::create_dir_all(parent).expect("create stub dir");
    }

    let existing = fs::read_to_string(&stub_path).ok();
    if existing.as_deref() != Some(STUB_CONTENT) {
        fs::write(&stub_path, STUB_CONTENT).expect("write pyi file");
    }
}
