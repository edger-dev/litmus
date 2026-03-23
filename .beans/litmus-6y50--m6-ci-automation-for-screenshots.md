---
# litmus-6y50
title: 'M6: CI Automation for Screenshots'
status: completed
type: task
priority: normal
created_at: 2026-03-23T09:46:21Z
updated_at: 2026-03-23T10:08:25Z
parent: litmus-k2id
blocked_by:
    - litmus-lizo
    - litmus-j0d6
---

GitHub Actions workflow screenshots.yml: workflow_dispatch + push trigger on themes/** and fixtures/**. Installs Nix, runs litmus-capture capture-all, uploads to R2, updates manifest.

## Summary of Changes

Created `.github/workflows/screenshots.yml` with:
- Triggers: workflow_dispatch (with optional provider/theme/fixture filters) + push on themes/**, fixtures/**, crates/litmus-capture/**
- Environment: WLR_BACKENDS=headless + WLR_RENDERER=pixman + LIBGL_ALWAYS_SOFTWARE=1 for GPU-free headless Wayland
- Steps: Nix install → cargo build litmus-capture → capture-all → manifest build → coverage check → aws s3 upload to R2
- Uses R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY, R2_ENDPOINT GitHub secrets
- Reports screenshot count and staging size in job summary

Note: Headless Wayland capture requires testing on actual runner; may need to add Mesa software rendering packages or adjust WLR/GL env vars.
