---
# litmus-exy0
title: Configure R2 cache rules
status: todo
type: task
priority: normal
created_at: 2026-03-26T15:42:35Z
updated_at: 2026-03-26T15:42:53Z
parent: litmus-v2g1
blocked_by:
    - litmus-mclx
---

Set up cache headers via Cloudflare cache rules (or R2 response headers):
- [ ] Images (*.webp, *.png): Cache-Control: public, max-age=31536000, immutable
- [ ] manifest.json: Cache-Control: public, max-age=60
- [ ] Add CORS headers: Access-Control-Allow-Origin: * (needed for cross-origin fetch from litmus.edger.dev)
- [ ] Verify headers with curl -I on a sample image and manifest.json
