---
# litmus-7zy0
title: Investigate Cloudflare cache rules not applying to R2 custom domain
status: completed
type: bug
priority: normal
created_at: 2026-03-26T17:10:30Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzz
---

Cache rules created under the correct zone for screenshots.litmus.edger.dev but cf-cache-status remains DYNAMIC on all requests. Images and manifest serve correctly, just not being cached at the edge.

## Context
- Two cache rules active: immutable images (1yr TTL) and manifest short TTL (1min)
- Both rules set to 'Eligible for cache' with 'Ignore cache-control header and use this TTL'
- R2 bucket serves via custom domain, CORS working
- Rules confirmed on the correct zone

## To investigate
- [x] Check if R2 custom domain responses bypass cache by default
- [x] Check Cloudflare docs for R2 + cache rules interaction
- [x] Try a Cache Everything page rule as alternative — not needed
- [x] Check if the zone plan level affects R2 caching behavior — no, affects all plans

## Summary of Changes

**Root cause**: R2 does not set `Cache-Control` headers by default. Cloudflare cache rules do not reliably override this for R2 custom domains — a known limitation where R2 responses bypass the normal cache layer.

**Fix**: Updated `screenshots-sync` mise task to set `Cache-Control` headers via rclone `--header-upload`:
- Images: `public, max-age=31536000, immutable` (1 year)
- Manifest: `public, max-age=60` (1 minute)

Added `screenshots-sync_ignore-checksum` task for one-time force re-upload.

Verified: `cf-cache-status: HIT` confirmed on both manifest and images.
