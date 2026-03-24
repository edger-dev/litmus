---
# litmus-28sq
title: Build ANSI-to-spans parser using VTE
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:47:19Z
updated_at: 2026-03-24T15:28:14Z
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

## Plan

### Architecture
- New module: `crates/litmus-capture/src/ansi_parser.rs`
- Add `vte` crate dependency for ANSI escape sequence parsing
- Cell grid approach: track cursor position, attributes per cell, then collapse

### Core types
- `CellAttrs`: fg/bg TermColor + bold/italic/dim/underline
- `Cell`: character + CellAttrs
- `Grid`: rows×cols of Cells, cursor position tracking
- `AnsiParser`: wraps Grid, implements VTE Perform trait

### SGR mapping
- 30-37 → Ansi(0-7) fg, 40-47 → Ansi(0-7) bg
- 90-97 → Ansi(8-15) fg, 100-107 → Ansi(8-15) bg
- 38;5;N → indexed fg, 48;5;N → indexed bg
- 38;2;R;G;B → RGB fg, 48;2;R;G;B → RGB bg
- 39 → Default fg, 49 → Default bg
- 0 → reset, 1 → bold, 2 → dim, 3 → italic, 4 → underline

### Public API
```text
pub fn parse_ansi(input: &[u8], cols: u16, rows: u16) -> TermOutput
```

### Tests (TDD)
- [ ] Plain text without escapes
- [ ] Basic SGR colors (30-37, 40-47)
- [ ] Bright colors (90-97, 100-107)
- [ ] 256-color mode (38;5;N, 48;5;N)
- [ ] 24-bit truecolor (38;2;R;G;B)
- [ ] Bold/italic/dim/underline attributes
- [ ] Reset (SGR 0) clears all attributes
- [ ] Newline handling
- [ ] Span collapsing (adjacent same-attr cells merge)
- [ ] Cursor movement (basic)
- [ ] Real git diff output parsing
