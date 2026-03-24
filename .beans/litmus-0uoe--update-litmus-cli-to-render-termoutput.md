---
# litmus-0uoe
title: Update litmus-cli to render TermOutput
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:47:32Z
updated_at: 2026-03-24T13:47:44Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-9eg8
---

Migrate litmus-cli mockup views from Scene to TermOutput:

- Load TermOutput data (bundled or from fixtures directory)
- Render TermSpan using crossterm colors:
  - TermColor::Default → crossterm reset
  - TermColor::Ansi(n) → crossterm AnsiValue
  - TermColor::Indexed(n) → crossterm AnsiValue
  - TermColor::Rgb → crossterm Rgb
- Minimal changes — CLI is simpler than web

Depends on: TermOutput types, fixture pipeline generating output files
