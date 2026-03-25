# Review: ripgrep-search

**Description**: ripgrep search results across multiple files with heading mode
**Command**: `rg --color=always --heading --line-number "Config" src/`

## Quality Criteria

- [x] Color variety (≥4 distinct ANSI colors) — magenta (filenames), green (line numbers), bold red (match highlight), default (context)
- [x] Instant recognition — immediately recognizable as ripgrep/grep search output
- [x] Fits 80x24 — 13 lines
- [x] Deterministic — searches fixed source files
- [x] Self-contained — only needs rg (widely available)

## Color Coverage

- fg: magenta (filenames), green (line numbers), bold+red (match), default (context text)
- bg: none (default only)
- 256/truecolor: none

## Notes

- Good complement to existing git-diff and git-log fixtures
- Shows a common developer workflow (searching code)
- 13 lines is compact, could add more source files to fill more of the 80x24 grid
- rg is not in POSIX but is very common in developer environments

## Decision

- [x] Promote to fixtures/
- [ ] Needs changes
- [ ] Discard
