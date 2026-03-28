---
# litmus-vkne
title: Vendor provider theme data (kitty-themes, wezterm color schemes)
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:22:50Z
updated_at: 2026-03-26T14:16:47Z
order: zzzzs
parent: litmus-knrz
---

Add vendored provider theme registries under vendor/:

- vendor/kitty-themes/ — from kovidgoyal/kitty-themes repo (the .conf theme files)
- vendor/wezterm-colorschemes/ — from wez/wezterm color scheme data

Use git subtree (preferred over submodule for simplicity). Document the update procedure in a vendor/README.md.

These are the source of truth for the extract-colors step.

## Summary of Changes

Vendored provider theme data from two sources:

- **kitty-themes** (385 .conf files): Added via `git subtree` from kovidgoyal/kitty-themes
- **wezterm-colorschemes** (996 .toml files): Extracted from wez/wezterm's embedded scheme_data.rs (1001 schemes, 5 lost to filename collisions from aliases)

Also added:
- `vendor/README.md` with update procedures for both sources
- `scripts/extract-wezterm-schemes.py` for reproducible wezterm extraction
