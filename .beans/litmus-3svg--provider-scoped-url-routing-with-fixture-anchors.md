---
# litmus-3svg
title: Provider-scoped URL routing with fixture anchors
status: completed
type: feature
priority: normal
created_at: 2026-03-26T14:51:18Z
updated_at: 2026-03-26T16:26:46Z
---

Restructure URL routing to include the provider and support fixture-level deep links.

Current: /theme/ayu-light
Target:  /kitty/theme/ayu-light#python-repl

## Requirements

- [x] Change route from /theme/:slug to /:provider/theme/:slug
- [x] Derive ActiveProvider from URL parameter instead of global signal
- [x] Apply same pattern to other routes: /:provider/compare/:slugs, /:provider/scene/:scene_id
- [x] Browse page: /:provider/ (or keep / and redirect based on provider)
- [x] Add anchor IDs to fixture sections in detail page (scene-{fixture.id} may already exist)
- [x] Update URL hash on scroll using existing IntersectionObserver logic
- [x] Navigating to a URL with #fixture-id scrolls to that fixture
- [x] Update all internal links (browse page cards, sidebar, minimap) to include provider prefix

## Notes

Provider switching in sidebar should navigate to the new provider-scoped URL rather than toggling a global signal.


## Summary of Changes

Implemented provider-scoped URL routing:
- Route enum uses `#[nest("/:provider")]` to prefix all routes with the provider
- Root `/` redirects to `/{default_provider}/`
- Shell syncs URL provider → ActiveProvider signal
- Provider selector navigates via Link instead of toggling state
- All internal links include provider parameter
- Fixture deep-links: `#fixture-id` scrolls on load, URL hash updates on scroll via IntersectionObserver

Key files: `main.rs` (Route enum + helpers), `shell.rs` (sync), `sidebar.rs` (navigation), all page components
