---
# litmus-p5xo
title: Per-theme issue dots in sidebar fixture minimap
status: todo
type: task
priority: normal
created_at: 2026-03-27T04:53:15Z
updated_at: 2026-03-27T04:53:28Z
parent: litmus-ysy5
blocked_by:
    - litmus-962t
---

Show per-theme colored dots on the sidebar fixture minimap during compare.

- [ ] Extend SceneIssueCounts to support per-theme counts (HashMap<fixture_id, Vec<(theme_slug, count)>>)
- [ ] Assign a color to each compared theme (match column header)
- [ ] Render 2-3 small colored dots per fixture in the minimap
- [ ] Each dot represents one theme's issue count for that fixture
- [ ] Hide dots when not on compare page (fall back to single count for detail page)
