# Litmus Fixtures

Fixtures are reproducible terminal scenarios used for capturing real screenshots of themes.
Each fixture lives in its own subdirectory and follows a standard script interface.

## Directory Structure

```
fixtures/
  {fixture-id}/
    setup.sh      # (required) Creates files/state in $FIXTURE_WORK_DIR
    command.sh    # (required) Runs the actual command; CWD is set to $FIXTURE_WORK_DIR
    teardown.sh   # (optional) Cleanup after screenshot is taken
    README.md     # (optional) Description of what this fixture demonstrates
```

## Script Interface

The capture tool calls scripts with `FIXTURE_WORK_DIR` set to an existing temp directory:

```
FIXTURE_WORK_DIR=/tmp/litmus-capture-XXXXXX
```

**setup.sh**
- Creates files, git repos, projects, etc. inside `$FIXTURE_WORK_DIR`
- Must be idempotent (called once per capture)
- Exit 0 on success, non-zero on failure

**command.sh**
- The capture tool changes into `$FIXTURE_WORK_DIR` before running this
- Produces the terminal output that will be screenshotted
- Should exit when done (terminal emulator is kept open by the capture tool via `--hold`)
- Colors must use ANSI/terminal escape codes (not hard-coded RGB)

**teardown.sh** (optional)
- Runs after screenshot is taken
- Cleans up any resources not in `$FIXTURE_WORK_DIR` (the temp dir is cleaned up automatically)

## Writing Good Fixtures

1. **Pin versions/revisions**: Use `git init -b main`, fixed author names, fixed timestamps where possible.
   ```bash
   GIT_AUTHOR_DATE="2024-01-15T10:00:00" GIT_COMMITTER_DATE="2024-01-15T10:00:00" git commit ...
   ```

2. **Exercise all 16 ANSI colors**: Good fixtures use the full color range so themes can be compared meaningfully.

3. **Keep output within 80×24**: The capture terminal is fixed at 80 columns × 24 rows. Output that wraps or scrolls will be cut off.

4. **Test your fixture locally**:
   ```bash
   export FIXTURE_WORK_DIR=$(mktemp -d)
   bash fixtures/{id}/setup.sh
   cd $FIXTURE_WORK_DIR && bash /path/to/fixtures/{id}/command.sh
   rm -rf $FIXTURE_WORK_DIR
   ```

## Fixture Inventory

| ID | Command | What it demonstrates |
|----|---------|---------------------|
| `git-diff` | `git diff` | Diff colors: additions, deletions, context |
| `ls-color` | `ls -la --color` | File type colors: dirs, executables, symlinks |
| `cargo-build` | `cargo build` | Compiler output: success, warnings, errors |
| `shell-prompt` | bash session | Prompt colors and common command output |
| `git-log` | `git log --graph` | Log graph colors and branch decorations |
| `python-repl` | `python3` session | REPL output, tracebacks, colored values |
| `htop` | `top -b -n 1` | System monitor colors: CPU, memory, process table |
