//! Built-in terminal scenes that exercise theme colors.

use crate::scene::{Scene, SceneLine, StyledSpan, ThemeColor};

/// Shorthand: ANSI color reference.
fn ansi(i: u8) -> ThemeColor {
    ThemeColor::Ansi(i)
}

/// All built-in scenes.
pub fn all_scenes() -> Vec<Scene> {
    vec![
        shell_prompt_scene(),
        git_diff_scene(),
        ls_color_scene(),
        cargo_build_scene(),
        log_viewer_scene(),
        neovim_scene(),
        python_repl_scene(),
        htop_scene(),
    ]
}

/// Shell prompt with common segments.
pub fn shell_prompt_scene() -> Scene {
    Scene {
        id: "shell-prompt".into(),
        name: "Shell Prompt".into(),
        description: "Common shell prompt styles with command output".into(),
        lines: vec![
            // Prompt line
            SceneLine::new(vec![
                StyledSpan::colored("user@host", ansi(10)).bold(),
                StyledSpan::plain(":"),
                StyledSpan::colored("~/projects/myapp", ansi(12)).bold(),
                StyledSpan::plain(" "),
                StyledSpan::colored("(main)", ansi(13)),
                StyledSpan::plain(" $ "),
                StyledSpan::plain("echo \"Hello, world!\""),
            ]),
            SceneLine::new(vec![StyledSpan::plain("Hello, world!")]),
            SceneLine::empty(),
            // Second prompt with exit code
            SceneLine::new(vec![
                StyledSpan::colored("user@host", ansi(10)).bold(),
                StyledSpan::plain(":"),
                StyledSpan::colored("~/projects/myapp", ansi(12)).bold(),
                StyledSpan::plain(" "),
                StyledSpan::colored("(main)", ansi(13)),
                StyledSpan::plain(" $ "),
                StyledSpan::plain("cat nonexistent.txt"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("cat: nonexistent.txt: No such file or directory", ansi(1)),
            ]),
            SceneLine::empty(),
            // Third prompt with piped commands
            SceneLine::new(vec![
                StyledSpan::colored("user@host", ansi(10)).bold(),
                StyledSpan::plain(":"),
                StyledSpan::colored("~/projects/myapp", ansi(12)).bold(),
                StyledSpan::plain(" "),
                StyledSpan::colored("(main*)", ansi(11)),
                StyledSpan::plain(" $ "),
                StyledSpan::plain("grep -rn TODO src/ | head -5"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("src/main.rs", ansi(13)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::colored("42", ansi(2)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::plain("    // "),
                StyledSpan::colored("TODO", ansi(11)).bold(),
                StyledSpan::plain(": refactor this function"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("src/lib.rs", ansi(13)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::colored("108", ansi(2)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::plain("    // "),
                StyledSpan::colored("TODO", ansi(11)).bold(),
                StyledSpan::plain(": add error handling"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("src/config.rs", ansi(13)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::colored("7", ansi(2)),
                StyledSpan::colored(":", ansi(6)),
                StyledSpan::plain("    // "),
                StyledSpan::colored("TODO", ansi(11)).bold(),
                StyledSpan::plain(": load from env"),
            ]),
        ],
    }
}

/// Git diff output with context, additions, and deletions.
pub fn git_diff_scene() -> Scene {
    Scene {
        id: "git-diff".into(),
        name: "Git Diff".into(),
        description: "Git diff with context lines, additions, deletions, and hunk headers".into(),
        lines: vec![
            SceneLine::new(vec![
                StyledSpan::colored("diff --git a/src/main.rs b/src/main.rs", ThemeColor::Foreground).bold(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("index 3f4a1b2..8c9d0e1 100644", ThemeColor::Foreground).bold(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("--- a/src/main.rs", ansi(1)).bold(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+++ b/src/main.rs", ansi(2)).bold(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("@@ -12,8 +12,11 @@ fn main() -> Result<()> {", ansi(6)),
            ]),
            // Context
            SceneLine::new(vec![StyledSpan::plain("     let config = Config::load()?;")]),
            SceneLine::new(vec![StyledSpan::plain("     let mut app = App::new(config);")]),
            // Deletions
            SceneLine::new(vec![
                StyledSpan::colored("-    app.run()?;", ansi(1)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("-    Ok(())", ansi(1)),
            ]),
            // Additions
            SceneLine::new(vec![
                StyledSpan::colored("+    let theme = Theme::load(&config.theme_path)?;", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+    app.set_theme(theme);", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+    app.run()?;", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+    Ok(())", ansi(2)),
            ]),
            // Context
            SceneLine::new(vec![StyledSpan::plain(" }")]),
            SceneLine::empty(),
            // Second hunk
            SceneLine::new(vec![
                StyledSpan::colored("@@ -45,3 +48,9 @@ impl App {", ansi(6)),
            ]),
            SceneLine::new(vec![StyledSpan::plain("     fn render(&self) {")]),
            SceneLine::new(vec![
                StyledSpan::colored("-        todo!()", ansi(1)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+        for line in &self.scene.lines {", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+            for span in &line.spans {", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+                self.draw_span(span);", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+            }", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("+        }", ansi(2)),
            ]),
            SceneLine::new(vec![StyledSpan::plain("     }")]),
        ],
    }
}

/// Colorized ls -la output.
pub fn ls_color_scene() -> Scene {
    Scene {
        id: "ls-color".into(),
        name: "Directory Listing".into(),
        description: "Colorized ls -la showing directories, files, executables, and symlinks".into(),
        lines: vec![
            // Prompt
            SceneLine::new(vec![
                StyledSpan::colored("user@host", ansi(10)).bold(),
                StyledSpan::plain(":"),
                StyledSpan::colored("~/projects/myapp", ansi(12)).bold(),
                StyledSpan::plain(" $ "),
                StyledSpan::plain("ls -la"),
            ]),
            SceneLine::new(vec![StyledSpan::plain("total 48")]),
            // Directories
            SceneLine::new(vec![
                StyledSpan::plain("drwxr-xr-x  8 user user  4096 Mar 20 09:15 "),
                StyledSpan::colored(".", ansi(12)).bold(),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("drwxr-xr-x 12 user user  4096 Mar 19 14:22 "),
                StyledSpan::colored("..", ansi(12)).bold(),
            ]),
            // Hidden dir
            SceneLine::new(vec![
                StyledSpan::plain("drwxr-xr-x  9 user user  4096 Mar 20 09:10 "),
                StyledSpan::colored(".git", ansi(12)).bold(),
            ]),
            // Hidden file (dimmed)
            SceneLine::new(vec![
                StyledSpan::plain("-rw-r--r--  1 user user   284 Mar 20 08:40 "),
                StyledSpan::colored(".gitignore", ansi(8)),
            ]),
            // Regular files
            SceneLine::new(vec![
                StyledSpan::plain("-rw-r--r--  1 user user  1024 Mar 20 09:10 "),
                StyledSpan::plain("Cargo.toml"),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("-rw-r--r--  1 user user   512 Mar 20 09:10 "),
                StyledSpan::plain("Cargo.lock"),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("-rw-r--r--  1 user user   198 Mar 18 11:00 "),
                StyledSpan::plain("README.md"),
            ]),
            // Source directory
            SceneLine::new(vec![
                StyledSpan::plain("drwxr-xr-x  3 user user  4096 Mar 20 09:15 "),
                StyledSpan::colored("src", ansi(12)).bold(),
            ]),
            // Executable
            SceneLine::new(vec![
                StyledSpan::plain("-rwxr-xr-x  1 user user  8192 Mar 20 09:15 "),
                StyledSpan::colored("target/debug/myapp", ansi(10)).bold(),
            ]),
            // Symlink
            SceneLine::new(vec![
                StyledSpan::plain("lrwxrwxrwx  1 user user    12 Mar 18 11:00 "),
                StyledSpan::colored("latest", ansi(14)),
                StyledSpan::plain(" -> "),
                StyledSpan::colored("target/debug/myapp", ansi(10)),
            ]),
        ],
    }
}

/// Cargo build output with warnings and errors.
pub fn cargo_build_scene() -> Scene {
    Scene {
        id: "cargo-build".into(),
        name: "Cargo Build".into(),
        description: "Compiler output with warnings, errors, and notes".into(),
        lines: vec![
            SceneLine::new(vec![
                StyledSpan::colored("   Compiling", ansi(2)).bold(),
                StyledSpan::plain(" litmus-model v0.1.0"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   Compiling", ansi(2)).bold(),
                StyledSpan::plain(" litmus-cli v0.1.0"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("warning", ansi(11)).bold(),
                StyledSpan::colored("[unused_imports]", ansi(11)),
                StyledSpan::plain(": unused import: `std::io::Write`"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(" --> ", ansi(6)),
                StyledSpan::plain("src/main.rs:3:5"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  |", ansi(6)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("3 |", ansi(6)),
                StyledSpan::plain("     use "),
                StyledSpan::colored("std::io::Write", ansi(11)),
                StyledSpan::plain(";"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  |", ansi(6)),
                StyledSpan::plain("         "),
                StyledSpan::colored("^^^^^^^^^^^^^^", ansi(11)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  = ", ansi(6)),
                StyledSpan::colored("note", ThemeColor::Foreground).bold(),
                StyledSpan::plain(": `#[warn(unused_imports)]` on by default"),
            ]),
            SceneLine::empty(),
            SceneLine::new(vec![
                StyledSpan::colored("error[E0308]", ansi(1)).bold(),
                StyledSpan::plain(": mismatched types"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  --> ", ansi(6)),
                StyledSpan::plain("src/render.rs:47:12"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   |", ansi(6)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("45 |", ansi(6)),
                StyledSpan::plain("     fn color(&self) -> Color {"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   |", ansi(6)),
                StyledSpan::plain("                        "),
                StyledSpan::colored("-----", ansi(6)),
                StyledSpan::plain(" expected `Color` because of return type"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("47 |", ansi(6)),
                StyledSpan::plain("         "),
                StyledSpan::colored("\"red\"", ansi(1)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   |", ansi(6)),
                StyledSpan::plain("         "),
                StyledSpan::colored("^^^^^", ansi(1)),
                StyledSpan::plain(" expected `Color`, found `&str`"),
            ]),
            SceneLine::empty(),
            SceneLine::new(vec![
                StyledSpan::colored("   help", ansi(6)).bold(),
                StyledSpan::plain(": try using `Color::from_hex`:"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   |", ansi(6)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("47 |", ansi(6)),
                StyledSpan::plain("         "),
                StyledSpan::colored("Color::from_hex(\"red\")", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("   |", ansi(6)),
                StyledSpan::plain("         "),
                StyledSpan::colored("+++++++++++++++      +", ansi(2)),
            ]),
            SceneLine::empty(),
            SceneLine::new(vec![
                StyledSpan::colored("error", ansi(1)).bold(),
                StyledSpan::plain(": could not compile `litmus-cli` (bin \"litmus\") due to 1 previous error; 1 warning emitted"),
            ]),
        ],
    }
}

/// Git log / tig-style view with branch graph.
pub fn log_viewer_scene() -> Scene {
    Scene {
        id: "log-viewer".into(),
        name: "Git Log".into(),
        description: "Git log with branch graph, commit hashes, and decorations".into(),
        lines: vec![
            SceneLine::new(vec![
                StyledSpan::colored("*", ansi(11)),
                StyledSpan::plain(" "),
                StyledSpan::colored("a1b2c3d", ansi(3)),
                StyledSpan::plain(" "),
                StyledSpan::colored("(", ThemeColor::Foreground),
                StyledSpan::colored("HEAD -> ", ansi(6)).bold(),
                StyledSpan::colored("main", ansi(2)).bold(),
                StyledSpan::colored(", ", ThemeColor::Foreground),
                StyledSpan::colored("origin/main", ansi(1)).bold(),
                StyledSpan::colored(")", ThemeColor::Foreground),
                StyledSpan::plain(" Add scene format with semantic color refs"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("*", ansi(11)),
                StyledSpan::plain(" "),
                StyledSpan::colored("e4f5678", ansi(3)),
                StyledSpan::plain(" Add curated theme library with 19 themes"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("*", ansi(11)),
                StyledSpan::plain(" "),
                StyledSpan::colored("9a0b1c2", ansi(3)),
                StyledSpan::plain(" Add kitty.conf theme parser and CLI file loading"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("|\\", ansi(11)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("| ", ansi(11)),
                StyledSpan::colored("*", ansi(13)),
                StyledSpan::plain(" "),
                StyledSpan::colored("d3e4f56", ansi(3)),
                StyledSpan::plain(" "),
                StyledSpan::colored("(", ThemeColor::Foreground),
                StyledSpan::colored("feature/web-preview", ansi(2)).bold(),
                StyledSpan::colored(")", ThemeColor::Foreground),
                StyledSpan::plain(" WIP: web renderer scaffold"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("| ", ansi(11)),
                StyledSpan::colored("*", ansi(13)),
                StyledSpan::plain(" "),
                StyledSpan::colored("7890abc", ansi(3)),
                StyledSpan::plain(" Add Dioxus project skeleton"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("|/", ansi(11)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("*", ansi(11)),
                StyledSpan::plain(" "),
                StyledSpan::colored("def1234", ansi(3)),
                StyledSpan::plain(" Add theme navigation and status bar"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("*", ansi(11)),
                StyledSpan::plain(" "),
                StyledSpan::colored("5678901", ansi(3)),
                StyledSpan::plain(" Initial commit: model + CLI skeleton"),
            ]),
        ],
    }
}

/// Neovim / code editor scene with syntax highlighting.
pub fn neovim_scene() -> Scene {
    Scene {
        id: "neovim".into(),
        name: "Neovim / Code".into(),
        description: "Code editor with syntax highlighting, line numbers, and LSP diagnostics".into(),
        lines: vec![
            // Status line (top)
            SceneLine::new(vec![
                StyledSpan::colored("  NORMAL ", ansi(0)).bold().on(ansi(2)),
                StyledSpan::colored(" main.rs ", ansi(15)).on(ansi(8)),
                StyledSpan::colored(" [+] ", ansi(11)).on(ansi(8)),
                StyledSpan::colored("                              ", ThemeColor::Foreground).on(ansi(8)),
                StyledSpan::colored(" utf-8  rust  42:10 ", ansi(15)).on(ansi(8)),
            ]),
            // Code with line numbers
            SceneLine::new(vec![
                StyledSpan::colored("  38 ", ansi(8)).dim(),
                StyledSpan::colored("use ", ansi(5)),
                StyledSpan::colored("std::collections::HashMap", ThemeColor::Foreground),
                StyledSpan::plain(";"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  39 ", ansi(8)).dim(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  40 ", ansi(8)).dim(),
                StyledSpan::colored("/// ", ansi(8)).italic(),
                StyledSpan::colored("Process incoming data and return results.", ansi(8)).italic(),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  41 ", ansi(8)).dim(),
                StyledSpan::colored("pub ", ansi(5)),
                StyledSpan::colored("fn ", ansi(12)),
                StyledSpan::colored("process", ansi(3)),
                StyledSpan::plain("("),
                StyledSpan::colored("data", ThemeColor::Foreground),
                StyledSpan::plain(": &["),
                StyledSpan::colored("u8", ansi(3)),
                StyledSpan::plain("]) -> "),
                StyledSpan::colored("Result", ansi(3)),
                StyledSpan::plain("<"),
                StyledSpan::colored("Vec", ansi(3)),
                StyledSpan::plain("<"),
                StyledSpan::colored("String", ansi(3)),
                StyledSpan::plain(">> {"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  42 ", ansi(11)),
                StyledSpan::plain("    "),
                StyledSpan::colored("let ", ansi(5)),
                StyledSpan::colored("mut ", ansi(5)),
                StyledSpan::plain("results = "),
                StyledSpan::colored("Vec", ansi(3)),
                StyledSpan::plain("::new();"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  43 ", ansi(8)).dim(),
                StyledSpan::plain("    "),
                StyledSpan::colored("for ", ansi(5)),
                StyledSpan::plain("chunk "),
                StyledSpan::colored("in ", ansi(5)),
                StyledSpan::plain("data.chunks("),
                StyledSpan::colored("64", ansi(11)),
                StyledSpan::plain(") {"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  44 ", ansi(8)).dim(),
                StyledSpan::plain("        "),
                StyledSpan::colored("let ", ansi(5)),
                StyledSpan::plain("parsed = "),
                StyledSpan::colored("String", ansi(3)),
                StyledSpan::plain("::from_utf8(chunk)"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  45 ", ansi(8)).dim(),
                StyledSpan::plain("            .map_err(|e| e.to_string())?;"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  46 ", ansi(8)).dim(),
                StyledSpan::plain("        results.push(parsed);"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  47 ", ansi(8)).dim(),
                StyledSpan::plain("    }"),
            ]),
            // LSP diagnostic hint
            SceneLine::new(vec![
                StyledSpan::colored("  48 ", ansi(8)).dim(),
                StyledSpan::plain("    "),
                StyledSpan::colored("Ok", ansi(3)),
                StyledSpan::plain("(results)"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  49 ", ansi(8)).dim(),
                StyledSpan::plain("}"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("       hint: ", ansi(6)).italic(),
                StyledSpan::colored("consider adding `#[must_use]`", ansi(6)).italic(),
            ]),
        ],
    }
}

/// Python REPL scene with prompts, tracebacks, and output.
pub fn python_repl_scene() -> Scene {
    Scene {
        id: "python-repl".into(),
        name: "Python REPL".into(),
        description: "Interactive Python session with prompts, output, and tracebacks".into(),
        lines: vec![
            SceneLine::new(vec![
                StyledSpan::colored(">>> ", ansi(2)).bold(),
                StyledSpan::colored("import ", ansi(5)),
                StyledSpan::plain("json"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(">>> ", ansi(2)).bold(),
                StyledSpan::plain("data = {"),
                StyledSpan::colored("\"name\"", ansi(2)),
                StyledSpan::plain(": "),
                StyledSpan::colored("\"litmus\"", ansi(2)),
                StyledSpan::plain(", "),
                StyledSpan::colored("\"version\"", ansi(2)),
                StyledSpan::plain(": "),
                StyledSpan::colored("3", ansi(11)),
                StyledSpan::plain("}"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(">>> ", ansi(2)).bold(),
                StyledSpan::colored("print", ansi(6)),
                StyledSpan::plain("(json.dumps(data, indent="),
                StyledSpan::colored("2", ansi(11)),
                StyledSpan::plain("))"),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("{"),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("  "),
                StyledSpan::colored("\"name\"", ansi(6)),
                StyledSpan::plain(": "),
                StyledSpan::colored("\"litmus\"", ansi(2)),
                StyledSpan::plain(","),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("  "),
                StyledSpan::colored("\"version\"", ansi(6)),
                StyledSpan::plain(": "),
                StyledSpan::colored("3", ansi(11)),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("}"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(">>> ", ansi(2)).bold(),
                StyledSpan::plain("data["),
                StyledSpan::colored("\"missing_key\"", ansi(2)),
                StyledSpan::plain("]"),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("Traceback (most recent call last):", ansi(1)),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("  File "),
                StyledSpan::colored("\"<stdin>\"", ansi(13)),
                StyledSpan::plain(", line "),
                StyledSpan::colored("1", ansi(11)),
                StyledSpan::plain(", in "),
                StyledSpan::colored("<module>", ansi(6)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("KeyError", ansi(1)).bold(),
                StyledSpan::colored(": ", ansi(1)),
                StyledSpan::colored("'missing_key'", ansi(2)),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(">>> ", ansi(2)).bold(),
                StyledSpan::plain("[x**"),
                StyledSpan::colored("2 ", ansi(11)),
                StyledSpan::colored("for ", ansi(5)),
                StyledSpan::plain("x "),
                StyledSpan::colored("in ", ansi(5)),
                StyledSpan::colored("range", ansi(6)),
                StyledSpan::plain("("),
                StyledSpan::colored("8", ansi(11)),
                StyledSpan::plain(")]"),
            ]),
            SceneLine::new(vec![
                StyledSpan::plain("["),
                StyledSpan::colored("0", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("1", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("4", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("9", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("16", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("25", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("36", ansi(11)),
                StyledSpan::plain(", "),
                StyledSpan::colored("49", ansi(11)),
                StyledSpan::plain("]"),
            ]),
        ],
    }
}

/// htop-style process viewer with CPU/memory bars and process list.
pub fn htop_scene() -> Scene {
    Scene {
        id: "htop".into(),
        name: "System Monitor".into(),
        description: "htop-style view with CPU bars, memory usage, and process list".into(),
        lines: vec![
            // CPU bars
            SceneLine::new(vec![
                StyledSpan::colored("  1", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||||||||||||", ansi(2)),
                StyledSpan::colored("||||", ansi(11)),
                StyledSpan::colored("||", ansi(1)),
                StyledSpan::plain("                  "),
                StyledSpan::colored("32.4%", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
                StyledSpan::plain("  "),
                StyledSpan::colored("  3", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||||||", ansi(2)),
                StyledSpan::colored("|||", ansi(11)),
                StyledSpan::plain("                       "),
                StyledSpan::colored("18.7%", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  2", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||||||||||||||||||", ansi(2)),
                StyledSpan::colored("|||||", ansi(11)),
                StyledSpan::colored("||||", ansi(1)),
                StyledSpan::plain("       "),
                StyledSpan::colored("57.1%", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
                StyledSpan::plain("  "),
                StyledSpan::colored("  4", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||||", ansi(2)),
                StyledSpan::plain("                          "),
                StyledSpan::colored(" 8.3%", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
            ]),
            // Memory
            SceneLine::new(vec![
                StyledSpan::colored("  Mem", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||||||||||||||||||||", ansi(2)),
                StyledSpan::colored("|||||||", ansi(11)),
                StyledSpan::plain("          "),
                StyledSpan::colored("5.2G", ThemeColor::Foreground),
                StyledSpan::plain("/"),
                StyledSpan::colored("16.0G", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  Swp", ansi(6)),
                StyledSpan::colored("[", ThemeColor::Foreground),
                StyledSpan::colored("||", ansi(2)),
                StyledSpan::plain("                                   "),
                StyledSpan::colored("0.1G", ThemeColor::Foreground),
                StyledSpan::plain("/"),
                StyledSpan::colored("8.0G", ThemeColor::Foreground),
                StyledSpan::colored("]", ThemeColor::Foreground),
            ]),
            SceneLine::empty(),
            // Column headers
            SceneLine::new(vec![
                StyledSpan::colored("  PID", ansi(2)).bold(),
                StyledSpan::colored(" USER     ", ansi(2)).bold(),
                StyledSpan::colored(" PRI", ansi(2)).bold(),
                StyledSpan::colored("  NI", ansi(2)).bold(),
                StyledSpan::colored("   VIRT", ansi(2)).bold(),
                StyledSpan::colored("    RES", ansi(2)).bold(),
                StyledSpan::colored("  CPU%", ansi(2)).bold(),
                StyledSpan::colored("  MEM%", ansi(2)).bold(),
                StyledSpan::colored("  Command", ansi(2)).bold(),
            ]),
            // Processes
            SceneLine::new(vec![
                StyledSpan::colored(" 1842", ansi(2)),
                StyledSpan::plain(" user      20   0  2.1G  384M"),
                StyledSpan::colored("  24.3", ansi(11)),
                StyledSpan::plain("   2.4"),
                StyledSpan::colored("  cargo build --release", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(" 2104", ansi(2)),
                StyledSpan::plain(" user      20   0  1.8G  220M"),
                StyledSpan::colored("  18.1", ansi(11)),
                StyledSpan::plain("   1.4"),
                StyledSpan::colored("  nvim src/main.rs", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  903", ansi(2)),
                StyledSpan::plain(" user      20   0  812M  164M"),
                StyledSpan::plain("   6.2"),
                StyledSpan::plain("   1.0"),
                StyledSpan::colored("  kitty", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored("  456", ansi(1)),
                StyledSpan::colored(" root", ansi(1)),
                StyledSpan::plain("      20   0  412M   48M"),
                StyledSpan::plain("   2.1"),
                StyledSpan::plain("   0.3"),
                StyledSpan::colored("  systemd", ThemeColor::Foreground),
            ]),
            SceneLine::new(vec![
                StyledSpan::colored(" 3201", ansi(2)),
                StyledSpan::plain(" user      20   0  620M  102M"),
                StyledSpan::plain("   1.4"),
                StyledSpan::plain("   0.6"),
                StyledSpan::colored("  firefox", ThemeColor::Foreground),
            ]),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_scenes_non_empty() {
        let scenes = all_scenes();
        assert_eq!(scenes.len(), 8);
        for scene in &scenes {
            assert!(!scene.id.is_empty());
            assert!(!scene.name.is_empty());
            assert!(!scene.lines.is_empty());
        }
    }

    #[test]
    fn scenes_have_unique_ids() {
        let scenes = all_scenes();
        let mut ids: Vec<&str> = scenes.iter().map(|s| s.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), scenes.len());
    }

    #[test]
    fn git_diff_uses_red_green_cyan() {
        let scene = git_diff_scene();
        let colors_used: Vec<&ThemeColor> = scene
            .lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .filter_map(|s| s.fg.as_ref())
            .collect();
        // Should use red(1), green(2), and cyan(6)
        assert!(colors_used.contains(&&ThemeColor::Ansi(1)));
        assert!(colors_used.contains(&&ThemeColor::Ansi(2)));
        assert!(colors_used.contains(&&ThemeColor::Ansi(6)));
    }
}
