---
# litmus-gyq0
title: Theme listing page with family grouping
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:15Z
updated_at: 2026-03-20T18:01:32Z
parent: litmus-m8ze
---

Theme listing page organized by theme family.

## Summary of Changes

Added family grouping to theme listing page:
- `family.rs`: theme_family() extracts family from name using known prefix list; group_by_family() groups themes
- Listing page now shows themes organized under family headings (Catppuccin, Gruvbox, etc.)
- Standalone themes (Dracula, Nord, Kanagawa) appear as their own single-theme groups
