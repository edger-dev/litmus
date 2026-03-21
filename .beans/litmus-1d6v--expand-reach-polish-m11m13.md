---
# litmus-1d6v
title: Expand Reach & Polish (M11–M13)
status: completed
type: milestone
priority: normal
created_at: 2026-03-21T03:11:27Z
updated_at: 2026-03-21T03:20:42Z
---

M11: A11y & Mobile Polish, M12: Theme & Scene Expansion, M13: Color Blindness Simulation

## Summary of Changes

### Step 0: README Update
- Updated features list with M6-M10 features (search, filters, tabs, compare, export)
- Removed 'Config generation' from Future Directions (done)
- Updated theme/scene counts

### M11: Accessibility & Mobile Polish
- Extracted CSS custom properties: --color-bg, --color-fg, --color-accent, --color-success, --color-error, --color-border, --color-surface, --font-size-sm/base/lg
- Moved Shell inline style to .shell CSS class
- Added semantic HTML: role=navigation, role=tablist/tab with aria-selected, role=tabpanel, aria-pressed on filter/toggle buttons
- Added skip-to-content link and .sr-only class for screen reader label on search
- Added :focus-visible outlines for all interactive elements
- Added mobile responsiveness: 44px touch targets, stacked filter bar, full-width search, single-column compare grid at 640px
- Extracted compare grid to .compare-grid class with CSS custom property for column override

### M12: Theme & Scene Expansion
- Added 10 new themes: One Dark, Ayu Dark, Ayu Light, Moonlight, Nightfox, Kanagawa Dragon, Material, Palenight, Horizon, Monokai (19→29 themes)
- Reorganized Kanagawa into family directory (kanagawa.toml → kanagawa/kanagawa-wave.toml + kanagawa-dragon.toml)
- Added Ayu and Kanagawa to family.rs families list
- Added 3 new scenes: Neovim/Code (syntax highlighting, line numbers, LSP diagnostic), Python REPL (prompts, tracebacks, output), System Monitor/htop (CPU bars, memory, process list) (5→8 scenes)
- Updated README with new counts (29 themes, 15 families, 8 scenes)

### M13: Color Blindness Simulation
- Created cvd.rs module with Machado 2009 matrices for protanopia, deuteranopia, tritanopia
- Implemented sRGB↔linear RGB conversion with proper gamma handling
- Added simulate_cvd() and simulate_theme() functions
- 5 unit tests for CVD transforms
- Added CvdSelector component shared across all pages
- Integrated CVD toggle into ThemeDetail, CompareThemes, ThemeList, and SceneAcrossThemes
- Updated README features and architecture sections
