#!/usr/bin/env bash
# Show ripgrep search results with filename, line numbers, and match highlighting.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
rg --color=always --heading --line-number "Config" src/
