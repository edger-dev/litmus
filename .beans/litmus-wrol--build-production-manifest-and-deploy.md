---
# litmus-wrol
title: Build production manifest and deploy
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:43Z
updated_at: 2026-03-26T17:00:37Z
parent: litmus-v2g1
blocked_by:
    - litmus-6gbb
    - litmus-mclx
---

Build manifest.json with production base_url and upload:
- [x] Run capture-manifest with --base-url https://screenshots.litmus.edger.dev
- [x] Include manifest.json in the rclone sync (it lives at the bucket root)
- [x] Verify manifest.json is accessible at https://screenshots.litmus.edger.dev/manifest.json
- [x] Added mise tasks: `screenshots-sync` and `screenshots-deploy`
