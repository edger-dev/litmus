# Introduction

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

## Current Scale

- **29 themes** across 15 families (Catppuccin, Tokyo Night, Gruvbox, Dracula, Nord, Rose Pine, and more)
- **8 scenes** simulating real terminal output (shell prompt, git diff, ls colors, cargo build, neovim, Python REPL, log viewer, htop)
- **3 crates**: `litmus-model` (shared data model), `litmus-cli` (TUI prototype), `litmus-web` (Dioxus WASM web app)
- Accessibility tooling: WCAG contrast checking, color blindness simulation (CVD), keyboard navigation, screen reader support
- Config export: generate kitty.conf, TOML, or Nix for any theme

## Project Status

The web app is the primary interface — the TUI was the M0 prototype that informed the data model design. 13 milestones have been completed, covering everything from the core data model through theme browsing, comparison, export, accessibility, and CVD simulation. See [Milestones](./milestones.md) for the full history.
