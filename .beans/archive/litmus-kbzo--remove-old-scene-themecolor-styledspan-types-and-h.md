---
# litmus-kbzo
title: Remove old Scene, ThemeColor, StyledSpan types and hand-written scenes
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:38Z
updated_at: 2026-03-26T14:16:47Z
order: zzy
parent: litmus-coma
blocked_by:
    - litmus-lm76
    - litmus-0uoe
    - litmus-bcel
---

Final cleanup once all consumers migrated to TermOutput:

- Delete scene.rs (ThemeColor, StyledSpan, SceneLine, Scene)
- Delete scenes.rs (hand-written scene definitions)
- Remove scene_id_to_fixture_id() mapping (no longer needed)
- Clean up any remaining references to old types
- Verify all tests pass

Depends on: web and CLI migrations, contrast validation update


## Plan

1. Add `term_readability_score(theme, fixtures)` to contrast.rs
2. Make `fixtures::all_fixtures()` pub in litmus-web
3. Update SceneMinimap to accept fixture metadata instead of Scene
4. Update sidebar to pass fixture data to minimap
5. Update all pages (theme_detail, scene_across, compare, theme_list) to iterate fixtures instead of scenes
6. Remove SceneView/ScenePreview fallback calls
7. Remove scene_renderer.rs module
8. Remove scene-based contrast functions and tests
9. Delete scene.rs and scenes.rs from litmus-model
10. Clean up all remaining references

## Summary of Changes

Removed all Scene/ThemeColor/StyledSpan types and hand-written scene definitions:

- Deleted `scene.rs` and `scenes.rs` from litmus-model
- Deleted `scene_renderer.rs` from litmus-web  
- Removed `scene_id_to_fixture_id()` mapping from screenshot_view.rs
- Removed `ContrastIssue`, `validate_scene_contrast()`, `readability_score()`, `validate_theme_readability()` and their tests from contrast.rs
- Updated `scene_across.rs` and `compare.rs` to iterate fixtures directly
- Updated `theme_list.rs` to use `term_readability_score()` and removed scene_renderer fallback
- Updated `state.rs` filter to use `term_readability_score()`
- Removed unused `fixture_by_id()` and its test
- All 177 tests pass, zero compiler warnings
