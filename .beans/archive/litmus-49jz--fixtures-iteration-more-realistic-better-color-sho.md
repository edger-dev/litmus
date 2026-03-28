---
# litmus-49jz
title: Fixtures iteration — more realistic, better color showcase
status: completed
type: epic
priority: normal
created_at: 2026-03-23T15:17:30Z
updated_at: 2026-03-26T14:16:47Z
order: V
---

## Goal

Establish a repeatable workflow to improve fixtures over time. Not a one-shot redesign — a process for ongoing iteration with human curation.

## Quality Criteria

Every fixture must meet:

1. **Color variety** — uses ≥4 distinct ANSI colors naturally (not forced)
2. **Instant recognition** — a developer recognizes the scenario within 2 seconds
3. **Fits 80x24** — no truncation, no scrolling needed
4. **Deterministic** — no timestamps, PIDs, paths that vary between runs (use fixed dates, fake PIDs, $FIXTURE_WORK_DIR)
5. **Self-contained** — setup.sh creates all needed state, no external dependencies beyond the tool itself

## Two Special Fixtures

**Color swatch (raw reference):**
- Small program that prints all 16 ANSI colors as labeled fg+bg blocks, 256-color palette grid, truecolor gradients
- Pure validation artifact, not a real scenario
- `fixtures/color-swatch/`

**Color showcase (themed):**
- Synthetic but real-looking scenario designed to hit every ANSI color naturally
- E.g. a status dashboard: green OK, yellow WARN, red FAIL, blue info, magenta debug, cyan links, bright variants for headers, bg colors for status bars
- `fixtures/color-showcase/`

## Content Sources (priority order)

1. **Existing datasets/collections** — terminal output samples, ANSI test suites, terminal emulator test fixtures
2. **Agent-generated** — Claude writes fixture scripts for specific scenarios, human reviews
3. **Real tool output** — capture from popular dev tools (ripgrep, delta, bat, docker, kubectl)

## Staging & Review Workflow

```
fixtures/
  candidates/           # staging area
    bat-syntax/
      setup.sh
      command.sh
      REVIEW.md         # source, colors used, quality assessment
  git-diff/             # promoted fixtures
  ls-color/
  ...
```

**Flow:**
1. Generate or discover candidate → write to `fixtures/candidates/{name}/`
2. Run capture pipeline → inspect screenshot + parsed output
3. Evaluate against quality criteria (document in REVIEW.md)
4. Accepted → move to `fixtures/{name}/`, delete REVIEW.md
5. Rejected → delete or note why for future reference

## Candidate Scenarios to Research

High-value fixtures that don't exist yet:
- **ripgrep/grep** — search results with filename, line number, match highlighting
- **bat/cat** — syntax-highlighted source code (Rust, Python, YAML)
- **docker ps / kubectl** — tabular colored output with status indicators
- **journalctl / log viewer** — severity-colored log lines (DEBUG, INFO, WARN, ERROR)
- **tmux/zellij status bar** — TUI chrome with background colors
- **neovim/helix** — editor UI (already a scene, no fixture yet)
- **diff (delta)** — enhanced diff with syntax highlighting

## Relationship to Other Epics

Independent of but complementary to litmus-coma (unify scenes/fixtures). New fixtures created here automatically benefit from the unified pipeline once built.

## Subtasks (in dependency order)

**Unblocked (can run in parallel):**
1. `litmus-c55s` — Build color swatch and color showcase fixtures
2. `litmus-feex` — Set up fixtures/candidates/ staging directory and review workflow
3. `litmus-3lcp` — Research existing terminal output datasets and ANSI test suites
4. `litmus-sk2k` — Audit existing fixtures against quality criteria

**Blocked by 2 + 3:**
5. `litmus-52qn` — Generate and curate first batch of candidate fixtures

## Summary of Changes

All 5 subtasks completed. The fixtures system now has 12 fixtures covering common terminal scenarios: git-diff, git-log, ls-color, cargo-build, shell-prompt, python-repl, htop, color-showcase, ripgrep-search, bat-syntax, log-viewer, and editor-ui. A candidates/ staging workflow with quality criteria review process is established. Research identified key external resources (tinted-theming/schemes, Gogh, shell-color-scripts) for future expansion.
