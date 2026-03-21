---
# litmus-yy2r
title: Multi-theme compare route
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:28Z
updated_at: 2026-03-20T18:45:07Z
parent: litmus-gspc
---

Change compare from /compare/:left/:right to support 2-4 themes. N-column layout with horizontal scroll on mobile.

## Summary of Changes

Changed compare route from /compare/:left/:right to /compare/:slugs with comma-separated slugs supporting 2-4 themes. N-column grid layout with compact scenes for 3+ themes. CompareSelector renders N dropdowns dynamically.
