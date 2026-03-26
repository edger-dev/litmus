---
# litmus-zg8b
title: 'Smoke test: verify live screenshots end-to-end'
status: in-progress
type: task
priority: normal
created_at: 2026-03-26T15:42:47Z
updated_at: 2026-03-26T17:00:47Z
parent: litmus-v2g1
blocked_by:
    - litmus-wrol
    - litmus-exy0
---

Full end-to-end verification:
- [x] Sync all screenshots to R2 (117.9 MiB, 1560 screenshots)
- [x] Verify a sample image loads (200 OK, 84 KB)
- [x] Verify manifest loads (200 OK, production base_url confirmed)
- [ ] Check cache headers are correct on both (still DYNAMIC — cache rules not taking effect yet)
- [x] CORS working: access-control-allow-origin: * (via R2 bucket CORS policy)
- [ ] Load the live web app and confirm screenshots render correctly
- [ ] Test cache busting by re-capturing one screenshot and re-syncing
