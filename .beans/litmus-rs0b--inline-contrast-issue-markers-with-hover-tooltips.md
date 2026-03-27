---
# litmus-rs0b
title: Inline contrast issue markers with hover tooltips
status: completed
type: bug
priority: high
created_at: 2026-03-26T14:51:07Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzw
---

The contrast validation logic exists (validate_fixtures_contrast in theme_detail.rs) and shows count badges, but inline marking on the actual rendered text spans is missing/regressed. Need to highlight the specific TermSpan elements that fail contrast rules.

## Requirements

- [x] Identify which TermSpan elements fail contrast rules and mark them inline in the rendered output
- [x] Visual marker: multi-layer colored border (2-3 colors with increasing sizes) to ensure visibility on any theme background — use app theme colors, not the previewed theme's colors
- [x] Hover popup on marked spans: show rule name (e.g. "WCAG AA normal text"), computed contrast ratio, required threshold, and the fg/bg colors involved
- [x] Preserve existing count badges in header and per-fixture sections
- [x] Verify contrast validation works end-to-end after recent refactors (TermOutput migration etc.)

## Design Notes

The border trick: use a multi-layer border (e.g. 1px inner + 2px outer in contrasting colors from the app chrome theme) so the marker is visible regardless of the previewed theme's background color. The app theme is always accessible via CSS custom properties.


## Summary of Changes

Restored inline contrast issue markers that were lost in the Scene→TermOutput migration. Added `SpanIssueDetail` type and `issue_details` prop through the TermOutputView→TermLineView→TermSpanView pipeline. Marked spans display a multi-layer box-shadow border (app-error + app-bg halo) visible on any theme background, with hover tooltips showing APCA threshold, WCAG ratio, and fg/bg color chips. Wired issue details from theme_detail.rs into the renderer.

Files changed:
- `crates/litmus-web/src/term_renderer.rs` — SpanIssueDetail, issue-aware rendering pipeline
- `crates/litmus-web/src/pages/theme_detail.rs` — build issue detail tuples, pass to renderer
- `crates/litmus-web/assets/style.css` — multi-layer box-shadow border, tooltip max-width
