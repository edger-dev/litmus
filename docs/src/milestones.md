# Milestones

litmus development is organized into milestones, each delivering a focused set of features. 13 milestones have been completed, taking the project from a TUI prototype to a full-featured web app with 29 themes, 8 scenes, accessibility tooling, and config export.

## M0: TUI Prototype

Built a terminal-based preview tool using ratatui to explore theme rendering before committing to a data model. Parsed kitty.conf files directly, displayed ANSI color swatches, and rendered hardcoded terminal mock-ups with theme colors applied.

## M1: Theme Data Model & Parsing

Defined the unified theme representation — `Color`, `AnsiColors` (16 named fields), and `Theme` struct. Established TOML as the canonical theme format. Implemented parsers for kitty.conf and base16 YAML. Added validation and unit tests.

## M2: Theme Curation

Built the initial curated theme library with ~15 high-quality themes: Catppuccin (4 variants), Tokyo Night (3), Gruvbox (2), Dracula, Nord, Rose Pine (3), Solarized (2), Kanagawa, and Everforest (2). Organized by theme family with prefix-based grouping.

## M3: Terminal Ecosystem Rendering

Created the scene system — the core abstraction where `ThemeColor` enum references semantic color slots rather than hardcoded RGB values. Built `StyledSpan` builder API and initial scenes (shell prompt, git diff, ls colors). Implemented the web renderer that resolves theme colors to CSS inline styles.

## M4: Theme Browsing UI

Launched the Dioxus WASM web app with a theme listing page (family-grouped cards with swatch strips), single-theme detail page showing all scenes, and a responsive monospace layout.

## M5: Comparison & Polish

Added side-by-side theme comparison, theme-first and provider-first navigation, and visual polish for edge cases (very bright/dark themes, low-contrast text).

## M6: Filters & Mini Previews

Added mini scene previews to theme cards on the home page. Implemented search filtering by theme name/family. Added light/dark variant filter and WCAG contrast quality filter.

## M7: Theme Detail Redesign

Redesigned the theme detail page with tabbed scene views. Added keyboard navigation (arrow keys to cycle scenes). Introduced the compare accumulator — press 'c' to collect themes for comparison.

## M8: Scene Grid & Compact Rendering

Implemented scene grid layout for viewing scenes side by side. Added compact rendering mode for denser previews. Introduced scene tabs for quick switching between scenes.

## M9: Multi-Theme Compare

Extended comparison from 2 to 2–4 themes side by side. Added a color diff table showing how palette values differ across compared themes.

## M10: Config Export

Added export functionality — generate kitty.conf, TOML, or Nix attribute set for any theme. Built a share/choose flow for copying or downloading the exported config.

## M11: Accessibility & Mobile

CSS custom properties for layout tokens. Semantic HTML elements and ARIA attributes for screen reader support. `:focus-visible` indicators for keyboard users. Responsive layout down to mobile viewports.

## M12: Theme & Scene Expansion

Expanded from 19 to 29 themes — added Ayu, Horizon, Material, Monokai, Moonlight, Nightfox, One Dark, Palenight, and additional Kanagawa variants. Grew from 5 to 8 scenes — added cargo build output, Python REPL, and htop scenes.

## M13: Color Blindness Simulation

Implemented CVD (color vision deficiency) simulation using Machado et al. 2009 transformation matrices. Three modes: protanopia, deuteranopia, tritanopia. Transforms entire themes through the simulation pipeline so all existing scenes and views work without modification.
