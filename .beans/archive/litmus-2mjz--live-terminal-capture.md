---
# litmus-2mjz
title: Live terminal capture
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:02Z
updated_at: 2026-03-20T16:49:09Z
order: w
parent: litmus-f1b3
---

Run real commands (git diff, ls) and display output with theme colors applied.

## Plan

- [x] Create  with  that takes real command output and applies theme colors
- [x] Update  to export 
- [x] Update  to capture command output and add  (Tab cycles: Swatches → Mockups → Live)

## Summary of Changes

Added `widgets/live.rs` with `LiveWidget` that captures real `git diff` and `ls -la` output at startup and renders it with theme colors applied. Git diff lines are colored by prefix (+/-/@@/headers); ls entries are colored by file type (directory/symlink/executable/hidden). Tab now cycles Swatches → Mockups → Live.
