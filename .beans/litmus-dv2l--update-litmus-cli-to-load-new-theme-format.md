---
# litmus-dv2l
title: Update litmus-cli to load new theme format
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:23:09Z
updated_at: 2026-03-24T15:38:06Z
parent: litmus-knrz
blocked_by:
    - litmus-jmna
    - litmus-o4w9
---

Migrate litmus-cli from old Theme struct to ThemeDefinition + ProviderColors:

- Load ThemeDefinition + pick one ProviderColors (first available, or --provider flag)
- Thread ProviderColors through to rendering (swatches, mockups, live views)
- Minimal changes — CLI is simpler than web

Depends on: new model types, converted themes

## Plan

1. Update `theme_data.rs`:
   - Add `load_bundled_provider_themes(provider: Option<&str>) -> Vec<Theme>`
   - Uses `load_themes_dir()` to load ThemeDefinitions + ProviderColors
   - For each ThemeDefinition, picks first available ProviderColors (or filtered by provider)
   - Converts (ThemeDefinition, ProviderColors) → Theme via `to_theme()` helper
   - Falls back to hardcoded themes if nothing found

2. Update `main.rs`:
   - Add `--provider` CLI flag
   - Pass to `load_bundled_provider_themes()`

3. Keep all widget code unchanged — still render with `&Theme`

### Todo
- [ ] Add ProviderColors → Theme conversion
- [ ] Update load_bundled_themes to use load_themes_dir
- [ ] Add --provider CLI flag
- [ ] Tests pass, zero warnings
- [ ] Review
