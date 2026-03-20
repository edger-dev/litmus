---
# litmus-7rix
title: Litmus repo skeleton setup
status: completed
type: task
priority: normal
created_at: 2026-03-20T02:43:57Z
updated_at: 2026-03-20T02:45:09Z
---

Set up build system and code skeleton for the litmus terminal color theme previewer. Includes Cargo workspace, three crates (litmus-model, litmus-web, litmus-cli), flake.nix, and updates to existing files.

## Summary of Changes

Created the full repo skeleton:

- **Cargo.toml** (workspace root): resolver 2, workspace package defaults, shared deps
- **rust-toolchain.toml**: stable channel, wasm32-unknown-unknown target, rustfmt+clippy
- **crates/litmus-model**: lib crate with `Color` and `Theme` structs, serde derive
- **crates/litmus-web**: bin crate with Dioxus 0.7 hello-world app, web feature flag, Dioxus.toml
- **crates/litmus-cli**: bin crate with stub main
- **flake.nix**: fenix stable toolchain + wasm target, crane builds litmus-cli, clippy+fmt checks, devShell with dioxus-cli and just
- **.gitignore**: added `result`, `result-*`, `dist/`
- **justfile**: added `dev`, `build-web`, `build-cli`, `check`, `fmt` recipes
