#!/usr/bin/env bash
# Show syntax-highlighted Python source with bat.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
bat --color=always --style=numbers,grid --line-range=1:22 --paging=never server.py
