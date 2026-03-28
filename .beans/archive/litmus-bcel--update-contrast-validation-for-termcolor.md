---
# litmus-bcel
title: Update contrast validation for TermColor
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:35Z
updated_at: 2026-03-26T14:16:47Z
order: zzzw
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-lm76
---

Update contrast validation to work with TermOutput instead of Scene:

- TermColor::Default and Ansi(0-15) pairs: validate against theme palette (same as before)
- TermColor::Indexed/Rgb vs Default/Ansi: validate fixed color against theme-dependent counterpart
- TermColor::Indexed/Rgb vs Indexed/Rgb: skip (both fixed, theme-independent)
- New insight: can flag "this fixture uses hardcoded colors that clash with this theme's background"
- Update ReadabilityIssue to reference TermSpan positions

Depends on: TermOutput types, web rendering migration


## Plan

1. Add `TermContrastIssue` struct (references fixture_id and TermColor variants)
2. Add `validate_term_output_contrast(output: &TermOutput, theme: &Theme)` function
3. Skip pairs where both fg and bg are fixed (Indexed/Rgb vs Indexed/Rgb)
4. Validate all other pairs using APCA
5. Add `validate_all_fixtures_contrast(fixtures: &[TermOutput], theme: &Theme)` convenience
6. Tests: low-contrast detection, skip-fixed-pairs, dim exclusion

## Summary of Changes

- Added `TermContrastIssue` struct with fixture_id and TermColor variant fields
- Added `validate_term_output_contrast()` — validates TermOutput spans using APCA
  - Skips fixed-color pairs (Indexed/Rgb vs Indexed/Rgb)
  - Skips Default/Default pairs (theme-controlled)
  - Skips dim and whitespace-only spans
- Added `validate_fixtures_contrast()` — aggregates across multiple fixtures
- 7 tests: ANSI detection, fixed-pair skip, fixed-on-theme, dim skip, default/default skip, Ansi-on-Ansi bg, multi-fixture aggregation

Commits: f868232, 53b2a5e
