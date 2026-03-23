---
# litmus-rz7q
title: 'M2: Fixture System'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:05Z
updated_at: 2026-03-23T10:26:52Z
order: zzzy
parent: litmus-k2id
---

Create fixtures/ directory at workspace root. Each fixture has setup.sh, command.sh, optional teardown.sh, README.md. Initial fixtures: git-diff, ls-color, cargo-build, shell-prompt, git-log, python-repl, htop. Add fixtures/README.md with authoring guide.

## Summary of Changes

Created `fixtures/` directory at workspace root with 7 fixture subdirectories:
- `git-diff/` — real git diff with additions, deletions, context (setup: git repo with committed files + unstaged changes)
- `ls-color/` — real ls -la with dirs, symlinks, executables, hidden files
- `git-log/` — real git log --graph with branches, merges, tags (pinned timestamps for reproducibility)
- `cargo-build/` — real cargo build with intentional warnings + type error
- `shell-prompt/` — scripted bash session output (interactive shell not scriptable reliably)
- `python-repl/` — scripted Python REPL session output
- `htop/` — top -b batch mode with scripted fallback

Each fixture has setup.sh + command.sh (all tested working). fixtures/README.md with authoring guide.
Interface: FIXTURE_WORK_DIR env var, CWD set to work dir when command.sh runs.
