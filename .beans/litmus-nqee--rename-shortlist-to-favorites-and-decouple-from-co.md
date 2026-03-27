---
# litmus-nqee
title: Rename shortlist to favorites and decouple from compare
status: completed
type: task
priority: normal
created_at: 2026-03-27T04:53:20Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzzzs
parent: litmus-ysy5
---

Decouple the bookmarking and comparison features.

- [x] Renamed Shortlist → Favorites everywhere (structs, components, CSS, labels)
- [x] Changed labels to use star icons (★/☆)
- [x] Raised MAX_FAVORITES from 5 to 20
- [ ] Persist favorites to localStorage (deferred)
- [x] Removed auto-build of compare URL from favorites
- [x] Sidebar favorites section decoupled from compare


## Summary of Changes

Renamed Shortlist to Favorites across all code, CSS, and UI labels. Changed toggle labels to use star icons. Raised limit from 5 to 20. Decoupled favorites from compare: removing a favorite no longer updates the compare URL, Feel Lucky no longer adds to favorites, and the compare nav link is independent. localStorage persistence deferred to a follow-up.
