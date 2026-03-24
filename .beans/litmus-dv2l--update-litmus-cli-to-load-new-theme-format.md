---
# litmus-dv2l
title: Update litmus-cli to load new theme format
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:23:09Z
updated_at: 2026-03-24T15:40:57Z
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
- [x] Add ProviderColors → Theme conversion
- [x] Update load_bundled_themes to use load_themes_dir
- [x] Add --provider CLI flag
- [x] Tests pass, zero warnings
- [x] Review

## Summary of Changes

Migrated litmus-cli from directly loading Theme objects to using the new provider-based system:

- **ProviderColors::to_theme()**: New method converts provider colors + theme name into a renderable Theme
- **load_bundled_themes(provider)**: Rewrote to use load_themes_dir(), picking one ProviderColors per ThemeDefinition (first available alphabetically, or filtered by --provider)
- **--provider CLI flag**: Simple arg parser for --provider <slug> filtering
- All widget rendering unchanged — still receives &Theme
- Hardcoded fallback themes preserved for environments without themes/ directory
- 160 tests pass, zero warnings
