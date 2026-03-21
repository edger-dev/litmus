---
# litmus-fp9r
title: Parse kitty.conf theme files
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:02Z
updated_at: 2026-03-20T17:08:30Z
order: z
parent: litmus-f1b3
---

Parse kitty.conf theme files directly as input (no canonical format yet).

## Summary of Changes

- Added  to  — parses  with/without prefix, case-insensitive
- Created  with  struct and  parser
  - Handles: name metadata comment, foreground/background, color0–color15, cursor, selection_background/foreground
  - Returns  if required fields missing; optional fields (cursor, selection) are  when absent
  - 4 unit tests all passing
- Added  to  — converts  →  with sensible defaults
- Updated : accepts  file paths as CLI args; if provided, uses only those themes; otherwise falls back to hardcoded themes
