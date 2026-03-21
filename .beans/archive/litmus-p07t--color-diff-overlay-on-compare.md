---
# litmus-p07t
title: Color diff overlay on compare
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:28Z
updated_at: 2026-03-20T18:45:07Z
parent: litmus-gspc
---

Toggle that highlights spans where resolved colors differ between compared themes. Dashed underline on differing spans.

## Summary of Changes

Added ColorDiffTable component to the compare page. Shows a collapsible table of all 19 colors (bg, fg, cursor + 16 ANSI) with swatches and hex values per theme. Rows where colors differ are highlighted with a subtle pink background. Header shows count of differing colors.
