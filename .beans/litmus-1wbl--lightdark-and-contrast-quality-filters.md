---
# litmus-1wbl
title: Light/dark and contrast quality filters
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:00Z
updated_at: 2026-03-20T18:32:34Z
parent: litmus-m8gs
---

Add toggle filters above theme grid: light/dark mode filter and 'good contrast only' toggle that hides themes with WCAG AA failures.

## Summary of Changes

Added filter controls to theme listing:
- VariantFilter enum with All/Dark/Light modes using relative_luminance check
- Good contrast toggle using validate_theme_readability
- FilterButton component with active/inactive styling
- Filtered theme count indicator
- Empty state when no themes match
- CSS: .filter-bar and .filter-btn styles
