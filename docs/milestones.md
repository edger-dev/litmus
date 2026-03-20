# Litmus MVP — Milestones

## Context

The MVP scope is defined in README.md: curated themes, realistic app previews within provider ecosystems, static in-browser rendering, and a theme browsing UI. We start with a TUI prototype to inform data model design and serve as a playground, then build out 5 more milestones. Terminal-first; editor ecosystem is deferred to post-MVP.

## Milestones

### M0: TUI Prototype
A terminal-based preview tool to explore theme rendering before committing to a data model. Serves as a design playground and sanity check.

- Parse kitty.conf theme files directly as input (no canonical format yet)
- ANSI color swatches: display the 16 ANSI colors + fg/bg/cursor/selection as colored blocks
- Simple app mock-ups: hardcoded terminal output rendered with theme colors (fake git diff, ls output, shell prompt)
- Live terminal capture: run real commands (git diff, ls) and display output with theme colors applied
- TUI navigation: switch between themes, toggle between swatches/mock-ups/live views
- Use ratatui or similar TUI framework

### M1: Theme Data Model & Parsing
Define the unified theme format and implement parsers, informed by learnings from M0.

- Design the internal theme representation (color palette schema — ANSI 0-15, foreground, background, cursor, selection, etc.)
- Define a canonical theme file format (likely TOML or JSON)
- Implement parsers for at least: kitty.conf, base16 YAML
- Validation and error handling for theme data
- Unit tests for parsing

### M2: Theme Curation
Build the initial curated theme library.

- Select 10-20 high-quality themes (Catppuccin Mocha/Latte, Tokyo Night, Gruvbox Dark/Light, Dracula, Nord, Rose Pine, Solarized Dark/Light, Kanagawa, Everforest, etc.)
- Convert each to the canonical format
- Organize by theme family (e.g., Catppuccin family has Mocha, Latte, Frappe, Macchiato)
- Quality checks: ensure all required colors are present, no missing values

### M3: Terminal Ecosystem Rendering
Build the web rendering engine and terminal provider/consumer previews.

- Core renderer: theme data + annotated content → colored HTML spans (monospace)
- Define "scene" format — annotated sample content with semantic color references
- Create realistic terminal scenes:
  - Shell prompt
  - `ls --color` output
  - `git diff` output (with context, additions, deletions, merge conflicts)
  - `delta` output
  - `tig` log view
- Ensure scenes expose real readability issues (low contrast, blending)

### M4: Theme Browsing UI
Web app for browsing and viewing theme previews.

- Web app shell (likely a static site — Rust backend optional, could be pure frontend)
- Theme listing page with family grouping
- Single-theme detail page showing all ecosystem previews
- Provider ecosystem view (e.g., "kitty ecosystem" showing all consumers together)
- Responsive layout, monospace font rendering

### M5: Comparison & Polish
Side-by-side comparison and final MVP polish.

- Side-by-side theme comparison (same scene, two themes)
- Theme-first vs provider-first navigation (or hybrid)
- Visual polish, edge case handling (very bright/dark themes, low contrast)
- README update with usage instructions

## Post-MVP: Editor Ecosystem Rendering
Deferred — acceptable to ship MVP without this.

- Neovim scenes: syntax highlighting, nvim-tree, lualine statusline
- Silo/dual-mode app preview (e.g., jjui)
- Scene format should be reusable across providers

## Implementation Notes

- M0 (TUI) is deliberately rough — its purpose is to inform M1's data model, not to be production-quality
- M0 reads kitty.conf directly; M1 introduces the canonical format based on what we learn
- Terminal ecosystem (M3) is the core value prop
- Each milestone includes its own design decisions (no separate design phase)
- M1 and M2 are foundational for the web app — everything from M3 onward depends on them
- M4 can start once M3 has basic rendering working

## Verification

After each milestone:
- M0: TUI launches, can load a kitty theme, shows swatches + mock-ups + live output, switch themes
- M1: `cargo test` passes, can parse kitty.conf and base16 YAML into internal format
- M2: All curated themes load and validate successfully
- M3: Can generate HTML preview of terminal scenes with any curated theme
- M4: Web app serves theme browser, can navigate themes and see previews
- M5: Can compare two themes side-by-side, all edge cases handled
