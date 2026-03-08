# Litmus

> Litmus test your terminal themes — preview them across all your apps before you commit.

## The Problem

Switching terminal themes is a frustrating loop:

1. You find a theme that looks nice in your terminal's preview
2. You edit configs for kitty/wezterm, neovim, zellij, tig, delta...
3. You discover `git diff` is unreadable, or jjui's text blends into the background
4. You revert everything and try the next theme
5. Repeat

The core issue: **you can't see how a theme actually looks across your real workflow until after you've fully set it up.** Kitty's built-in theme preview shows ANSI swatches, but that doesn't tell you whether a complex `git diff` or a tig log view will be readable. The decision is visual, but the evaluation process is mechanical and slow — especially on NixOS where config changes require a home-manager rebuild.

## The Solution

A web app that lets you **preview any theme across all your terminal apps instantly**, with realistic sample content that exposes real readability issues.

Pick a theme. See exactly how `git diff`, neovim, tig, zellij, and more will look — before touching a single config file.

## Core Concepts

### The Three-Layer Theme Model

Terminal apps relate to themes in fundamentally different ways. Understanding this is key to the system's design.

**1. Theme Providers**

Apps that define a complete color palette independently. They are the "source of truth" for colors in their ecosystem.

Examples: kitty, wezterm, alacritty, neovim, helix

A provider's theme fully determines what you see — both for itself and for any consumer apps running inside it.

**2. Theme Consumers**

Apps that inherit colors from a provider. They use ANSI color codes or reference the provider's palette rather than defining their own.

Examples: `git diff`, `delta`, `ls --color`, `tig`, `bat`, `fd`, most CLI tools

A consumer **must be previewed within the context of a provider**. Showing `git diff` output alone is meaningless — it looks completely different under kitty+Tokyo Night vs kitty+Gruvbox.

**3. Theme Silos (and Dual-Mode Apps)**

Apps that define their own isolated theme, used only by themselves. Some apps can operate in both modes — e.g., `jjui` can use its own built-in theme (silo mode) or fall back to terminal ANSI colors (consumer mode).

The preview system should be able to show both modes for dual-mode apps.

### Provider Ecosystems

The provider/consumer relationship creates natural **ecosystems**:

- **Terminal ecosystem**: kitty (provider) → git diff, delta, tig, ls, bat, fd, jjui (consumers)
- **Editor ecosystem**: neovim (provider) → nvim-tree, telescope, lualine, which-key (consumers)
- **Editor ecosystem**: helix (provider) → (built-in UI elements as consumers)

A theme preview is most useful when it shows an entire ecosystem together — the provider plus its consumers rendering realistic content.

## MVP Scope

### What's Included

- **A curated set of high-quality themes** — approximately 10–20 hand-picked themes to start (e.g., Catppuccin Mocha/Latte, Tokyo Night, Gruvbox Dark/Light, Dracula, Nord, Rosé Pine, Solarized Dark/Light, Kanagawa, Everforest). Quality over quantity — a few well-integrated themes are far more useful than hundreds of broken ones.

- **Realistic app previews within provider ecosystems:**
  - Terminal provider (e.g., kitty): shell prompt, `ls` output, `git diff`, `delta` output, `tig` log view
  - Editor provider (e.g., neovim): source file with syntax highlighting, nvim-tree file browser, status line
  - At least one silo/dual-mode app preview

- **Static in-browser rendering** — monospace font, colored spans, realistic content. No live terminals. This is the sweet spot of fidelity vs. complexity.

- **Theme browsing UI** — browse by theme family, see all provider variants at a glance, compare themes side-by-side.

### What's Deferred

- Live terminal / interactive playground
- Config generation / Nix module output
- Automated theme data import pipeline
- Community features (sharing, forking, user-submitted scenes)
- Advanced font rendering (nerd font symbols, ligatures)
- Desktop app version

## Open Questions (To Discuss)

### Preview Content Design

How do we design sample content that genuinely exposes readability issues? This is critical to proving the tool's value over simple palette swatches.

Topics to resolve:

- What makes a "good" git diff sample? (merge conflicts, long lines, faint context, binary markers, rename detection, etc.)
- What source file(s) best test syntax highlighting? (multi-language? specific constructs that stress color differentiation?)
- How do we test "blending" problems — text that disappears into the background in certain themes?
- Should sample content be static or configurable per-user?
- Edge cases: very bright themes with light backgrounds, low-contrast themes, themes with unusual accent choices

### Theme Browsing UX Flow

How does a user navigate the theme space?

Options to explore:

- **Theme-first**: pick Catppuccin → see all ecosystems (terminal, neovim, etc.)
- **Provider-first**: pick kitty → browse themes for kitty with its consumers
- **Side-by-side**: compare two themes across the same ecosystem
- Hybrid approach: does the three-layer model suggest a natural hierarchy?

### Architecture & Data Model

To discuss in a future session:

- Theme data normalization: how to represent colors from heterogeneous sources (base16 YAML, kitty.conf, neovim Lua, etc.) in a unified format
- Our own theme definition standard — what fields, what constraints
- Rendering engine: how to go from theme data + sample content → colored preview in the browser
- Static site vs. dynamic app — does this need a backend at all for MVP?

## Long-Term Vision

The static preview MVP is the foundation. Future directions include:

- **Live terminal playground** — embed a real terminal (e.g., via xterm.js backed by a true terminal) so users can interactively try a theme, not just look at screenshots
- **Config generation** — "Apply this theme" produces ready-to-use config snippets for each app, including home-manager/Nix modules
- **Theme forking & customization** — tweak a base theme's colors and see changes live
- **Community scenes** — user-submitted sample content ("here's how it looks with a Rust project" or "here's my zellij layout")
- **Theme quality scoring** — automated contrast checks, WCAG readability analysis, color blindness simulation
- **Desktop integration** — a native app that can apply themes directly to running applications

## Contributing

_This project is in early planning. Stay tuned._

