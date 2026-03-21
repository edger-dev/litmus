---
# litmus-iqfq
title: Update mdbook documentation to reflect current project state (M6-M13)
status: completed
type: task
priority: normal
created_at: 2026-03-21T03:40:33Z
updated_at: 2026-03-21T03:43:24Z
---

Rewrite docs: update intro/milestones, add architecture/development/agentic-workflow pages, delete contributing.md placeholder

## Summary of Changes

- **introduction.md**: Added current scale (29 themes, 15 families, 8 scenes, 3 crates), project status section
- **architecture.md**: New — workspace layout, data model, scene system, web app, accessibility/CVD
- **development.md**: New — replaces contributing.md, covers prerequisites, quick start, mise tasks, dev loop
- **development/adding-themes.md**: New — step-by-step guide with TOML format, themes.rs registration, family setup
- **development/adding-scenes.md**: New — guide covering ThemeColor, StyledSpan API, scene registration
- **milestones.md**: Rewritten — M0-M13 in past tense, removed Post-MVP section
- **agentic-workflow.md**: New — documents Claude Code + beans development process
- **SUMMARY.md**: Updated with new book structure
- **contributing.md**: Deleted (replaced by development.md)
- mdbook builds cleanly with no broken links
