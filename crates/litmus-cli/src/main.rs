mod theme_data;
mod widgets;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use litmus_model::Theme;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color as RColor, Style},
    text::{Line, Span},
};
use std::{io, path::Path, process::Command};
use widgets::{LiveWidget, MockupsWidget, SwatchesWidget};

/// Parse --provider <slug> from CLI args, returning the provider and remaining args.
fn parse_provider_arg(args: &[String]) -> (Option<String>, Vec<String>) {
    let mut provider = None;
    let mut rest = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--provider" {
            match iter.next() {
                Some(val) => provider = Some(val.clone()),
                None => eprintln!("Warning: --provider requires a value (e.g. --provider kitty)"),
            }
        } else if let Some(val) = arg.strip_prefix("--provider=") {
            provider = Some(val.to_string());
        } else {
            rest.push(arg.clone());
        }
    }
    (provider, rest)
}

#[derive(Clone, Copy)]
enum View {
    Swatches,
    Mockups,
    Live,
}

impl View {
    fn name(self) -> &'static str {
        match self {
            View::Swatches => "Swatches",
            View::Mockups => "Mockups",
            View::Live => "Live",
        }
    }
}

struct App {
    themes: Vec<Theme>,
    theme_index: usize,
    view: View,
    fixture_index: usize,
    git_diff: Vec<String>,
    ls_output: Vec<String>,
}

impl App {
    fn new(extra_themes: Vec<Theme>, provider: Option<&str>) -> Self {
        let mut themes = theme_data::load_bundled_themes(provider);
        themes.extend(extra_themes);
        if themes.is_empty() {
            themes = theme_data::all_themes();
        }
        App {
            themes,
            theme_index: 0,
            view: View::Swatches,
            fixture_index: 0,
            git_diff: capture_command("git", &["diff"]),
            ls_output: capture_command("ls", &["-la", "--color=never"]),
        }
    }

    fn current_theme(&self) -> &Theme {
        &self.themes[self.theme_index]
    }

    fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % self.themes.len();
    }

    fn prev_theme(&mut self) {
        self.theme_index = (self.theme_index + self.themes.len() - 1) % self.themes.len();
    }

    fn next_view(&mut self) {
        self.view = match self.view {
            View::Swatches => View::Mockups,
            View::Mockups => View::Live,
            View::Live => View::Swatches,
        };
    }

    fn next_fixture(&mut self) {
        let count = widgets::fixture_count();
        if count > 0 {
            self.fixture_index = (self.fixture_index + 1) % count;
        }
    }

    fn prev_fixture(&mut self) {
        let count = widgets::fixture_count();
        if count > 0 {
            self.fixture_index = (self.fixture_index + count - 1) % count;
        }
    }

    fn prev_view(&mut self) {
        self.view = match self.view {
            View::Swatches => View::Live,
            View::Mockups => View::Swatches,
            View::Live => View::Mockups,
        };
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let (provider, remaining_args) = parse_provider_arg(&args);

    let theme_paths: Vec<_> = remaining_args
        .iter()
        .filter(|a| {
            a.ends_with(".conf")
                || a.ends_with(".yaml")
                || a.ends_with(".yml")
                || a.ends_with(".toml")
        })
        .collect();
    let extra_themes: Vec<Theme> = theme_paths
        .iter()
        .filter_map(|p| {
            let path = Path::new(p.as_str());
            match theme_data::load_theme(path) {
                Ok(t) => Some(t),
                Err(e) => {
                    eprintln!("Warning: could not load theme from {p}: {e}");
                    None
                }
            }
        })
        .collect();

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal, extra_themes, provider.as_deref());
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn capture_command(program: &str, args: &[&str]) -> Vec<String> {
    Command::new(program)
        .args(args)
        .output()
        .map(|out| {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| l.to_owned())
                .collect()
        })
        .unwrap_or_default()
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, extra_themes: Vec<Theme>, provider: Option<&str>) -> Result<()> {
    let mut app = App::new(extra_themes, provider);

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(frame.area());

            let theme = app.current_theme();
            match app.view {
                View::Swatches => frame.render_widget(SwatchesWidget { theme }, chunks[0]),
                View::Mockups => frame.render_widget(MockupsWidget { theme, fixture_index: app.fixture_index }, chunks[0]),
                View::Live => frame.render_widget(
                    LiveWidget { theme, git_diff: &app.git_diff, ls_output: &app.ls_output },
                    chunks[0],
                ),
            }

            let status = Line::from(vec![
                Span::styled(
                    format!(
                        " {} [{}/{}] ",
                        app.current_theme().name,
                        app.theme_index + 1,
                        app.themes.len()
                    ),
                    Style::default().fg(RColor::Yellow),
                ),
                Span::styled(" | ", Style::default().fg(RColor::DarkGray)),
                Span::styled(app.view.name(), Style::default().fg(RColor::Cyan)),
                Span::styled(" | ", Style::default().fg(RColor::DarkGray)),
                Span::styled(
                    "←/→ theme  ↑/↓ fixture  Tab/S-Tab view  q quit",
                    Style::default().fg(RColor::DarkGray),
                ),
            ]);
            frame.render_widget(status, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => app.next_view(),
                KeyCode::BackTab => app.prev_view(),
                KeyCode::Left => app.prev_theme(),
                KeyCode::Right => app.next_theme(),
                KeyCode::Up => app.prev_fixture(),
                KeyCode::Down => app.next_fixture(),
                _ => {}
            }
        }
    }
}
