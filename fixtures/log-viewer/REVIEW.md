# Review: log-viewer

**Description**: Simulated application logs with severity levels (INFO, WARN, ERROR, DEBUG)
**Command**: Direct ANSI output (no external tool dependency)

## Quality Criteria

- [x] Color variety (≥4 distinct ANSI colors) — green (INFO/success), yellow (WARN), red (ERROR), blue (DEBUG), cyan (module names), magenta (build numbers), dim (timestamps)
- [x] Instant recognition — immediately recognizable as structured log output
- [x] Fits 80x24 — 19 lines
- [x] Deterministic — hardcoded output
- [x] Self-contained — pure bash, no external dependencies

## Color Coverage

- fg: green (INFO, success values), bold green (INFO label), yellow (WARN, capacity numbers), bold yellow (WARN label), red (ERROR, error values), bold red (ERROR label), blue (URLs/paths), bold blue (DEBUG label), cyan (module names, timing), bold cyan (durations), magenta (build numbers), bold magenta (branch), white (HTTP methods), dim (timestamps, stack traces)
- bg: none
- 256/truecolor: none — pure ANSI 16

## Notes

- Exercises nearly all 16 ANSI colors — excellent for theme comparison
- Realistic log format with timestamps, severity, module, and message structure
- Includes error with stack trace, warnings, and successful operations
- Pure ANSI output means colors are fully theme-dependent — ideal for litmus
- Could replace or complement the existing color-showcase fixture

## Decision

- [x] Promote to fixtures/
- [ ] Needs changes
- [ ] Discard
