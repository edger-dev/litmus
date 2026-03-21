---
# litmus-t78w
title: Hardcoded app mock-ups
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:02Z
updated_at: 2026-03-20T13:31:50Z
order: s
parent: litmus-f1b3
---

Simple app mock-ups: hardcoded terminal output rendered with theme colors (fake git diff, ls output, shell prompt).

## Summary of Changes

- Created widgets/util.rs with shared to_ratatui_color() helper
- Created widgets/mockups.rs with MockupsWidget rendering hardcoded git diff and ls -la output using theme colors
- Updated widgets/swatches.rs to use the shared helper from util
- Updated widgets/mod.rs to export both widgets
- Updated main.rs with View enum (Swatches/Mockups) and Tab key cycling between views
