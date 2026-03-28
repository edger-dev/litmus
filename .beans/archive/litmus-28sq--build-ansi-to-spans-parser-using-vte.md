---
# litmus-28sq
title: Build ANSI-to-spans parser using VTE
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:19Z
updated_at: 2026-03-26T14:16:47Z
order: zw
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
- [x] Plain text without escapes
- [x] Basic SGR colors (30-37, 40-47)
- [x] Bright colors (90-97, 100-107)
- [x] 256-color mode (38;5;N, 48;5;N)
- [x] 24-bit truecolor (38;2;R;G;B)
- [x] Bold/italic/dim/underline attributes
- [x] Reset (SGR 0) clears all attributes
- [x] Newline handling
- [x] Span collapsing (adjacent same-attr cells merge)
- [x] Cursor movement (basic)
- [x] Real git diff output parsing

## Summary of Changes

Added `ansi_parser` module to litmus-capture with:

- **Cell grid architecture**: VTE-backed parser with cols×rows grid, cursor tracking, and SGR attribute state
- **Full SGR support**: standard colors (30-47), bright (90-107), 256-color (38;5;N), truecolor (38;2;R;G;B), bold/italic/dim/underline, individual resets (22/23/24)
- **Colon-form subparams**: handles 38:5:N and 38:2::R:G:B forms used by modern terminals
- **Cursor operations**: CUU/CUD/CUF/CUB movement, CUP absolute positioning, EL (erase in line), ED (erase in display)
- **Span collapsing**: merges adjacent cells with identical attributes, trims trailing default spaces
- **`parse-ansi` CLI subcommand**: reads raw ANSI from file/stdin, outputs structured TermOutput JSON
- **37 tests** covering all SGR mappings, edge cases (empty input, zero dimensions, column overflow), and real-world output patterns

Key design decisions:
- Characters beyond terminal width are clipped (no line wrapping)
- Trailing default-attribute spaces are trimmed; styled spaces are preserved
- Zero dimensions clamped to 1×1 to prevent underflow
