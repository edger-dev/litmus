# Development

This chapter covers setting up a development environment and working on litmus.

## Prerequisites

- **Rust toolchain**: defined in `rust-toolchain.toml` — stable channel with the `wasm32-unknown-unknown` target
- **Nix** (optional): the `flake.nix` provides a reproducible dev shell with all dependencies
- **mise**: task runner used for all dev commands

## Quick start

```bash
# Start the web app (Dioxus dev server, port 8883)
mise run dev

# Run the TUI prototype
mise run _cli

# Serve the docs with live reload (port 8882)
mise run _docs-serve
```

## mise tasks

| Task | Description |
|------|-------------|
| `dev` | Start Dioxus dev server (port 8883) |
| `build-web` | Build web release |
| `build-cli` | Build CLI release |
| `check` | Run `cargo check` across workspace |
| `fmt` | Format code with `cargo fmt` |
| `docs-serve` | Serve mdbook with live reload (port 8882) |
| `docs-build` | Build static docs |
| `bacon-claude-diagnostics` | Export compiler diagnostics to `.bacon-claude-diagnostics` |

## Development loop

The recommended workflow uses **bacon** for continuous compilation feedback:

1. Start bacon in a terminal pane: `mise run bacon-claude-diagnostics`
2. Edit code
3. bacon watches for changes and writes compiler diagnostics to `.bacon-claude-diagnostics`
4. Read that file to see errors with exact file/line/column locations
5. Fix and repeat

For quick one-off checks: `mise run check` (type-check) or `mise run fmt` (format).

## Project structure

See the [Architecture](./architecture.md) chapter for details on the three crates, data model, scene system, and web app structure.

## Work tracking

litmus uses **beans**, an agentic-first issue tracker. Work is organized into milestones, each containing focused task and feature beans. See [Milestones](./milestones.md) for the full history and [Agentic Workflow](./agentic-workflow.md) for how beans fits into the development process.
