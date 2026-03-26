---
# litmus-mclx
title: Configure custom domain for R2 bucket
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:32Z
updated_at: 2026-03-26T16:33:58Z
parent: litmus-v2g1
blocked_by:
    - litmus-7k5y
---

Point screenshots.litmus.edger.dev to the R2 bucket:
- [x] Enable public access on the R2 bucket with custom domain
- [x] Add CNAME record (auto-created by Cloudflare custom domain flow)
- [x] Verify SSL/TLS is working on the custom domain
- [x] Confirm bucket contents are accessible at https://screenshots.litmus.edger.dev/
