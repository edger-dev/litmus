---
# litmus-28sq
title: Build ANSI-to-spans parser using VTE
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:47:19Z
updated_at: 2026-03-24T13:47:43Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
---

Add ANSI output parsing to litmus-capture (or a new litmus-parse crate):

- Use vte or termwiz crate to process raw ANSI byte streams
- Intermediate cell grid to handle cursor movement and overwrites
- Collapse adjacent cells with identical attributes into TermSpans
- Input: raw bytes from fixture command output
- Output: TermOutput struct
- Handle: SGR (colors, bold, italic, dim, underline), newlines, basic cursor movement
- Map SGR color codes to TermColor variants:
  - \e[30-37m / \e[90-97m → Ansi(0-15)
  - \e[38;5;Nm → Ansi/Indexed depending on N
  - \e[38;2;R;G;Bm → Rgb(r,g,b)
  - \e[39m / no color → Default

Depends on: TermOutput types in litmus-model
