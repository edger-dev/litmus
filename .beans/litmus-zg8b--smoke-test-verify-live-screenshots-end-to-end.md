---
# litmus-zg8b
title: 'Smoke test: verify live screenshots end-to-end'
status: todo
type: task
priority: normal
created_at: 2026-03-26T15:42:47Z
updated_at: 2026-03-26T15:42:53Z
parent: litmus-v2g1
blocked_by:
    - litmus-wrol
    - litmus-exy0
---

Full end-to-end verification:
- [ ] Sync all screenshots to R2
- [ ] Verify a sample image loads: curl -I https://screenshots.litmus.edger.dev/kitty/catppuccin-mocha/bat-syntax.webp
- [ ] Verify manifest loads: curl -I https://screenshots.litmus.edger.dev/manifest.json
- [ ] Check cache headers are correct on both
- [ ] Check CORS headers work from litmus.edger.dev origin
- [ ] Load the live web app and confirm screenshots render correctly
- [ ] Test cache busting by re-capturing one screenshot and re-syncing
