---
# litmus-lizo
title: 'M3: Capture Tool (litmus-capture)'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:13Z
updated_at: 2026-03-23T10:01:23Z
parent: litmus-k2id
blocked_by:
    - litmus-b10b
    - litmus-rz7q
---

New workspace crate crates/litmus-capture/. CLI binary with: capture, capture-all, upload, manifest commands. Kitty provider module. Uses cage for headless Wayland, grim for screenshots, FiraCode font, 80x24 geometry. WebP output to staging dir.

## Summary of Changes

Created `crates/litmus-capture/` — a new workspace crate (native binary, excluded from wasm build).

**Modules:**
- `error.rs` — CaptureError enum for typed errors
- `providers/mod.rs` — ProviderCapture trait + TermGeometry struct + provider registry
- `providers/kitty.rs` — KittyProvider: generates kitty.conf with theme colors + FiraCode/80×24/no-decorations settings; builds launch args
- `capture.rs` — core capture logic: fixture setup, cage+kitty orchestration via wrapper script, sentinel-file completion detection, grim screenshot, PNG→WebP conversion, sha256 checksum
- `manifest.rs` — builds ScreenshotManifest by scanning staging/{provider}/{theme}/{fixture}.webp; CoverageReport for CI validation
- `main.rs` — CLI with clap: `capture`, `capture-all`, `manifest build`, `manifest check` subcommands

**Key design:**
- Fixture lifecycle: setup.sh → kitty+command.sh in cage → sentinel file detection → grim screenshot → PNG→WebP
- wrapper.sh runs inside cage's Wayland session; both kitty and grim connect to cage's compositor
- Fixed geometry: 80×24 cols/rows, FiraCode 12pt
- 16 unit tests pass; 83 total workspace tests pass
