---
# litmus-q9lp
title: Add TermColor, TermSpan, TermLine, TermOutput types to litmus-model
status: todo
type: task
created_at: 2026-03-24T13:47:13Z
updated_at: 2026-03-24T13:47:13Z
parent: litmus-coma
---

Add the new terminal output types to litmus-model:

- TermColor enum: Default, Ansi(u8), Indexed(u8), Rgb(u8, u8, u8)
- TermSpan: text + fg/bg TermColor + bold/italic/dim/underline
- TermLine: Vec<TermSpan>
- TermOutput: id, name, cols, rows, Vec<TermLine>
- Serde JSON serialization/deserialization for all types
- TermColor resolution method: resolve(provider_colors) → CSS-ready rgb values
- Indexed(16-255) → fixed RGB lookup table (standard 256-color palette)

Keep existing Scene/ThemeColor types — removed in a later task.
