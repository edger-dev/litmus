---
# litmus-k2id
title: Image-Backed Screenshot System
status: completed
type: milestone
priority: normal
created_at: 2026-03-23T09:45:58Z
updated_at: 2026-03-23T10:26:52Z
order: zzzV
---

Replace simulated scene rendering with real terminal screenshots for 100% accurate theme previews. Covers data model, fixture system, capture tool, R2 storage, web integration, and CI automation.

## Summary of Changes

All 6 milestones implemented:

**M1** — Screenshot data model in litmus-model: Provider, Fixture, ScreenshotKey, ScreenshotMeta, ScreenshotManifest with full query API and 17 tests.

**M2** — Fixture system at fixtures/: 7 fixtures (git-diff, ls-color, cargo-build, shell-prompt, git-log, python-repl, htop) with setup.sh/command.sh scripts. Tested locally.

**M3** — litmus-capture crate: ProviderCapture trait, KittyProvider, cage-based headless capture with sentinel file completion detection, PNG→WebP conversion, manifest build/check CLI. 16 tests.

**M4** — R2 storage documented: bucket setup, CORS config, URL scheme, GitHub Actions secrets list.

**M5** — Web app integration: ManifestState + ActiveProvider global state, manifest fetch on load, ScreenshotSceneView component with fallback, ProviderSelector pill bar on ThemeDetail, CSS for all new UI elements.

**M6** — CI workflow screenshots.yml: headless Wayland via WLR_BACKENDS=headless + WLR_RENDERER=pixman, capture-all, R2 upload via awscli.
