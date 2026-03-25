---
# litmus-0uoe
title: Update litmus-cli to render TermOutput
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:32Z
updated_at: 2026-03-25T00:14:03Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-9eg8
---

Migrate litmus-cli mockup views from Scene to TermOutput:

- Load TermOutput data (bundled or from fixtures directory)
- Render TermSpan using crossterm colors:
  - TermColor::Default → crossterm reset
  - TermColor::Ansi(n) → crossterm AnsiValue
  - TermColor::Indexed(n) → crossterm AnsiValue
  - TermColor::Rgb → crossterm Rgb
- Minimal changes — CLI is simpler than web

Depends on: TermOutput types, fixture pipeline generating output files

## Plan

1. Add `resolve_with_theme()` to `TermColor` in litmus-model (reuses existing `indexed_color()`)
2. Add `serde_json` dep to litmus-cli
3. Write tests: `resolve_with_theme` unit tests, TermOutput-to-ratatui-Lines conversion test
4. Embed 3 fixture output.json files (git-diff, ls-color, shell-prompt) via `include_str!()`
5. Rewrite MockupsWidget to parse embedded fixtures, map TermSpan → ratatui Span using theme colors
6. Add `term_color_to_ratatui()` helper in util.rs

## Summary of Changes

- Added `TermColor::resolve_with_theme()` to litmus-model for resolving terminal colors against a Theme (5 tests)
- Rewrote `MockupsWidget` to render real parsed TermOutput from embedded fixture JSON files
- Embedded 5 fixtures: git-diff, git-log, ls-color, cargo-build, shell-prompt
- Added Up/Down arrow keys to cycle between fixtures in the TUI
- Used `LazyLock` to cache parsed fixture data (avoids re-parsing on every frame)
- 8 new tests for color resolution, line conversion, modifier handling, and fixture loading

Commits: d3a0818, d5b902d, 64d5917, f45b930
