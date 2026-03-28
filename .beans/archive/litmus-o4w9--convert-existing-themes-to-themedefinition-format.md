---
# litmus-o4w9
title: Convert existing themes to ThemeDefinition format
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:23:04Z
updated_at: 2026-03-26T14:16:47Z
order: zzk
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

## Summary of Changes

Converted 54 existing hand-curated themes to the new ThemeDefinition format with provider mappings:

- Wrote  with manual name override dictionaries for kitty and wezterm vendor theme names
- Rewrote 54 theme .toml files from old [colors] format to new ThemeDefinition format (name, variant, [providers])
- Generated 87 provider color files (.kitty.toml and .wezterm.toml) via `litmus-capture extract-colors`
- Added variant detection with word-boundary matching and manual overrides for edge cases (e.g. Moonlight)
- 6 themes could not be converted due to no vendor match: Cyberdream Dark/Light, Melange Dark/Light, Light Owl, Oxocarbon Light
- 2 vendor extraction failures accepted (Sonokai kitty has only 15 ANSI colors in vendor data)
