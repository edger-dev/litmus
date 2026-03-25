---
# litmus-x2vo
title: Screenshot in side-by-side view
status: completed
type: task
priority: normal
created_at: 2026-03-23T15:17:33Z
updated_at: 2026-03-25T00:41:44Z
blocked_by:
    - litmus-y6dc
---

## Goal

Add a global toggle on the compare page to switch all columns between simulated scene rendering and real provider screenshots.

## Current State

The compare page (`pages/compare.rs`) shows 2-4 themes side by side. Each column renders simulated `SceneView` for every scene. There are no screenshots on this page — those only appear on the detail page.

## Changes

- Add a toolbar at the top of the compare page with:
  - **Rendering toggle**: Simulated / Screenshot (default: Simulated)
  - **Provider selector**: dropdown (kitty, wezterm, etc.) — only visible when Screenshot mode is active
- When in Screenshot mode, replace `SceneView` with `ScreenshotImage` for each (theme, scene) cell
- Use `scene_id_to_fixture_id()` mapping (same as detail page)
- If a screenshot is missing for a given theme+fixture+provider, show the existing placeholder
- Responsive: on narrow screens (<900px), columns stack vertically regardless of mode

## Dependencies

- Blocked by `litmus-y6dc` (global provider selector) — provider is app-level state, no need for a per-page provider dropdown
- Uses existing `ScreenshotImage` component and manifest infrastructure
- Toggle is just Simulated/Screenshot; provider comes from global state

## Plan

1. Add a `use_signal(|| false)` for screenshot mode in CompareThemes
2. Add a toolbar above the grid with Simulated/Screenshot toggle buttons (reuse provider-btn styling pattern)
3. When screenshot mode is active, render ScreenshotImage (using ActiveProvider) instead of TermOutputView
4. Show placeholder when screenshot not available
5. Add CSS for the toggle toolbar
6. Add todo items:
- [ ] Add screenshot mode toggle signal
- [ ] Add toolbar with toggle buttons
- [ ] Conditionally render ScreenshotImage vs TermOutputView
- [ ] Add CSS styling for toolbar
- [ ] Verify compilation, zero warnings

## Summary of Changes

Added Simulated/Screenshot toggle to the compare page:

- Toggle buttons in toolbar above column headers (reuses provider-btn styling pattern)
- Screenshot mode renders ScreenshotImage using the global ActiveProvider
- Shows "No screenshot" placeholder when screenshot unavailable for a theme+fixture+provider
- Default is Simulated mode (TermOutputView rendering)
- CSS: .compare-toolbar, .compare-view-toggle, .compare-toggle-btn, .compare-screenshot-placeholder
