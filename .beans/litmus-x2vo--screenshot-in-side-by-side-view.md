---
# litmus-x2vo
title: Screenshot in side-by-side view
status: todo
type: task
priority: normal
created_at: 2026-03-23T15:17:33Z
updated_at: 2026-03-24T14:05:40Z
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

- Uses existing `ScreenshotImage` component and manifest infrastructure
- No dependency on litmus-knrz or litmus-coma — works with current system
- Will automatically benefit from those epics when they land
