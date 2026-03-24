---
# litmus-y6dc
title: Update litmus-web to provider-scoped theme rendering
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:23:07Z
updated_at: 2026-03-24T13:23:18Z
parent: litmus-knrz
blocked_by:
    - litmus-jmna
    - litmus-o4w9
---

Migrate litmus-web from the old Theme struct to ThemeDefinition + ProviderColors:

- Update load_embedded_themes() to return Vec<ThemeDefinition> + per-provider color map
- Theme only listed if it has ≥1 ProviderColors file
- Provider selector on detail page switches both screenshots AND simulated scenes
- Simulated scenes render using ProviderColors for the selected provider
- Contrast validation scoped to selected provider's colors
- Theme list cards show available provider badges/icons
- Update state management (AppThemeSlug, etc.) to track selected provider

Depends on: new model types, converted themes
