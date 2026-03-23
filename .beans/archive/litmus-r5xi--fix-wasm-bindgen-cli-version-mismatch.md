---
# litmus-r5xi
title: Fix wasm-bindgen-cli version mismatch
status: completed
type: bug
priority: normal
created_at: 2026-03-22T10:35:58Z
updated_at: 2026-03-23T10:26:52Z
order: zzzk
---

dx build fails because pkgs.dioxus-cli bundles wasm-bindgen-cli 0.2.108 but Cargo.lock requires 0.2.114. Fix by pinning wasm-bindgen-cli in flake.nix and CI.

## Summary of Changes

- Added  to  devShell packages so it takes precedence over the version bundled by 
- Pinned  in  and added explicit  install step for CI
