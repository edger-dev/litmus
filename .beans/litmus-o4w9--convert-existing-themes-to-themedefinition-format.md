---
# litmus-o4w9
title: Convert existing themes to ThemeDefinition format
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:23:04Z
updated_at: 2026-03-24T13:23:18Z
parent: litmus-knrz
blocked_by:
    - litmus-z20l
---

Write a script (or extend extract-colors) that:

1. Reads each existing themes/**/*.toml (which currently have [colors] sections)
2. Matches theme names to kitty/wezterm built-in theme names (fuzzy matching or manual mapping for popular ones)
3. Writes new authored .toml files with just name, variant, and [providers] section
4. Runs extract-colors to generate per-provider color files
5. Validates that extracted colors roughly match the old hand-curated colors (flag large diffs for review)

Start with the popular themes that have clear matches in both providers. Themes without provider matches stay in old format until manually mapped.

Depends on: extract-colors command
