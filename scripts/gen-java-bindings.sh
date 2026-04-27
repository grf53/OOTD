#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HEADER_PATH="$ROOT_DIR/include/ootd.h"
OUT_DIR="$ROOT_DIR/bindings/java/src/main/java"

if [[ ! -f "$HEADER_PATH" ]]; then
  echo "Header not found: $HEADER_PATH"
  echo "Run scripts/gen-c-header.sh first."
  exit 1
fi

jextract \
  --source \
  --output "$OUT_DIR" \
  --target-package io.ootd.ffi \
  "$HEADER_PATH"

echo "Generated Java Panama bindings into: $OUT_DIR"
