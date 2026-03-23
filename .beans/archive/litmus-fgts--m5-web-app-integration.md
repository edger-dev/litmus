---
# litmus-fgts
title: 'M5: Web App Integration'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:24Z
updated_at: 2026-03-23T10:26:52Z
order: zzzzk
parent: litmus-k2id
blocked_by:
    - litmus-b10b
    - litmus-j0d6
---

Add ManifestState + ActiveProvider to state.rs. Fetch manifest.json on app mount. Add ScreenshotView component with lazy loading and simulated-scene fallback. Add provider selector pill bar to ThemeDetail. Integrate into theme_detail, scene_across, compare pages.

## Summary of Changes

**state.rs**: Added `ActiveProvider` (slug string, default 'simulated') and `ManifestState` (Option<ScreenshotManifest>) global state types.

**main.rs**: Added context providers for ActiveProvider + ManifestState. Added manifest fetch on app mount using eval JS + spawn; populates ManifestState when manifest.json is available at MANIFEST_URL.

**screenshot_view.rs** (new): 
- `ScreenshotSceneView` component: renders real screenshot `<img>` from CDN URL when available, with simulated SceneView as fallback on load error or when no screenshot exists
- `ProviderSelector` component: pill buttons showing 'Simulated' + any providers with screenshots for the current theme (hidden when only Simulated is available)
- `scene_id_to_fixture_id()`: maps simulated scene IDs to fixture IDs (neovim → None, deferred)

**pages/theme_detail.rs**: Added ProviderSelector bar above scenes; conditionally uses ScreenshotSceneView when provider != 'simulated' and a fixture mapping exists.

**assets/style.css**: Added CSS for .screenshot-block, .screenshot-img, .screenshot-fallback, .provider-selector, .provider-pills, .provider-pill, .provider-pill-active.

**Cargo.toml (litmus-web)**: Added serde_json dependency for manifest deserialization.
