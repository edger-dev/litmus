---
# litmus-ct1q
title: TOML and Nix config export
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:28Z
updated_at: 2026-03-20T18:47:35Z
parent: litmus-lio0
---

Copy TOML button (canonical format) and Nix snippet (programs.kitty.settings attrset). Clipboard API.

## Summary of Changes

Added export::to_toml() and export::to_nix() in litmus-model. TOML exports in litmus canonical format, Nix exports as an attrset suitable for programs.kitty.settings. Both available via ExportButtons UI.
