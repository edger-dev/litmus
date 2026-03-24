---
# litmus-i4kf
title: Remove old Theme struct and hand-curated color sections
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:23:12Z
updated_at: 2026-03-24T13:23:18Z
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
