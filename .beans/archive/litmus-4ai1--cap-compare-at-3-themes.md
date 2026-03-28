---
# litmus-4ai1
title: Cap compare at 3 themes
status: completed
type: task
priority: normal
created_at: 2026-03-27T04:53:18Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzzk
parent: litmus-ysy5
---

Enforce a maximum of 3 themes in side-by-side compare.

- [x] Change MAX_COMPARE from 4 to 3 in state.rs
- [x] Truncate slug list to 3 in URL parsing
- [x] Update sidebar compare button to respect the cap
- [x] N/A — shortlist holds up to 5 but compare URL is capped; decoupling is in litmus-nqee
- [x] Ensure CSS grid adapts well to 2 and 3 columns with contrast markers


## Summary of Changes

Changed MAX_COMPARE from 4 to 3. Sidebar compare URL builder stops at 3 slugs. Compare page URL parser uses .take(MAX_COMPARE). Compact mode triggers at 3 columns.
