#!/usr/bin/env bash
# Run cargo build, showing warnings and errors with full color.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
# cargo build exits non-zero on error; we want to show the output regardless
cargo build --color=always 2>&1 || true
