---
# litmus-jmna
title: Add ThemeDefinition and ProviderColors types to litmus-model
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:22:47Z
updated_at: 2026-03-26T14:16:47Z
order: zk
parent: litmus-knrz
---

Add new types to litmus-model alongside the existing Theme struct (don't remove it yet):

- `ThemeDefinition`: name, variant (dark/light), slug, providers (HashMap<String, String>)
- `ProviderColors`: provider slug, source_version, all 22 color fields (same as current Theme colors)
- TOML deserialization for both: `.toml` for definitions, `.{provider}.toml` for provider colors
- Loader function that scans a themes directory and returns `Vec<ThemeDefinition>` + `HashMap<(slug, provider), ProviderColors>`

Keep the old Theme struct and parsers intact — they'll be removed in a later task after consumers migrate.

## Plan

### New file: `crates/litmus-model/src/provider.rs`

1. `Variant` enum (Dark/Light) with serde lowercase
2. `ThemeDefinition` struct: name, variant, slug (derived from filename), providers HashMap
3. `ProviderColors` struct: provider slug, source_version, same color fields as Theme
4. TOML parsing for both types (reuse existing parse_field pattern)
5. `load_themes_dir()` function: scan directory, return Vec<ThemeDefinition> + HashMap<(String, String), ProviderColors>

### Test-first approach
- Write tests for Variant serde, ThemeDefinition parsing, ProviderColors parsing, and loader function
- Then implement to make them pass

### Commits
1. Tests commit (failing)
2. Implementation commit

## Summary of Changes

Added `provider` module to litmus-model with:

- **`Variant`** enum (Dark/Light) with lowercase serde
- **`ThemeDefinition`** struct: name, variant, slug (from filename), providers HashMap<String, String>
- **`ProviderColors`** struct: provider slug, source_version, all 21 color fields matching Theme
- TOML parsing for both types via `parse_theme_definition()` and `parse_provider_colors()`
- **`load_themes_dir()`** recursive directory loader returning Vec<ThemeDefinition> + HashMap<(slug, provider), ProviderColors>
- Shared `parse_hex_color()` helper extracted to lib.rs, reused across toml_format and provider modules
- Filename/TOML provider validation in the loader
- 20 tests covering parsing, serde, edge cases, directory loading, error propagation

Old Theme struct left intact as specified.
