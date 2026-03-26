---
# litmus-6gbb
title: Set up rclone config and upload script
status: completed
type: task
priority: normal
created_at: 2026-03-26T15:42:40Z
updated_at: 2026-03-26T16:25:39Z
parent: litmus-v2g1
blocked_by:
    - litmus-7k5y
---

Create rclone-based sync tooling:
- [x] Add rclone to project dev dependencies (added to flake.nix devShell)
- [x] Create rclone config for R2 (inline flags, no config file needed)
- [x] Write mise task `screenshots-sync` with --checksum flag
- [x] Ensure correct Content-Type headers are set on upload (rclone auto-detects from extension)
- [x] Tested with dry-run, then full upload (117.9 MiB, ~1000 files)
- [x] Env vars: R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY, R2_ENDPOINTS (set via shadowenv)

## Summary of Changes

- Added `rclone` to flake.nix devShell packages
- Added `screenshots-sync` mise task: rclone sync with checksum-based diffing
- Added `screenshots-deploy` mise task: builds production manifest + syncs
- All env vars passed inline (no rclone.conf needed)
- Successfully uploaded all screenshots to R2 bucket
