---
# litmus-yx5o
title: Create terminal scenes
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:14Z
updated_at: 2026-03-20T17:52:23Z
parent: litmus-2wte
---

Create realistic terminal scenes: shell prompt, ls --color, git diff (with context/additions/deletions/merge conflicts), delta output, tig log view.

## Summary of Changes

Added `scenes` module with 5 built-in terminal scenes:
- Shell prompt: prompts with grep output, error messages, and dirty branch indicator
- Git diff: multi-hunk diff with context/additions/deletions
- Directory listing: colorized ls -la with dirs, executables, symlinks, hidden files
- Cargo build: compiler warnings, errors with code spans, notes, and help suggestions
- Git log: branch graph with commit hashes, HEAD/remote decorations, branch names
