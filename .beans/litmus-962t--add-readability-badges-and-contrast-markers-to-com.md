---
# litmus-962t
title: Add readability badges and contrast markers to compare page
status: in-progress
type: task
priority: normal
created_at: 2026-03-27T04:53:08Z
updated_at: 2026-03-27T05:01:24Z
parent: litmus-ysy5
---

Wire contrast issues into the compare page — the highest-value change.

- [x] Call validate_fixtures_contrast() per theme in compare.rs
- [x] Add ScoreRing (readability %) to each column header
- [x] Add issue count badge next to readability score
- [x] Pass issue_details to TermOutputView (enables span markers, tooltips, footnotes)
- [x] Add per-fixture issue count badge on fixture headers in compare grid
