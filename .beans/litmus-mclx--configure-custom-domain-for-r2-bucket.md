---
# litmus-mclx
title: Configure custom domain for R2 bucket
status: in-progress
type: task
priority: normal
created_at: 2026-03-26T15:42:32Z
updated_at: 2026-03-26T16:12:23Z
parent: litmus-v2g1
blocked_by:
    - litmus-7k5y
---

Point screenshots.litmus.edger.dev to the R2 bucket:
- [ ] Enable public access on the R2 bucket with custom domain
- [ ] Add CNAME record: screenshots.litmus.edger.dev → R2 bucket domain
- [ ] Verify SSL/TLS is working on the custom domain
- [ ] Confirm bucket contents are accessible at https://screenshots.litmus.edger.dev/
