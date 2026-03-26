---
# litmus-7zy0
title: Investigate Cloudflare cache rules not applying to R2 custom domain
status: todo
type: bug
created_at: 2026-03-26T17:10:30Z
updated_at: 2026-03-26T17:10:30Z
---

Cache rules created under the correct zone for screenshots.litmus.edger.dev but cf-cache-status remains DYNAMIC on all requests. Images and manifest serve correctly, just not being cached at the edge.

## Context
- Two cache rules active: immutable images (1yr TTL) and manifest short TTL (1min)
- Both rules set to 'Eligible for cache' with 'Ignore cache-control header and use this TTL'
- R2 bucket serves via custom domain, CORS working
- Rules confirmed on the correct zone

## To investigate
- [ ] Check if R2 custom domain responses bypass cache by default
- [ ] Check Cloudflare docs for R2 + cache rules interaction
- [ ] Try a Cache Everything page rule as alternative
- [ ] Check if the zone plan level affects R2 caching behavior
