---
# litmus-v2g1
title: Deploy screenshots to Cloudflare R2
status: completed
type: epic
priority: normal
created_at: 2026-03-26T15:42:23Z
updated_at: 2026-03-26T17:08:57Z
---

End-to-end setup to serve screenshot images from Cloudflare R2 at screenshots.litmus.edger.dev. Includes bucket creation, custom domain, rclone-based upload sync, cache headers, and manifest deployment. Approach: local capture → rclone sync to R2 → served via custom domain with aggressive caching on images (immutable, 1yr) and short TTL (60s) on manifest.json.

## Infrastructure Setup

- **R2 Bucket**: `litmus-screenshots`
- **Endpoint URL**: `https://537f79be922377f50fb4ed655f6ab6b7.r2.cloudflarestorage.com/litmus-screenshots`
- **Account ID env var**: `CLOUDFLARE_APP_ID`
- **API Token env var**: `SCREENSHOTS_UPLOADING_API_TOKEN`
- **Target domain**: `https://screenshots.litmus.edger.dev`

## Summary of Changes

All 1560 screenshots (117.9 MiB) deployed to Cloudflare R2 and serving live at https://screenshots.litmus.edger.dev. Version bumped to 0.3.0.

### What was done
- R2 bucket `litmus-screenshots` created with custom domain + CORS policy
- rclone added to devShell for checksum-based sync
- mise tasks: `screenshots-sync` (sync only) and `screenshots-deploy` (manifest + sync)
- Production manifest built and deployed
- Web app verified working end-to-end

### Known issue
- Cloudflare cache rules showing `cf-cache-status: DYNAMIC` — may need zone-level investigation. Not blocking; content serves correctly.
