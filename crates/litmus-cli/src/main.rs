mod theme_data;
mod widgets;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color as RColor, Style},
    text::{Line, Span},
};
use std::{io, process::Command};
use theme_data::ThemeWithExtras;
use widgets::{LiveWidget, MockupsWidget, SwatchesWidget};

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
    themes: Vec<ThemeWithExtras>,
    theme_index: usize,
    view: View,
    git_diff: Vec<String>,
    ls_output: Vec<String>,
}

impl App {
    fn new() -> Self {
        App {
            themes: theme_data::all_themes(),
            theme_index: 0,
            view: View::Swatches,
            git_diff: capture_command("git", &["diff"]),
            ls_output: capture_command("ls", &["-la", "--color=never"]),
        }
    }

    fn current_theme(&self) -> &ThemeWithExtras {
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

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
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

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(frame.area());

            let theme = app.current_theme();
            match app.view {
                View::Swatches => frame.render_widget(SwatchesWidget { theme }, chunks[0]),
                View::Mockups => frame.render_widget(MockupsWidget { theme }, chunks[0]),
                View::Live => frame.render_widget(
                    LiveWidget { theme, git_diff: &app.git_diff, ls_output: &app.ls_output },
                    chunks[0],
                ),
            }

            let status = Line::from(vec![
                Span::styled(
                    format!(
                        " {} [{}/{}] ",
                        app.current_theme().theme.name,
                        app.theme_index + 1,
                        app.themes.len()
                    ),
                    Style::default().fg(RColor::Yellow),
                ),
                Span::styled(" | ", Style::default().fg(RColor::DarkGray)),
                Span::styled(app.view.name(), Style::default().fg(RColor::Cyan)),
                Span::styled(" | ", Style::default().fg(RColor::DarkGray)),
                Span::styled(
                    "←/→ theme  Tab/S-Tab view  q quit",
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
                _ => {}
            }
        }
    }
}
