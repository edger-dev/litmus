---
# litmus-mm3f
title: Interactive contrast issue navigation with footnotes
status: completed
type: feature
priority: normal
created_at: 2026-03-26T15:49:28Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzk
---

Enhance contrast issue display with navigable issue IDs, merged footnotes, and click-to-cycle behavior.

## Issue IDs

- Assign short IDs (C1, C2, ...) to each unique contrast *rule* violation
- Key by (fg_term, bg_term) — the TermColor variants, not resolved hex values
- Same ANSI color role = same ID across fixtures and themes
- Chip legend shows: "C1: bright black on bg — 2.3:1"

## Merged Footnotes

- Display small superscript footnote tags (C1, C2) on affected spans in the rendered terminal output
- Merge footnotes for contiguous rectangular regions:
  - If consecutive lines have the same issue at the same column range, show one footnote for the block
  - e.g. editor-ui line numbers (C1 on lines 1-20, col 0-3) → single C1 tag at edge of block
- Footnotes are for visual connection to the header chips, not interactive themselves

## Click-to-Cycle

- Clicking a chip (C1) in the header scrolls to the first fixture containing that issue
- Subsequent clicks cycle to the next fixture with the same issue
- When all occurrences have been visited, cycling wraps around
- Clicking an already-selected chip deselects it

## Visual Feedback

- Selected chip: filled/highlighted style (vs default outline)
- Markers matching the focused issue: enhanced border (brighter/thicker) or subtle pulse
- Non-focused issue markers: dim slightly when another issue is focused

## State

- `active_issue: Option<(String, usize)>` — selected issue ID + current fixture index in cycle
- Derived: list of fixture IDs containing each issue for cycling

## Requirements

- [x] Change dedup key from (fg_hex, bg_hex) to (fg_term, bg_term) for issue identity
- [x] Assign stable short IDs (C1, C2, ...) to unique rule violations
- [x] Update header chips to show ID + color role name + ratio
- [x] Implement contiguous region merging for footnote placement
- [x] Render merged footnotes as superscript tags on affected spans/blocks
- [x] Add active_issue state and click-to-cycle on header chips
- [x] Scroll to fixture on chip click, cycle on repeat click
- [x] Visual feedback: selected chip style, enhanced focused markers, dimmed others
- [ ] CSS for footnote tags, selected chip, focused/dimmed marker states

## Notes

- Mobile: not a focus, just ensure it's not broken. Footnotes are visual-only, interaction is via header chips.
- If footnotes prove too noisy in practice, fall back to color-coded borders per issue ID (option C from brainstorm).


## Summary of Changes

Implemented interactive contrast issue navigation with:
- Stable rule IDs (C1, C2, ...) keyed by (fg_term, bg_term) via `build_issue_registry`
- Merged footnotes at end of contiguous issue regions via `compute_footnotes`
- Clickable issue chips that cycle through affected fixtures with smooth scrolling
- Focused/dimmed visual states for markers and footnotes
- Escape key to clear focus
- 7 unit tests for the pure-logic functions

Key files: `term_renderer.rs` (registry, footnotes, rendering), `theme_detail.rs` (state, chips, cycling), `style.css` (visual states)
