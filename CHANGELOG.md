# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.5.0] - 2026-03-27

### Added
- Compare page: push both themes into visit history when selected
- Sidebar compare label shows "vs \<right>" or "\<left> vs \<right>" instead of "Side by Side"

## [0.4.0] - 2026-03-27

### Added
- Side-by-side compare: strict 2-theme layout with inline theme pickers
- Compare button on theme cards and detail page for quick comparisons
- Interactive contrast issue chips on compare page with click-to-cycle navigation
- Per-theme issue dots in sidebar fixture minimap (compare mode)
- Sidebar visit history showing last 5 viewed themes
- "vs. \<Theme>" sidebar button remembering the last compared theme
- Favorites star toggle replacing the old checkbox UI

### Changed
- Renamed "shortlist" to "favorites" with increased cap (5 -> 20)
- Decoupled favorites from compare functionality
- Compare capped at 2 themes (down from variable N)
- Apply button restyled to subtle outline; Compare button is now accent-filled

### Fixed
- Flash of unstyled content on initial WASM load
- Compare page scroll offset when navigating from sidebar minimap
- Cloudflare R2 cache headers: screenshots now served with proper Cache-Control

## [0.3.0] - 2026-03-26

### Added
- R2 screenshot deployment tooling (rclone sync via mise tasks)
- Provider-scoped URL routing (`/:provider/` prefix)
- Fixture anchor deep-links with hash-based scroll and URL updates
- Interactive contrast issue navigation with footnotes on detail page
- Contrast issue summary chips below detail header
- Sticky toolbars on detail, compare, and scene-across pages
- Theme availability feedback on browse page (unavailable themes shown greyed)
- Global ActiveProvider replacing per-page dropdown

### Changed
- Migrated rendering from Scene to TermOutput model
- Removed Scene/ThemeColor/StyledSpan types in favor of unified TermOutput
- Default capture geometry changed from 80x24 to 80x32 (4:3 aspect ratio)
- Reordered fixtures: color-showcase and editor-ui first

### Fixed
- Graceful provider switch with alert when theme unavailable in target provider
- Material theme wezterm mapping (use Material not MaterialDark)
- Sticky toolbar overflow-x clipping
- Scroll-to-fixture offset to clear sticky detail header
- Decorative spans excluded from contrast validation

## [0.2.0] - 2026-03-24

### Added
- Image-backed screenshot system with headless capture (kitty, wezterm)
- Side-by-side split view on detail page (simulated + screenshot)
- Scene minimap with vertical navigation and issue badges
- APCA-based readability scoring (replacing WCAG 2.x)
- Litmus branding with SVG score ring and theme card title bar
- CVD (color vision deficiency) simulation
- GitHub star button and Cloudflare Pages deployment
- Config export: kitty.conf, TOML, Nix formats
- Multi-theme compare with color diff table
- Keyboard navigation on detail page
- Wezterm provider support
- Theme collection expanded from 19 to 60 curated themes

### Changed
- Replaced top nav with sidebar layout and full-width content
- App theming: use any theme as the UI chrome
- Shortlist UX with cap at 5, Apply pushes to shortlist, Feel Lucky button
- Sidebar simplified to shortlist panel, filters moved to browse page

### Fixed
- Light-theme readability scoring
- Minimap placement and scoring consistency
- App theme reactivity and filter navigation

## [0.1.0] - 2026-03-21

### Added
- Initial release: Rust workspace with litmus-model, litmus-cli, litmus-web
- Curated theme library with 19 themes and auto-discovery
- Kitty.conf theme parser and CLI file loading
- TUI binary (ratatui + crossterm) with theme navigation
- Web frontend (Dioxus, WASM) with theme browsing and detail views
- Scene grid layout with compact rendering and tabs
