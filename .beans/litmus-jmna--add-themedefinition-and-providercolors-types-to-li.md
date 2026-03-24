---
# litmus-jmna
title: Add ThemeDefinition and ProviderColors types to litmus-model
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:22:47Z
updated_at: 2026-03-24T14:57:30Z
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
