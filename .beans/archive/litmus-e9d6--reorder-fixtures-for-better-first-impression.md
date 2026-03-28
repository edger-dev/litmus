---
# litmus-e9d6
title: Reorder fixtures for better first impression
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:26:46Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzzzw
---

Reorder the fixture list so the most visually informative fixtures appear first. Current order is arbitrary (roughly insertion order). New order prioritizes showing theme character at a glance.

## New Order

**Tier 1 — At a glance (show all colors, reveal theme character):**
1. color-showcase — exercises all 16 ANSI colors
2. editor-ui — rich syntax highlighting, line numbers, status bars

**Tier 2 — Real-world developer workflows:**
3. bat-syntax — syntax highlighting
4. git-diff — add/remove coloring
5. cargo-build — error/warning colors
6. ripgrep-search — match highlighting

**Tier 3 — Shell & TUI:**
7. git-log — graph + decorations
8. shell-prompt — minimal but everyday
9. python-repl — prompt + output
10. ls-color — file type colors
11. htop — TUI with bars and gauges
12. log-viewer — structured log levels

## Requirements

- [x] Reorder FIXTURE_DATA in fixtures.rs
- [x] Verify default_fixture() returns color-showcase (used for browse page preview cards)
- [x] Run tests


## Summary of Changes

Reordered FIXTURE_DATA in fixtures.rs from arbitrary insertion order to a tiered layout. color-showcase is now first (best preview thumbnail for browse page cards).

Files changed:
- `crates/litmus-web/src/fixtures.rs`
