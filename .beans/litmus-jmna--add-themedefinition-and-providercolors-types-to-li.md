---
# litmus-jmna
title: Add ThemeDefinition and ProviderColors types to litmus-model
status: todo
type: task
created_at: 2026-03-24T13:22:47Z
updated_at: 2026-03-24T13:22:47Z
parent: litmus-knrz
---

Add new types to litmus-model alongside the existing Theme struct (don't remove it yet):

- `ThemeDefinition`: name, variant (dark/light), slug, providers (HashMap<String, String>)
- `ProviderColors`: provider slug, source_version, all 22 color fields (same as current Theme colors)
- TOML deserialization for both: `.toml` for definitions, `.{provider}.toml` for provider colors
- Loader function that scans a themes directory and returns `Vec<ThemeDefinition>` + `HashMap<(slug, provider), ProviderColors>`

Keep the old Theme struct and parsers intact — they'll be removed in a later task after consumers migrate.
