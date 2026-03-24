---
# litmus-vkne
title: Vendor provider theme data (kitty-themes, wezterm color schemes)
status: todo
type: task
created_at: 2026-03-24T13:22:50Z
updated_at: 2026-03-24T13:22:50Z
parent: litmus-knrz
---

Add vendored provider theme registries under vendor/:

- vendor/kitty-themes/ — from kovidgoyal/kitty-themes repo (the .conf theme files)
- vendor/wezterm-colorschemes/ — from wez/wezterm color scheme data

Use git subtree (preferred over submodule for simplicity). Document the update procedure in a vendor/README.md.

These are the source of truth for the extract-colors step.
