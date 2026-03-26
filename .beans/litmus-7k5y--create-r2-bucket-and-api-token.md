---
# litmus-7k5y
title: Create R2 bucket and API token
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:30Z
updated_at: 2026-03-26T16:12:05Z
parent: litmus-v2g1
---

Manual setup via Cloudflare dashboard:
- [x] Create R2 bucket (e.g. `litmus-screenshots`)
- [x] Create API token with R2 read/write permissions (for rclone)
- [x] Store Access Key ID and Secret Access Key securely
  - I've create a API Token
  - already put it in environment: SCREENSHOTS_UPLOADING_API_TOKEN
- [x] Note the account ID and bucket endpoint URL
  - app id in env: CLOUDFLARE_APP_ID
  - endpoint URL: https://537f79be922377f50fb4ed655f6ab6b7.r2.cloudflarestorage.com/litmus-screenshots
- [x] Document the setup in the epic body
