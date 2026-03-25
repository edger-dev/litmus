---
# litmus-4uyp
title: Show all themes with availability feedback
status: completed
type: feature
priority: normal
created_at: 2026-03-25T15:05:23Z
updated_at: 2026-03-25T15:57:48Z
---

Theme list only shows themes for the active provider — switching providers refreshes the list jarringly. Show all themes instead with dimmed cards + badge for unavailable ones. ## Design: Dimmed cards with unavailable badge

- Replace themes_for_provider() with a function that returns **all themes**, each annotated with an available: bool flag for the current provider.
- **Available themes:** Render normally, clickable, navigate to detail page.
- **Unavailable themes:** Reduced opacity (0.4–0.5), small "unavailable" badge or icon overlay, cursor: not-allowed, click suppressed (no navigation).
- Alphabetical sort preserved — unavailable themes stay in their natural position, not pushed to the bottom.
- Filter counts (variant toggles, total) reflect **only available themes** for the active provider.
- Search/variant/readability filters still apply to all themes (available and unavailable).

**Files:** themes.rs (new all_themes_with_availability()), theme_list.rs (card rendering + click gating), state.rs (filter counts), style.css (dimmed card styles)

## Tasks
- [x] Add all_themes_with_availability() returning (Theme, bool) tuples
- [x] Update theme_list.rs to render unavailable cards with reduced opacity
- [x] Add 'unavailable' badge/icon overlay on dimmed cards
- [x] Suppress click/navigation for unavailable themes (cursor: not-allowed)
- [x] Keep alphabetical sort with unavailable themes in natural position
- [x] Filter counts reflect only available themes
- [x] Add CSS for dimmed card state (.theme-card--unavailable or similar)
- [x] Test that search/variant/readability filters apply to all themes

## Plan

### themes.rs
Add `all_themes_with_availability(provider: &str) -> Vec<(Theme, bool)>`:
- For each definition, check if provider colors exist for the requested provider → `available = true`
- If unavailable, use first available provider's colors for rendering → `available = false`
- Every definition has at least one provider (existing test guarantees this), so all are renderable
- Sort alphabetically by name

### theme_list.rs
- Replace `themes_for_provider()` with `all_themes_with_availability()`
- Pass `available: bool` to ThemeCard component
- Filter counts (variant badges, total) count only available themes
- Shown count / filter applies to all themes (available + unavailable)

### ThemeCard component
- Accept `available: bool` prop
- If unavailable: wrap in div instead of Link, add `.theme-card--unavailable` class, show badge
- If available: keep existing Link behavior

### style.css
- `.theme-card--unavailable`: opacity 0.45, cursor not-allowed, pointer-events none on card-link
- `.theme-card-unavailable-badge`: small label overlay

## Summary of Changes

Added `all_themes_with_availability()` to themes.rs that returns every theme annotated with a bool for provider availability. Unavailable themes fall back to the first available provider's colors for rendering.

Updated ThemeList to show all themes — unavailable cards are dimmed (opacity 0.45), have an "unavailable" badge, no hover effect, and no navigation. Extracted ThemeCardBody as a shared component to avoid RSX duplication. Filter counts (variant badges) reflect available themes only; shown/total counter reflects all visible cards.

5 new tests cover the availability function (count, sorting, marking, unavailable presence, nonexistent provider).
