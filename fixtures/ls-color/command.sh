#!/usr/bin/env bash
# Show colorized ls output of the prepared directory.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
ls -la --color=always
