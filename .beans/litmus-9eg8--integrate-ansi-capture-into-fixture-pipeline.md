---
# litmus-9eg8
title: Integrate ANSI capture into fixture pipeline
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:47:24Z
updated_at: 2026-03-24T13:47:43Z
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
