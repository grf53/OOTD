#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HEADER_PATH="$ROOT_DIR/include/ootd.h"

cbindgen --config "$ROOT_DIR/cbindgen.toml" --crate ootd-ffi-c --output "$HEADER_PATH"
echo "Generated: $HEADER_PATH"
