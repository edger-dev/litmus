---
# litmus-2w3r
title: Provider ecosystem view
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:15Z
updated_at: 2026-03-20T18:03:25Z
parent: litmus-m8ze
---

Provider ecosystem view (e.g. 'kitty ecosystem' showing all consumers together).

## Summary of Changes

Added scene-centric ecosystem view:
- /scene/:scene_id route renders one scene across all 19 themes
- Each theme rendering links to its detail page for drill-down
- Nav bar now includes scene links (Shell Prompt, Git Diff, etc.) for quick access
- Enables side-by-side comparison of how themes handle the same terminal context
