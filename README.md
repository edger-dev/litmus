# Litmus

> Litmus test your terminal themes — preview them across all your apps before you commit.

## The Problem

Switching terminal themes is a frustrating loop:

1. You find a theme that looks nice in your terminal's preview
2. You edit configs for kitty/wezterm, neovim, zellij, tig, delta...
3. You discover `git diff` is unreadable, or jjui's text blends into the background
4. You revert everything and try the next theme
5. Repeat

The core issue: **you can't see how a theme actually looks across your real workflow until after you've fully set it up.** Kitty's built-in theme preview shows ANSI swatches, but that doesn't tell you whether a complex `git diff` or a tig log view will be readable.

## What Litmus Does

A web app that lets you **preview any theme across realistic terminal scenes instantly** — before touching a single config file.

### Features

- **29 curated themes** from 15 families (Ayu, Catppuccin, Dracula, Everforest, Gruvbox, Horizon, Kanagawa, Material, Monokai, Moonlight, Nightfox, Nord, One Dark, Palenight, Rose Pine, Solarized, Tokyo Night)
- **8 realistic terminal scenes**: shell prompt, git diff, directory listing, cargo build, git log, neovim/code, Python REPL, system monitor
- **WCAG contrast validation**: automatic readability checks for every color pair in every scene
- **Side-by-side comparison**: compare 2–4 themes with color diff table
- **Browse by theme or by scene**: theme-first (pick a theme, see all scenes) or scene-first (pick a scene, see all themes)
- **Family grouping**: themes organized by family on the listing page
- **Light/dark detection**: automatic variant labeling with contrast ratios
- **Mini scene previews** on theme cards
- **Theme search** by name and family
- **Light/dark variant and contrast quality filters**
- **Tabbed scene navigation** with keyboard shortcuts (arrow keys, `c` to compare)
- **Multi-theme comparison** (2–4 themes, color diff table)
- **Compare accumulator**: floating bar to collect themes from any page
- **Config export**: kitty.conf, TOML, Nix with copy-to-clipboard
- **Color blindness simulation**: preview themes under protanopia, deuteranopia, and tritanopia (Machado 2009)

## Getting Started

### Prerequisites

- Rust toolchain (edition 2024)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) for the web frontend

### Running the CLI

```sh
cargo run -p litmus-cli
```

Navigate themes with arrow keys, switch views with Tab, press `q` to quit.

### Running the Web App

```sh
dx serve --project crates/litmus-web
```

Opens a local web server with the theme browser.

## Project Structure

```
crates/
  litmus-model/    Shared data model: Theme, Scene, contrast validation
  litmus-cli/      TUI theme previewer (ratatui + crossterm)
  litmus-web/      Web theme browser (Dioxus, targets wasm32)
themes/            29 curated themes in TOML format
```

### Key Modules

- **`litmus_model::scene`** — Scene format: `ThemeColor` (semantic color refs), `StyledSpan`, `SceneLine`, `Scene`
- **`litmus_model::scenes`** — 8 built-in terminal scenes
- **`litmus_model::contrast`** — WCAG 2.1 contrast ratio calculation and scene validation
- **`litmus_model::cvd`** — Color vision deficiency simulation (Machado 2009 matrices)
- **`litmus_web::scene_renderer`** — Dioxus components that render scenes as styled HTML

### Theme Format

Themes use a simple TOML format with 16 ANSI colors plus background, foreground, cursor, and selection colors. See `themes/` for examples. Parsers also support kitty.conf and base16 YAML.

## Architecture

### Semantic Color References

Scenes don't hardcode colors. Instead, each text span references a `ThemeColor` — `Foreground`, `Background`, `Ansi(0)` through `Ansi(15)`, `Cursor`, `SelectionBackground`, or `SelectionForeground`. At render time, these resolve against the current theme. This means scenes are defined once and render correctly for any theme.

### Provider/Consumer Model

Terminal apps relate to themes in fundamentally different ways:

- **Providers** (kitty, wezterm, alacritty) define the complete color palette
- **Consumers** (git diff, delta, ls, tig, bat) inherit colors from the provider via ANSI codes
- **Silos** (some apps with built-in themes) define their own isolated palette

Litmus scenes model the consumer perspective — showing how standard terminal output looks under any provider's palette.

## Future Directions

- Live terminal playground (xterm.js)
- Editor ecosystem previews (neovim, helix)
- Theme forking and customization
- Community-submitted scenes

## Contributing

_This project is in active development._
