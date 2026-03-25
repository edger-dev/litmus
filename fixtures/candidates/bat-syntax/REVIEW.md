# Review: bat-syntax

**Description**: Syntax-highlighted Python source code with line numbers and grid
**Command**: `bat --color=always --style=numbers,grid --line-range=1:22 server.py`

## Quality Criteria

- [x] Color variety (≥4 distinct ANSI colors) — extensive syntax highlighting (keywords, strings, comments, decorators, types)
- [x] Instant recognition — immediately recognizable as syntax-highlighted source code
- [x] Fits 80x24 — 24 lines (exactly fills the grid)
- [x] Deterministic — highlights fixed source file
- [x] Self-contained — only needs bat (widely available)

## Color Coverage

- fg: green (strings), blue (keywords), cyan (decorators), magenta (types), yellow (numbers), red (operators), white (identifiers), dim (line numbers, grid)
- bg: none (default only)
- 256/truecolor: bat may use 256-color or truecolor depending on theme — uses its own syntax theme

## Notes

- Very color-rich output with bat's syntax highlighting
- bat uses its own color themes (not just ANSI 16), so colors may be 256-color or truecolor
- This means the fixture may not vary much between terminal color themes (bat picks its own colors)
- However, it still demonstrates how the theme's background color affects readability of syntax highlighting

## Decision

- [x] Promote to fixtures/
- [ ] Needs changes
- [ ] Discard
