---
# litmus-i4kf
title: Remove old Theme struct and hand-curated color sections
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:23:12Z
updated_at: 2026-03-24T23:58:28Z
parent: litmus-knrz
blocked_by:
    - litmus-y6dc
    - litmus-dv2l
---

Final cleanup once all consumers have migrated:

- Remove the old Theme struct from litmus-model (or rename ProviderColors to Theme if that's cleaner)
- Remove old toml_format.rs parser (or repurpose for ThemeDefinition parsing)
- Delete hand-curated [colors] and [colors.ansi] sections from authored theme TOMLs
- Verify all tests pass, no references to old types remain

Depends on: web and CLI migrations complete

## Plan

1. Update litmus-capture load_all_themes() to use load_themes_dir() instead of parse_toml_theme()
2. Remove unused parse_toml_theme import from capture crate
3. Verify no old-format theme TOMLs remain
4. Keep parse_toml_theme in litmus-model for CLI user-provided file support
5. Keep Theme struct as the runtime display model

### Todo
- [x] Update capture load_all_themes to use new format
- [x] Remove unused imports
- [x] Verify all theme files are in new format
- [x] Zero warnings across all crates
- [x] All tests pass
