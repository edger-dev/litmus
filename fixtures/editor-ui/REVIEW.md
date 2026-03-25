# Review: editor-ui

**Description**: Simulated text editor UI with syntax-highlighted Rust code, line numbers, and status bar
**Command**: Direct ANSI output (no external tool dependency)

## Quality Criteria

- [x] Color variety (≥4 distinct ANSI colors) — magenta (keywords), blue (function names), green (types), yellow (strings), cyan (comments), dim (line numbers)
- [x] Instant recognition — immediately recognizable as a code editor (neovim/helix style)
- [x] Fits 80x24 — 23 lines
- [x] Deterministic — hardcoded output
- [x] Self-contained — pure bash, no external dependencies

## Color Coverage

- fg: magenta (keywords: use, pub fn, let, match), bold blue (function names), green (types), yellow (strings), cyan (doc comments), bright black/dim (line numbers), white (identifiers), dim (command area)
- bg: reverse video (status bar)
- 256/truecolor: none — pure ANSI 16

## Notes

- Uses reverse video for the status bar — tests how themes handle attribute combinations
- Shows a realistic editor layout: line numbers + code + status bar + command area
- Good complement to bat-syntax — this one simulates the full editor chrome
- Pure ANSI output means colors are fully theme-dependent

## Decision

- [x] Promote to fixtures/
- [ ] Needs changes
- [ ] Discard
