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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_scenes_non_empty() {
        let scenes = all_scenes();
        assert_eq!(scenes.len(), 5);
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
