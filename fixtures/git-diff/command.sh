#!/usr/bin/env bash
# Show git diff of working tree changes against the last commit.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
git diff --color=always HEAD
