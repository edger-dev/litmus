---
# litmus-yte1
title: Upgrade bacon export for richer Claude Code diagnostics
status: completed
type: task
priority: normal
created_at: 2026-03-20T13:58:06Z
updated_at: 2026-03-20T17:09:45Z
order: zy
---

Switch from deprecated export_locations to analyser exporter with cargo_json analyzer for structured diagnostics. Add claude-diagnostics job, update mise task, update CLAUDE.md.

## Summary of Changes

Updated bacon export to use analyser exporter with cargo_json analyzer.
