---
# litmus-el0x
title: 'Fix: Missing Cargo.lock for crane/nix build'
status: completed
type: bug
priority: normal
created_at: 2026-03-20T05:10:54Z
updated_at: 2026-03-20T05:12:33Z
---

Crane requires Cargo.lock to be present and git-tracked. Generate it, git-add it, and add pname to commonArgs in flake.nix.

## Summary of Changes

- Generated `Cargo.lock` via `nix run nixpkgs#cargo -- generate-lockfile` (cargo not available in PATH)
- Staged `Cargo.lock` with `git add` so crane can see it
- Added `pname = "litmus";` to `commonArgs` in `flake.nix` to silence crane name warning
- Verified `nix develop --command echo OK` succeeds
