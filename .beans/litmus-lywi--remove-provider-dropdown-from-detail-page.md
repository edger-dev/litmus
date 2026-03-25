---
# litmus-lywi
title: Remove provider dropdown from detail page
status: completed
type: bug
priority: normal
created_at: 2026-03-25T15:05:20Z
updated_at: 2026-03-25T15:46:48Z
---

The detail page has its own local selected_provider signal and dropdown, leftover from before the UI rework. Remove it and read from global ActiveProvider instead. ## Design

Remove the local signal and dropdown from theme_detail.rs. Read the provider from the global ActiveProvider context signal instead. Single source of truth.

**Files:** theme_detail.rs, style.css (remove .detail-provider-select / .detail-provider-label)

## Tasks
- [x] Remove local selected_provider signal from theme_detail.rs
- [x] Remove provider dropdown UI from theme_detail.rs
- [x] Use global ActiveProvider context signal for screenshot lookups
- [x] Remove .detail-provider-select / .detail-provider-label CSS
- [x] Verify detail page works with sidebar provider switching

## Summary of Changes

Removed the local `selected_provider` signal and provider dropdown from the detail page. Screenshot lookups now use the global `ActiveProvider` context signal (single source of truth). Hoisted loop-invariant `cur_provider` and reused existing `this_slug` instead of recomputing per iteration. Removed the now-unused `manifest_provider_slugs` function and associated CSS.
