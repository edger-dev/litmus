---
# litmus-x2wd
title: Unified scene navigation & sticky filters
status: completed
type: feature
priority: normal
created_at: 2026-03-21T10:18:46Z
updated_at: 2026-03-21T13:10:26Z
order: zzV
---

Detail page: show all scenes vertically with minimap. Compare page: add minimap. Sticky filter bar on theme list.

## Summary of Changes

### 1. Detail page: all scenes vertically (`theme_detail.rs`)
- Removed tab bar, ActiveScene signal usage, and left/right arrow key scene navigation
- All scenes now render vertically with `id="scene-{id}"" for scroll targeting
- Each scene section has a heading with name and issue count badge
- Contrast issue scene buttons now scroll to the scene instead of switching tabs

### 2. Scene minimap component (`components.rs`)
- Added `SceneMinimap` component: fixed vertical strip on the right edge (120px wide)
- Uses IntersectionObserver to track which scenes are in the viewport
- Polls visible scene state every 200ms via eval-based async loop
- Click scrolls smoothly to the target scene
- Active scenes highlighted with accent border-left + accent text color
- Hidden on mobile (≤768px)

### 3. Compare page (`compare.rs`)
- Added `id="scene-{scene.id}"` to each `compare-scene-group` div
- Added `SceneMinimap` component

### 4. Sticky filter bar (`style.css`)
- Filter bar is now position: sticky with top: 0, z-index: 30
- Has background and border-bottom for visual separation
- Mobile: top: 45px to sit below mobile header

### 5. State (`state.rs` + `main.rs`)
- Added `VisibleScenes(HashSet<String>)` signal
- Provided as context in App component
