---
# litmus-gzd5
title: GitHub star button + Cloudflare Pages deployment prep
status: completed
type: feature
priority: normal
created_at: 2026-03-21T12:44:10Z
updated_at: 2026-03-21T13:10:26Z
order: zw
---

- [x] Add GitHubStars component to components.rs
- [x] Add star button to sidebar header row
- [x] Add CSS for star button and header row
- [x] Create _redirects file for SPA routing
- [x] Create _headers file for security/cache headers
- [x] Create GitHub Actions deploy workflow
- [x] Create beans for manual Cloudflare/DNS tasks

## Summary of Changes

- Added `GitHubStars` component using `use_resource` + `eval()` to fetch stargazer count from GitHub API
- Restructured sidebar header with flexbox row for logo + star pill button
- Created `_redirects` (SPA fallback) and `_headers` (security + caching) for Cloudflare Pages
- Created GitHub Actions workflow (`deploy.yml`) that builds WASM and deploys via wrangler
- Created follow-up beans for manual Cloudflare dashboard and DNS setup
