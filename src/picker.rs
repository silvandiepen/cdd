//! The fallback interactive picker.
//!
//! This is what runs when the command was already executed, so live shell
//! integration never got the chance to see it being typed. It is the same
//! interaction one keystroke later, and it obeys the same rules: inline only,
//! no alternate screen, directories only, and nothing left behind on exit.
//!
//! The chosen path goes to stdout. Everything the user sees goes to the
//! terminal, so a shell function can capture one without the other.

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{cursor, queue};

use crate::output::Response;
use crate::session::{Request, Session, DEFAULT_MAX_ROWS, DEFAULT_WIDTH};

/// The outcome of one picker run.
pub enum Outcome {
    Chosen(PathBuf),
    Cancelled,
}

/// Run the picker, starting from `cwd` with `raw` already typed.
pub fn run(cwd: PathBuf, raw: String, max_rows: usize) -> io::Result<Outcome> {
    // The UI is written to the terminal directly so stdout stays a clean
    // channel for the single line the shell function reads back.
    let mut tty = File::options().write(true).open("/dev/tty")?;

    let mut state = State {
        session: Session::new(),
        cwd,
        raw,
        max_rows,
        selected: 0,
        drawn_rows: 0,
    };

    terminal::enable_raw_mode()?;
    let outcome = state.event_loop(&mut tty);
    let cleanup = state.erase(&mut tty);
    let raw_mode = terminal::disable_raw_mode();

    cleanup?;
    raw_mode?;
    outcome
}

struct State {
    session: Session,
    cwd: PathBuf,
    raw: String,
    max_rows: usize,
    selected: usize,
    /// Lines currently on screen, so they can be erased before the next draw.
    drawn_rows: u16,
}

impl State {
    fn event_loop(&mut self, tty: &mut File) -> io::Result<Outcome> {
        loop {
            let response = self.query();
            self.selected = self.selected.min(response.rows.len().saturating_sub(1));
            self.draw(tty, &response)?;

            let Event::Key(key) = event::read()? else {
                continue;
            };
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match self.handle(key, &response) {
                Action::Continue => {}
                Action::Cancel => return Ok(Outcome::Cancelled),
                Action::Choose(path) => return Ok(Outcome::Chosen(path)),
            }
        }
    }

    fn handle(&mut self, key: KeyEvent, response: &Response) -> Action {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match key.code {
            KeyCode::Char('c' | 'd' | 'g') if ctrl => Action::Cancel,
            KeyCode::Esc => Action::Cancel,

            KeyCode::Down => {
                if self.selected + 1 < response.rows.len() {
                    self.selected += 1;
                }
                Action::Continue
            }
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
                Action::Continue
            }

            KeyCode::Tab => {
                if let Some(row) = response.rows.get(self.selected) {
                    self.raw = row.insert.clone();
                    self.selected = 0;
                }
                Action::Continue
            }

            KeyCode::Enter => match response.rows.get(self.selected) {
                Some(row) => Action::Choose(row.path.clone()),
                None => match &response.exact {
                    Some(path) => Action::Choose(path.clone()),
                    None => Action::Continue,
                },
            },

            KeyCode::Backspace => {
                self.raw.pop();
                self.selected = 0;
                Action::Continue
            }

            KeyCode::Char(c) => {
                self.raw.push(c);
                self.selected = 0;
                Action::Continue
            }

            _ => Action::Continue,
        }
    }

    fn query(&mut self) -> Response {
        // A terminal that reports no size at all — a pty nobody sized, a
        // detached session — must not collapse the width budget to nothing.
        let width = terminal::size()
            .ok()
            .map(|(columns, _)| columns as usize)
            .filter(|columns| *columns > 0)
            .unwrap_or(DEFAULT_WIDTH);

        self.session.resolve(&Request {
            raw: self.raw.clone(),
            cwd: self.cwd.clone(),
            home: std::env::var_os("HOME").map(PathBuf::from),
            width,
            max_rows: self.max_rows,
        })
    }

    /// Redraw in place: erase what we drew last time, then print the query line
    /// and the rows below it. The cursor returns to the query line so the
    /// terminal behaves like a normal prompt.
    fn draw(&mut self, tty: &mut File, response: &Response) -> io::Result<()> {
        self.erase(tty)?;

        let mut lines = vec![format!("cdd {}", crate::output::sanitize(&self.raw))];

        if let Some(error) = &response.error {
            lines.push(format!("  {error}"));
        } else if response.rows.is_empty() {
            lines.push("  No matching directories".to_string());
        } else {
            for (index, row) in response.rows.iter().enumerate() {
                let marker = if index == self.selected {
                    "\u{25b8} "
                } else {
                    "  "
                };
                lines.push(format!("{marker}{}", row.display));
            }
            if response.hidden_matches > 0 {
                lines.push(format!("  + {} more", response.hidden_matches));
            }
        }

        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                write!(tty, "\r\n")?;
            }
            write!(tty, "{line}")?;
        }

        // Park the cursor at the end of the query line.
        let below = (lines.len() - 1) as u16;
        if below > 0 {
            queue!(tty, cursor::MoveUp(below))?;
        }
        queue!(tty, cursor::MoveToColumn(lines[0].chars().count() as u16))?;
        tty.flush()?;

        self.drawn_rows = below;
        Ok(())
    }

    /// Remove every line this picker drew, leaving the prompt untouched.
    fn erase(&mut self, tty: &mut File) -> io::Result<()> {
        queue!(
            tty,
            cursor::MoveToColumn(0),
            Clear(ClearType::FromCursorDown)
        )?;
        tty.flush()?;
        self.drawn_rows = 0;
        Ok(())
    }
}

enum Action {
    Continue,
    Cancel,
    Choose(PathBuf),
}

/// Convenience wrapper used by the CLI: run the picker and report the choice.
pub fn pick(cwd: PathBuf, raw: String) -> io::Result<Outcome> {
    let max_rows = std::env::var("CDD_MAX_ROWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_MAX_ROWS);
    run(cwd, raw, max_rows)
}
