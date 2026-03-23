---
# litmus-qm77
title: 'Web UI/UX overhaul: sidebar + full-width + app theming'
status: completed
type: feature
priority: normal
created_at: 2026-03-21T05:00:36Z
updated_at: 2026-03-21T13:10:26Z
order: zzk
---

Replace top nav + floating compare bar with persistent left sidebar. Full-width layout. App chrome theming via CSS custom properties. Mobile drawer. CRT easter egg.

## Summary of Changes

### Phase 1: File Split + State Consolidation
- Split `main.rs` (1269 lines) into modular files:
  - `state.rs` — Global signals: CompareSelection, FilterState, ActiveScene, AppThemeSlug, SidebarOpen + helper functions
  - `components.rs` — Shared components: FilterButton, ColorSwatch, CompareToggle, CvdSelector, ExportButtons, ColorDiffTable
  - `pages/theme_list.rs` — ThemeList (home grid)
  - `pages/theme_detail.rs` — ThemeDetail
  - `pages/scene_across.rs` — SceneAcrossThemes
  - `pages/compare.rs` — CompareThemes + CompareSelector
  - `sidebar.rs` — Persistent sidebar with all navigation
  - `shell.rs` — Shell layout (sidebar + main area)
- `main.rs` reduced to ~47 lines: App, Route, main() only
- All filter/compare state lifted to global signals via use_context_provider

### Phase 2: Sidebar + Layout
- Replaced top nav + floating CompareBar with persistent 280px left sidebar
- Sidebar sections: logo, search, filters (All/Dark/Light/contrast), CVD selector, scrollable theme list (family-grouped with color dots), scene chips, compare management, app theme dropdown
- Main content area is full-width (fluid)
- Two-column flexbox layout

### Phase 3: App Theming
- 12 CSS custom properties (`--app-*`) dynamically set from any selected theme
- `theme_to_css_vars_js()` generates JS to set all properties on documentElement
- `use_effect` in Shell watches AppThemeSlug and applies theme
- Default Tokyo Night-inspired colors as fallback
- Light theme detection via luminance → sets `data-theme="light"` for CSS adjustments
- All hardcoded colors replaced with `var(--app-*)` references throughout style.css

### Phase 4: Mobile Polish  
- Sidebar becomes slide-in drawer on ≤768px with CSS transform transition
- Mobile header bar with hamburger button
- Overlay backdrop when drawer is open
- 44px minimum touch targets on interactive elements

### Phase 5: CRT Easter Egg
- CSS-only scanline effect on `.scene-block pre` when `data-crt="true"` is set on root
- Includes repeating gradient scanlines, inset box-shadow, text-shadow glow, and flicker animation
- Scene transitions: opacity 0.15s ease on scene pre blocks
