---
# litmus-bpp9
title: kitty.conf export
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:28Z
updated_at: 2026-03-20T18:47:35Z
parent: litmus-lio0
---

Copy kitty.conf button on detail page. Serialize Theme to kitty config format. Copy to clipboard via Clipboard API.

## Summary of Changes

Added export::to_kitty_conf() in litmus-model that serializes a Theme to kitty.conf format. ExportButtons component on detail page with clipboard copy via document::eval.
