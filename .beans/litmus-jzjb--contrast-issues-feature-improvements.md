---
# litmus-jzjb
title: Contrast issues feature improvements
status: completed
type: feature
priority: normal
created_at: 2026-03-21T06:40:03Z
updated_at: 2026-03-21T06:43:52Z
---

Add stable slugs, scene tab badges, visual-first issue display, and scene span markers

## Summary of Changes

### 1. Model: Stable slugs for issue types
- Added `ThemeColor::slug()` method returning short identifiers ("fg", "bg", "ansi1", etc.)
- Added `slug`, `fg_color`, and `bg_color` fields to `ContrastIssue`
- Slug format: `"scene-id/fg-slug-on-bg-slug"` (e.g. `"cargo-build/ansi1-on-bg"`)
- Populated from span fg/bg in `validate_scene_contrast`

### 2. Scene tab badges
- Built `HashMap<&str, usize>` counting issues per scene
- Scene tab buttons show a pill badge with count when > 0
- CSS: small red pill using `--app-error` background

### 3. Visual-first issue display
- Deduplicated issues by slug within each scene group
- Replaced text-heavy layout with colored sample spans showing actual fg/bg
- Ratio shown prominently, hex codes dimmer as secondary info
- Scene group headers are clickable buttons that navigate to that scene tab

### 4. Scene span markers
- Added `issue_spans: Vec<(usize, usize)>` prop to SceneView
- Threaded through LineView (line_idx) → SpanView (has_issue)
- CSS class `contrast-issue-span` with dashed red outline
- theme_detail.rs filters issues to current scene and builds the (line, span) vec
- ScenePreview and AllScenesView use defaults (no markers)
