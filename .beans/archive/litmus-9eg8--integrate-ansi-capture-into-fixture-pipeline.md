---
# litmus-9eg8
title: Integrate ANSI capture into fixture pipeline
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:24Z
updated_at: 2026-03-26T14:16:47Z
order: zzw
parent: litmus-coma
blocked_by:
    - litmus-28sq
---

Update the capture pipeline to also capture and parse ANSI output:

- When running fixture command.sh through a provider terminal, tee the ANSI byte stream (via PTY or script command)
- Parse the captured stream into TermOutput using the ANSI parser
- Write output.{provider}.json alongside the fixture scripts
- Add CLI flags: --parse-only (skip screenshot, just parse), --fixture (filter)
- Generated files are checked into git so litmus-web can embed them

Depends on: ANSI parser

## Plan

- [x] Add ParseFixtures subcommand with --fixture, --force, --cols, --rows flags
- [x] Implement run_fixture_and_parse: runs setup.sh + command.sh, captures stdout, parses ANSI
- [x] Write output.json alongside fixture scripts
- [x] Generate output.json for all 9 fixtures
- [x] Verify zero warnings, all tests pass

## Summary of Changes

Added parse-fixtures subcommand to litmus-capture that runs each fixture's setup.sh + command.sh, captures raw ANSI stdout bytes, parses them into structured TermOutput JSON using the VTE-based parser, and writes output.json alongside the fixture scripts.

The output files are provider-independent (raw terminal output is the same regardless of which terminal renders it) and are checked into git so litmus-web can embed them for theme-independent scene rendering.
