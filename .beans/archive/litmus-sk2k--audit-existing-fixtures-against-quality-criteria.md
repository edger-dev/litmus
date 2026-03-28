---
# litmus-sk2k
title: Audit existing fixtures against quality criteria
status: completed
type: task
priority: normal
created_at: 2026-03-24T14:01:44Z
updated_at: 2026-03-26T14:16:47Z
order: zs
parent: litmus-49jz
---

Review the 7 existing fixtures against the quality criteria:

1. Color variety (≥4 distinct ANSI colors)
2. Instant recognition
3. Fits 80x24
4. Deterministic
5. Self-contained

Fix any that don't meet criteria. Known issues:
- htop fallback may not be deterministic (real top output varies)
- Some fixtures rely on tool color defaults which may vary by system
- shell-prompt and python-repl are simulated (printf/echo) — check if they still make sense as fixtures or should be replaced with real tool output

This can run in parallel with new fixture work.

## Audit Results

| Fixture | Color | Recognition | 80x24 | Deterministic | Self-Contained | Action |
|---------|:---:|:---:|:---:|:---:|:---:|--------|
| git-diff | PASS | PASS | PASS | PASS | PASS | None |
| git-log | PASS | PASS | PASS | PASS | PASS | None |
| ls-color | PASS | PASS | PASS | FIXED | PASS | Set explicit LS_COLORS |
| cargo-build | PASS | PASS | PASS | PASS | PASS | None |
| shell-prompt | PASS | PASS | PASS | PASS | PASS | Simulated (OK) |
| python-repl | PASS | PASS | PASS | PASS | PASS | Simulated (OK) |
| htop | PASS | PASS | PASS | FIXED | PASS | Always use scripted output |

## Summary of Changes

- **htop**: Removed non-deterministic `top -b -n 1` path that produced varying process data. Now always uses scripted htop-like display with fixed processes and ANSI colors.
- **ls-color**: Added explicit `LS_COLORS` export to ensure consistent file type coloring across systems.
- **shell-prompt, python-repl**: Confirmed simulated approach is appropriate for reproducibility.
- 5 fixtures (git-diff, git-log, cargo-build, shell-prompt, python-repl) passed all criteria with no changes needed.
