---
# litmus-z20l
title: Build litmus extract-colors command
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:22:55Z
updated_at: 2026-03-24T13:22:58Z
parent: litmus-knrz
blocked_by:
    - litmus-jmna
    - litmus-vkne
---

Add an `extract-colors` subcommand to litmus-capture (or a new litmus-extract crate):

- Reads ThemeDefinition files from themes/
- For each provider mapping, looks up the theme name in vendored data (vendor/kitty-themes/, vendor/wezterm-colorschemes/)
- Parses the provider's native format into ProviderColors (reuse/extend existing kitty.rs parser, add wezterm TOML parser)
- Writes {theme-slug}.{provider}.toml next to the definition file
- Flags: --provider (filter to one provider), --theme (filter to one theme)
- Skips if generated file already exists and vendored source hasn't changed (optional optimization)

Depends on: new model types, vendored theme data
