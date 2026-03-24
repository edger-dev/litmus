---
# litmus-bcel
title: Update contrast validation for TermColor
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:47:35Z
updated_at: 2026-03-24T13:47:44Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-lm76
---

Update contrast validation to work with TermOutput instead of Scene:

- TermColor::Default and Ansi(0-15) pairs: validate against theme palette (same as before)
- TermColor::Indexed/Rgb vs Default/Ansi: validate fixed color against theme-dependent counterpart
- TermColor::Indexed/Rgb vs Indexed/Rgb: skip (both fixed, theme-independent)
- New insight: can flag "this fixture uses hardcoded colors that clash with this theme's background"
- Update ReadabilityIssue to reference TermSpan positions

Depends on: TermOutput types, web rendering migration
