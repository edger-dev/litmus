---
# litmus-s6e2
title: Mini scene preview on theme cards
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:00Z
updated_at: 2026-03-20T18:31:25Z
parent: litmus-m8gs
---

Render a compact 4-5 line excerpt of the shell-prompt scene on each ThemeCard so users can feel a theme at a glance.

## Summary of Changes

Added mini scene preview to theme cards:
- New ScenePreview component renders first N lines of a scene with no title
- ThemeCard now shows 5-line shell prompt preview between name and swatches
- CSS: .scene-preview with 0.6rem font, max-height, and gradient fade-out mask
- Also added .scene-compact CSS class for future grid views
