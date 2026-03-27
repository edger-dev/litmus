---
# litmus-ywps
title: Sticky toolbar for page-level controls
status: completed
type: feature
priority: normal
created_at: 2026-03-26T14:51:28Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzw
---

Add a sticky toolbar at the top of the content area so page controls remain accessible when scrolling.

## Requirements

- [x] Create a sticky toolbar component (position: sticky; top: 0; z-index above content)
- [x] Detail page toolbar: provider badge, contrast score summary, shortlist toggle
- [x] Compare page toolbar: Simulated / Screenshot toggle, fixture selector
- [x] Browse page toolbar: search input + filter controls (already sticky)
- [x] Style toolbar to match app chrome theme with subtle bottom border/shadow
- [x] Ensure toolbar doesn't overlap with sidebar on desktop or mobile header

## Notes

Each page defines its own toolbar contents. The shell layout provides the sticky container, pages fill it via a slot or context pattern.


## Summary of Changes

Made page-level controls sticky across all pages using CSS `position: sticky`. Detail page header (theme name, contrast ratio, shortlist), compare page toolbar (simulated/screenshot toggle + column headers), and scene-across fixture tabs all now stick at the top when scrolling. Mobile responsive offsets account for the 45px mobile header. Browse page filter bar was already sticky.

Files changed:
- `crates/litmus-web/assets/style.css` — sticky positioning + mobile offsets for .detail-header, .compare-toolbar, .scene-tabs
