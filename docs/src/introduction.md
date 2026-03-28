# Introduction

> Litmus test your terminal themes — preview them across all your apps before you commit.

<a href="https://litmus.edger.dev"><img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32' fill='none'%3E%3Crect width='32' height='32' rx='6' fill='%231a1b26'/%3E%3Crect x='6' y='5' width='5' height='22' rx='2.5' fill='%23ff6b6b'/%3E%3Crect x='13.5' y='5' width='5' height='22' rx='2.5' fill='%23ffd93d'/%3E%3Crect x='21' y='5' width='5' height='22' rx='2.5' fill='%236bcb77'/%3E%3C/svg%3E" alt="Litmus" height="20" style="vertical-align: middle;" /></a> Try it live at [**litmus.edger.dev**](https://litmus.edger.dev)

## The Problem

Switching terminal themes is a frustrating loop:

1. You find a theme that looks nice in a preview
2. You edit configs for kitty, wezterm, neovim, zellij, delta...
3. You discover `git diff` is unreadable, or cargo warnings blend into the background
4. You revert everything and try the next theme
5. Repeat

The core issue: **you can't see how a theme actually looks across your real workflow until after you've fully set it up.** A terminal's built-in theme preview shows color swatches, but that doesn't tell you whether a `git diff` or a `cargo build` will be readable. The decision is visual, but the evaluation process is mechanical and slow — especially on NixOS where config changes require a home-manager rebuild.

## The Solution

A web app that lets you **preview any theme across realistic terminal scenarios instantly** — before touching a single config file.

Pick a theme. See exactly how `git diff`, cargo build output, bat syntax highlighting, and more will look across different terminal emulators. Litmus captures real screenshots from real terminals, so what you see is exactly what you'd get.

## What's In the Box

- **58 themes** across 30+ families (Catppuccin, Tokyo Night, Gruvbox, Dracula, Nord, Rose Pine, and many more)
- **13 fixtures** simulating real terminal output (git diff, cargo build, bat, ripgrep, shell prompt, ls, htop, and more)
- **2 providers** — kitty and wezterm, with real per-provider screenshots
- **Side-by-side comparison** of any two themes
- **Accessibility tooling** — WCAG contrast checking, APCA readability scoring, color vision deficiency simulation (protanopia, deuteranopia, tritanopia)
- **Config export** — generate kitty.conf, TOML, or Nix attribute set for any theme

## How It Works

Litmus captures real terminal screenshots by running actual commands in headless terminal emulators, then serves them through a web app where you can browse, compare, and evaluate themes. It also parses the ANSI output from those same commands to provide simulated rendering — enabling features like contrast analysis and CVD simulation that can't work on raster images alone.

Read [The Model](./model/README.md) to understand the conceptual framework that makes this work.
