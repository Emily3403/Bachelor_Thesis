#!/usr/bin/env bash
set -euo pipefail

cargo build --release --target=aarch64-unknown-linux-gnu \
    --features "$FEATURES" \
    --bin "$BINARY_NAME"