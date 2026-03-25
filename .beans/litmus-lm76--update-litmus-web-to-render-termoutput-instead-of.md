---
# litmus-lm76
title: Update litmus-web to render TermOutput instead of Scene
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:28Z
updated_at: 2026-03-25T00:22:01Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-9eg8
---

Migrate litmus-web from Scene to TermOutput rendering:

- Embed output.{provider}.json files via include_str!() or include_bytes!()
- New TermOutputView component replaces SceneView:
  - Renders TermSpan as <span> with inline CSS color
  - TermColor::Default → theme fg/bg from ProviderColors
  - TermColor::Ansi(n) → ProviderColors ANSI palette
  - TermColor::Indexed(n) → fixed 256-color → rgb CSS
  - TermColor::Rgb → literal rgb CSS
  - Bold/italic/dim/underline → CSS font-weight/style/opacity/decoration
- Provider selector switches which output.{provider}.json is rendered
- Update theme_detail.rs side-by-side to use TermOutputView on the left
- Update ScenePreview (theme list cards) to use TermOutput

Depends on: TermOutput types, fixture pipeline generating output files


## Plan

1. Create `crates/litmus-web/src/fixtures.rs` — embed fixture output.json files, parse with LazyLock
2. Create `crates/litmus-web/src/term_renderer.rs` — TermOutputView and TermOutputPreview components
3. Wire TermOutputView into theme_detail.rs (replace SceneView on left panel)
4. Wire TermOutputPreview into theme_list.rs (replace ScenePreview in cards)
5. Wire TermOutputView into scene_across.rs and compare.rs
6. Tests for color resolution and fixture loading

## Summary of Changes

- Created `fixtures.rs` module: embeds 8 fixture output.json files, OnceLock caching, `fixture_by_id()` and `default_fixture()` API (4 tests)
- Created `term_renderer.rs` module: `TermOutputView` and `TermOutputPreview` Dioxus components that render TermSpan with theme-aware inline CSS colors
- Updated theme_list.rs: TermOutputPreview for card previews (ScenePreview fallback)
- Updated theme_detail.rs: TermOutputView on left panel of side-by-side (SceneView fallback for neovim)
- Updated scene_across.rs: TermOutputView in grid cards
- Updated compare.rs: TermOutputView in comparison grid

Note: Contrast analysis tooltips are not available in TermOutput path (they were Scene-model-specific). Header stats still computed. Can be reimplemented as a follow-up.

Commits: 975c94d, c475a27, bc2393a
