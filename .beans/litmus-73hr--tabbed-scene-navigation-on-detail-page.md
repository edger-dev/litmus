---
# litmus-73hr
title: Tabbed scene navigation on detail page
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:16Z
updated_at: 2026-03-20T18:37:15Z
parent: litmus-74j8
---

Replace AllScenesView (stacked) with tab bar. Only one scene renders at a time. Signal-driven active tab.

## Summary of Changes

Replaced AllScenesView on the detail page with tabbed scene navigation. Added scene-tab buttons with active state styling. Only one scene renders at a time, driven by a use_signal index. CSS for .scene-tabs, .scene-tab, and .scene-tab-active added.
