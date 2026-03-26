---
# litmus-exy0
title: Configure R2 cache rules
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:35Z
updated_at: 2026-03-26T16:42:29Z
parent: litmus-v2g1
blocked_by:
    - litmus-mclx
---

Set up cache headers via Cloudflare cache rules (or R2 response headers):
- [x] Images (*.webp): 1 year edge + browser TTL via Cloudflare cache rule
- [x] manifest.json: 1 minute edge + browser TTL via Cloudflare cache rule
- [x] CORS: deferred to smoke test — custom domain through Cloudflare proxy should handle it
- [ ] Verify headers with curl -I (will check during smoke test)
