---
# litmus-3zkh
title: Set up GitHub Pages for docs.litmus.edger.dev
status: completed
type: task
priority: normal
created_at: 2026-03-22T04:36:17Z
updated_at: 2026-03-23T10:26:52Z
order: zzzc
---

Manual steps needed to publish mdbook docs to GitHub Pages:

## Tasks

- [ ] Enable GitHub Pages in repo settings (Settings → Pages → Source: "GitHub Actions")
- [ ] Add DNS CNAME record: `docs.litmus.edger.dev` → `edger-dev.github.io`
- [ ] Configure custom domain in repo Settings → Pages → Custom domain: `docs.litmus.edger.dev`
- [ ] Verify HTTPS is enabled after DNS propagates
