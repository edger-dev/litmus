# Adding Scenes

Scenes are simulated terminal outputs that showcase how a theme looks in real-world contexts (git diff, shell prompt, code editors, etc.). This guide covers adding new scenes.

## How scenes work

A scene is built from three primitives defined in `crates/litmus-model/src/scene.rs`:

- **`ThemeColor`** — a semantic color reference (e.g. `Ansi(1)` for red, `Foreground`, `Background`). Scenes never contain hardcoded RGB values; they reference theme slots that get resolved at render time.
- **`StyledSpan`** — a chunk of text with optional foreground color, background color, and text style (bold, italic, dim, underline).
- **`SceneLine`** — a row of styled spans.
- **`Scene`** — an id, name, description, and a list of scene lines.

## 1. Add a scene function in `scenes.rs`

Open `crates/litmus-model/src/scenes.rs` and add a function that returns a `Scene`. Here's a minimal example:

```rust
fn my_tool_scene() -> Scene {
    use ThemeColor::*;

    let lines = vec![
        SceneLine::new(vec![
            StyledSpan::colored("$ ", Ansi(2)).bold(),
            StyledSpan::colored("my-tool", Ansi(4)).bold(),
            StyledSpan::colored(" --flag", Ansi(6)),
        ]),
        SceneLine::new(vec![
            StyledSpan::colored("OK", Ansi(2)),
            StyledSpan::plain(": operation complete"),
        ]),
        SceneLine::new(vec![
            StyledSpan::colored("WARN", Ansi(3)),
            StyledSpan::plain(": 2 items skipped"),
        ]),
    ];

    Scene {
        id: "my-tool".into(),
        name: "My Tool".into(),
        description: "My tool output showing status messages".into(),
        lines,
    }
}
```

### Builder API reference

```rust
// Plain text (uses theme foreground)
StyledSpan::plain("hello")

// Colored text (ANSI 0-15)
StyledSpan::colored("error", Ansi(1))       // red
StyledSpan::colored("success", Ansi(2))     // green
StyledSpan::colored("warning", Ansi(3))     // yellow
StyledSpan::colored("info", Ansi(4))        // blue

// Special theme colors
StyledSpan::colored("text", Foreground)
StyledSpan::colored("text", Cursor)

// Style modifiers (chainable)
StyledSpan::colored("bold red", Ansi(1)).bold()
StyledSpan::plain("subtle").dim()
StyledSpan::plain("emphasis").italic()

// Background color
StyledSpan::colored("selected", Ansi(0)).on(SelectionBackground)
```

The key rule: **only use `ThemeColor` references, never hardcoded RGB**. This ensures every scene adapts to every theme.

### ANSI color mapping

| Index | Color | Bright variant |
|-------|-------|---------------|
| 0 | Black | 8 — Bright Black |
| 1 | Red | 9 — Bright Red |
| 2 | Green | 10 — Bright Green |
| 3 | Yellow | 11 — Bright Yellow |
| 4 | Blue | 12 — Bright Blue |
| 5 | Magenta | 13 — Bright Magenta |
| 6 | Cyan | 14 — Bright Cyan |
| 7 | White | 15 — Bright White |

## 2. Register in `all_scenes()`

Add your scene function to the `all_scenes()` vector in the same file:

```rust
pub fn all_scenes() -> Vec<Scene> {
    vec![
        shell_prompt_scene(),
        git_diff_scene(),
        // ... existing scenes ...
        my_tool_scene(),  // add here
    ]
}
```

## 3. Verify

No web-side changes are needed — scenes automatically appear in all views (theme detail tabs, scene-across-themes grid, compare mode).

1. Run `cargo check` (or read `.bacon-claude-diagnostics`)
2. Start the web app with `mise run dev`
3. Verify your scene appears in the scene tabs on any theme detail page
4. Check it renders well across a few different themes (light and dark)
