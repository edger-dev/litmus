---
# litmus-yesx
title: Set up mdbook docs site
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:26:42Z
updated_at: 2026-03-20T17:09:45Z
order: zw
---

Add mdbook documentation site with book.toml, src/ files, mise tasks, flake.nix integration, and .gitignore update

## Summary of Changes

- Created `docs/book.toml` (title: Litmus, src: src, build-dir: dist)
- Created `docs/src/SUMMARY.md` with 4-page TOC
- Created `docs/src/introduction.md` from README problem/solution section
- Created `docs/src/concepts.md` with three-layer model and provider ecosystems
- Created `docs/src/milestones.md` adapted from existing docs/milestones.md
- Created `docs/src/contributing.md` placeholder
- Added `pkgs.mdbook` to flake.nix devShells packages
- Added `docs-serve` and `docs-build` tasks to .mise.toml
- Added `docs/dist/` to .gitignore
- Verified: `mdbook build docs` produces `docs/dist/` successfully
