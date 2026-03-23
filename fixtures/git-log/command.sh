#!/usr/bin/env bash
# Show colorized git log graph with branch decorations.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"
git log --graph --color=always --format="%C(yellow)%h%C(reset) %C(green)(%ar)%C(reset) %s %C(blue)%an%C(reset)%C(red)%d%C(reset)" --all
