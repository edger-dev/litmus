---
# litmus-u1cy
title: Web app shell
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:15Z
updated_at: 2026-03-20T17:58:22Z
parent: litmus-m8ze
---

Set up the web app shell (likely static site — Rust backend optional, could be pure frontend).

## Summary of Changes

Restructured litmus-web with proper Dioxus routing:
- Shell layout: nav header with app title, shared across all pages
- ThemeList page: responsive grid of clickable theme cards showing name + 16-color swatch
- ThemeDetail page: back link, full color palette (special + ANSI), all scene previews
- Added dioxus/router feature for client-side navigation
