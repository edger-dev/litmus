---
# litmus-j0d6
title: 'M4: Cloudflare R2 Storage Setup'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:13Z
updated_at: 2026-03-23T10:02:15Z
parent: litmus-k2id
blocked_by:
    - litmus-b10b
---

Create R2 bucket litmus-screenshots with public access and custom domain. Configure CORS. Document URL scheme: {base_url}/v1/{provider}/{theme}/{fixture}.webp. Add R2 credentials to GitHub Actions secrets. Note: manual infra step, document the process.

## Summary of Changes

M4 is primarily a manual infrastructure setup task. The codebase is ready for R2 integration — the manifest model, URL scheme, and upload hooks are designed.

**Setup steps for the user:**
1. Create R2 bucket `litmus-screenshots` in Cloudflare dashboard
2. Enable public access with custom domain `screenshots.litmus.edger.dev`
3. Configure CORS:
   - Allowed origin: `https://litmus.edger.dev`
   - Allowed methods: GET
4. Add GitHub Actions secrets:
   - `R2_ACCESS_KEY_ID`
   - `R2_SECRET_ACCESS_KEY`
   - `R2_ACCOUNT_ID`
   - `R2_ENDPOINT` (e.g. `https://${ACCOUNT_ID}.r2.cloudflarestorage.com`)
   - `LITMUS_SCREENSHOTS_BASE_URL` (e.g. `https://screenshots.litmus.edger.dev`)

**URL scheme implemented:**
`{base_url}/v1/{provider}/{theme}/{fixture}.webp`

**Cache headers to configure on the bucket:**
- Images: `Cache-Control: public, max-age=31536000, immutable`
- manifest.json: `Cache-Control: public, max-age=3600`

No code changes needed for M4 — the data model and CLI upload command are ready in M3.
