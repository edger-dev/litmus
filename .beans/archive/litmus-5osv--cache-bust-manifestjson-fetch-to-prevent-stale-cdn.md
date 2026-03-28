---
# litmus-5osv
title: Cache-bust manifest.json fetch to prevent stale CDN cache
status: completed
type: task
priority: normal
created_at: 2026-03-25T15:35:21Z
updated_at: 2026-03-26T14:16:47Z
order: zzV
---

The browser cached an old CDN manifest.json, causing screenshots for newer fixtures to appear missing. The manifest URL has no cache-busting — switching from CDN to local dev still serves the cached CDN response.

## Design

Add a hash or timestamp query parameter to the manifest.json fetch URL to prevent browser caching. Options:
- Use a build-time hash (e.g. from git rev or compile timestamp) appended as `?v=<hash>`
- Or set `cache: 'no-cache'` on the fetch request in the JS code

The simplest fix is adding `cache: 'no-cache'` to the fetch options in main.rs's tryFetch function, since the manifest itself is small and should always be fresh.

## Tasks
- [x] Add cache-busting to manifest.json fetch in main.rs (tryFetch)
- [x] Verify dev and production both get fresh manifests
- [x] Test that screenshot images still load (they already have `?v=<checksum>` via cache_busted_url)

## Plan

Add `{ cache: 'no-cache' }` to the fetch call in the tryFetch JS function in main.rs. This tells the browser to revalidate with the server on every fetch (sends conditional request with If-None-Match/If-Modified-Since). The manifest is small (<600KB) so the revalidation overhead is negligible. No Rust code changes needed — just the embedded JS string.

## Summary of Changes

Added `cache: 'no-cache'` option to the `fetch()` call in the tryFetch JS function (main.rs). This forces the browser to revalidate the manifest with the server on every load, preventing stale CDN responses from being served from disk cache. Screenshot images are unaffected — they already use checksum-based cache busting.
