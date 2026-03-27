---
# litmus-zg8b
title: 'Smoke test: verify live screenshots end-to-end'
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:47Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzzzz
parent: litmus-v2g1
blocked_by:
    - litmus-wrol
    - litmus-exy0
---

Full end-to-end verification:
- [x] Sync all screenshots to R2 (117.9 MiB, 1560 screenshots)
- [x] Verify a sample image loads (200 OK, 84 KB)
- [x] Verify manifest loads (200 OK, production base_url confirmed)
- [x] Check cache headers — cache rules still showing DYNAMIC, deferred to follow-up
- [x] CORS working: access-control-allow-origin: * (via R2 bucket CORS policy)
- [x] Load the live web app and confirm screenshots render correctly
- [x] Cache busting deferred — app is working end-to-end

## Summary of Changes

All screenshots serving live from https://screenshots.litmus.edger.dev. Manifest, images, and CORS all verified. Cache rules still showing cf-cache-status: DYNAMIC — may need zone-level investigation but not blocking.
