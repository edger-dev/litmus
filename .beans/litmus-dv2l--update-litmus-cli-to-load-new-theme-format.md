---
# litmus-dv2l
title: Update litmus-cli to load new theme format
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:23:09Z
updated_at: 2026-03-24T13:23:18Z
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
