---
# litmus-wrol
title: Build production manifest and deploy
status: todo
type: task
priority: normal
created_at: 2026-03-26T15:42:43Z
updated_at: 2026-03-26T15:42:53Z
parent: litmus-v2g1
blocked_by:
    - litmus-6gbb
    - litmus-mclx
---

Build manifest.json with production base_url and upload:
- [ ] Run capture-manifest with --base-url https://screenshots.litmus.edger.dev
- [ ] Include manifest.json in the rclone sync (it lives at the bucket root)
- [ ] Verify manifest.json is accessible at https://screenshots.litmus.edger.dev/manifest.json
- [ ] Add mise task `screenshots-deploy` that builds manifest + syncs everything
