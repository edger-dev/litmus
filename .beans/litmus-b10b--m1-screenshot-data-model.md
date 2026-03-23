---
# litmus-b10b
title: 'M1: Screenshot Data Model'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:05Z
updated_at: 2026-03-23T09:48:02Z
parent: litmus-k2id
---

Add screenshot.rs to litmus-model with Provider, Fixture, ScreenshotKey, ScreenshotMeta, ScreenshotManifest types. Wire into lib.rs. Add serde round-trip tests.

## Summary of Changes

Added `crates/litmus-model/src/screenshot.rs` with:
- `Provider` — terminal emulator metadata (slug, name, optional version)
- `Fixture` — reproducible terminal scenario (id, name, description)
- `ScreenshotKey` — composite key (provider, theme, fixture slugs), flattened in JSON
- `ImageFormat` — Png | Webp with extension/mime helpers
- `ScreenshotMeta` — full screenshot record with URL, dimensions, format, timestamp, SHA-256 checksum
- `ScreenshotManifest` — top-level index with find/for_provider/for_theme/build_index query methods

17 tests covering: serde round-trip, find, URL construction, cache busting, index building, JSON shape verification.

Wired into `lib.rs` as `pub mod screenshot;`.
