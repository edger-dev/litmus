---
# litmus-z20l
title: Build litmus extract-colors command
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:22:55Z
updated_at: 2026-03-24T15:20:17Z
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

## Plan

### Step 1: Add wezterm TOML parser to litmus-model
New `wezterm.rs` module with `parse_wezterm_scheme()` → Theme
Format: uses `ansi`/`brights` arrays, cursor_bg/cursor_fg, selection_bg/selection_fg

### Step 2: Add Theme → ProviderColors conversion
Helper in provider module: `ProviderColors::from_theme(theme, provider_slug, version)`

### Step 3: Add ProviderColors serialization to TOML
`ProviderColors::to_toml()` → formatted output matching expected generated file format

### Step 4: Add ExtractColors subcommand to litmus-capture
- Read ThemeDefinition files from themes/
- For each provider mapping, look up vendored theme file
- Parse with appropriate parser → Theme → ProviderColors
- Write {slug}.{provider}.toml
- Flags: --provider, --theme

### Step 5: Add kitty vendor lookup
Look up theme name in vendor/kitty-themes/themes.json index → find .conf file

### Step 6: Add wezterm vendor lookup
Look up theme name in vendored .toml files (scan by metadata.name field)

## Summary of Changes

Implemented the extract-colors pipeline across two crates:

**litmus-model additions:**
- `wezterm.rs`: New parser for wezterm color scheme TOML format (ansi/brights arrays, optional cursor/selection)
- `provider.rs`: Added `ProviderColors::from_theme()` conversion and `to_toml()` serialization

**litmus-capture additions:**
- `extract.rs`: Vendor index builders (kitty themes.json, wezterm metadata.name + aliases), theme definition scanner, provider-specific color extraction
- `main.rs`: New `ExtractColors` subcommand with --provider, --theme, --force flags

End-to-end tested: successfully extracts colors from both kitty and wezterm vendor data, writing correctly formatted ProviderColors TOML files.
