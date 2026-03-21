# Agentic Workflow

litmus is developed with **Claude Code** as the primary coding agent. This chapter documents the tools and patterns that make this workflow effective.

## Overview

The development process pairs a human who defines scope and reviews output with an AI agent that implements, tests, and iterates. Over 13 milestones and 60+ completed beans, this workflow produced the entire codebase — from data model to WASM web app.

## CLAUDE.md

The `CLAUDE.md` file in the project root provides persistent context for the agent across conversations. It contains:

- **Workspace layout**: which crates exist, what each one does
- **Bacon diagnostics workflow**: how to read `.bacon-claude-diagnostics` instead of running `cargo check`
- **Conventions**: anything the agent needs to remember between sessions

CLAUDE.md is loaded automatically at the start of every conversation. Keeping it concise and up-to-date is critical — it's the agent's "working memory" for the project.

## Beans

**Beans** is an agentic-first issue tracker designed for AI agent workflows. Each bean is a Markdown file with YAML frontmatter tracking status, type, priority, and relationships.

Key properties:
- **File-based**: beans live in the repo as `.md` files, readable by both humans and agents
- **CLI-driven**: `beans create`, `beans update`, `beans list` — all with `--json` for machine parsing
- **Structured relationships**: parent/child hierarchies, blocking/blocked-by dependencies
- **Milestone organization**: beans group into milestones (M1, M2, ...) for phased delivery

The agent creates beans before starting work, updates them as tasks complete, and marks them done when finished. This creates a clear audit trail of what was done and why.

## Development loop

A typical milestone follows this cycle:

1. **Human defines scope** — describes what the milestone should achieve, key features, constraints
2. **Agent creates beans** — decomposes the scope into concrete, focused tasks
3. **Agent implements** — works through beans sequentially, committing code and updating bean status
4. **Agent marks done** — checks off todo items within beans, marks beans as completed
5. **Human reviews** — examines the result, provides feedback, identifies follow-up work
6. **Archive** — completed beans are archived with `beans archive` to keep the working set clean

## Bacon diagnostics

The `.bacon-claude-diagnostics` file provides machine-readable compiler feedback. Instead of running `cargo check` (which takes compile time and produces human-formatted output), the agent reads this file to get structured diagnostics:

```
error|:|src/main.rs|:|42|:|42|:|mismatched types|:|<full rendered output>
```

This is faster (bacon is already watching) and parseable (pipe-delimited fields with severity, file, line range, and message).

## What worked well

- **Structured decomposition**: breaking milestones into small, focused beans keeps each task tractable and creates natural commit boundaries
- **CLAUDE.md as persistent instructions**: the agent retains project-specific knowledge across conversations without re-explanation
- **Machine-readable tool output**: `.bacon-claude-diagnostics` and `beans --json` give the agent structured data instead of text to parse
- **File-based everything**: beans, themes, and CLAUDE.md are all files in the repo — no external services, no context lost between sessions

## Lessons learned

- **Keep CLAUDE.md concise**: when it grows too long, the agent spends context window on instructions rather than work. Prune aggressively.
- **Decompose into focused tasks**: a bean that tries to do three things often gets confused. One bean, one concern.
- **Prefer file-based feedback over interactive tools**: the agent works best reading files, not interacting with prompts or TUIs
- **Review early, not just at the end**: catching a wrong direction after one bean is much cheaper than after five
