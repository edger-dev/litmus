---
# litmus-52qn
title: Generate and curate first batch of candidate fixtures
status: completed
type: task
priority: normal
created_at: 2026-03-24T14:01:40Z
updated_at: 2026-03-26T14:16:47Z
order: zzs
parent: litmus-49jz
blocked_by:
    - litmus-feex
    - litmus-3lcp
---

Using findings from research + agent generation, create first batch of candidates:

Priority scenarios:
- ripgrep search results (filename, line number, match highlighting)
- bat/cat syntax-highlighted source code
- journalctl/log viewer with severity colors
- neovim/helix editor UI

For each candidate:
- Write setup.sh + command.sh
- Run capture, inspect output
- Evaluate against quality criteria
- Document assessment in REVIEW.md
- Promote good ones, discard or iterate on the rest

Depends on: staging workflow, research

## Plan

1. Create setup.sh + command.sh for each candidate: ripgrep, bat, journalctl, neovim-like
2. Test each locally (FIXTURE_WORK_DIR + command.sh)
3. Parse with litmus-capture parse-ansi
4. Fill out REVIEW.md for each
5. Promote good ones to fixtures/

- [ ] Create ripgrep candidate
- [ ] Create bat candidate
- [ ] Create journalctl candidate
- [ ] Create editor-ui candidate
- [ ] Test and review all
- [ ] Promote passing candidates

## Summary of Changes

Created 4 fixture candidates, all passing quality criteria:

1. **ripgrep-search** — rg search results with heading mode (magenta filenames, green line numbers, bold red match highlighting). 13 lines, uses real rg.
2. **bat-syntax** — Syntax-highlighted Python with bat (extensive color from syntax theme). 24 lines, fills 80x24 exactly.
3. **log-viewer** — Simulated structured app logs (INFO/WARN/ERROR/DEBUG levels). 19 lines, pure ANSI 16-color, excellent color variety.
4. **editor-ui** — Simulated text editor UI (neovim/helix style) with syntax-highlighted Rust, line numbers, and reverse-video status bar. 23 lines, pure ANSI.

All 4 promoted to fixtures/ with output.json, REVIEW.md. Embedded in both litmus-web and litmus-cli. README updated.
