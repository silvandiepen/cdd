//! A long-lived request/response loop over stdin/stdout.
//!
//! The shell adapter keeps one of these open for the duration of a shell
//! session so the directory listing can stay cached between keystrokes, which
//! is what makes the preview feel instant. It is an optimisation only: every
//! request is equivalent to a one-shot `cdd list`, and the adapter falls back
//! to that if the process is unavailable.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use crate::output::unescape_field;
use crate::session::{Request, Session, DEFAULT_MAX_ROWS, DEFAULT_WIDTH};

/// Run the loop until stdin closes or a `QUIT` request arrives.
pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut session = Session::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let mut fields = line.split('\t');

        match fields.next() {
            Some("LIST") => {
                let request = parse_request(&mut fields);
                write!(stdout, "{}", session.resolve(&request).encode())?;
            }
            Some("DROP") => {
                // The adapter noticed something that invalidates the listing.
                session.invalidate();
                writeln!(stdout, "END")?;
            }
            Some("QUIT") => break,
            _ => writeln!(stdout, "END")?,
        }

        stdout.flush()?;
    }

    Ok(())
}

/// `LIST <cwd> <raw query> <width> <max rows>`, with tabs, newlines and
/// backslashes escaped in the two text fields.
fn parse_request<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Request {
    let cwd = fields.next().map(unescape_field).unwrap_or_default();
    let raw = fields.next().map(unescape_field).unwrap_or_default();
    let width = fields
        .next()
        .and_then(|f| f.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);
    let max_rows = fields
        .next()
        .and_then(|f| f.parse().ok())
        .unwrap_or(DEFAULT_MAX_ROWS);

    let cwd = if cwd.is_empty() {
        crate::path::working_directory().unwrap_or_else(|_| PathBuf::from("/"))
    } else {
        PathBuf::from(cwd)
    };

    Request {
        raw,
        cwd,
        home: std::env::var_os("HOME").map(PathBuf::from),
        width,
        max_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_line_is_parsed_field_by_field() {
        let line = "LIST\t/tmp\tsrc/co\t100\t5";
        let mut fields = line.split('\t');
        assert_eq!(fields.next(), Some("LIST"));

        let request = parse_request(&mut fields);
        assert_eq!(request.cwd, PathBuf::from("/tmp"));
        assert_eq!(request.raw, "src/co");
        assert_eq!(request.width, 100);
        assert_eq!(request.max_rows, 5);
    }

    #[test]
    fn escaped_tabs_and_newlines_survive_the_request() {
        let line = "LIST\t/tmp/odd\\nname\ta\\tb\t80\t8";
        let mut fields = line.split('\t');
        fields.next();

        let request = parse_request(&mut fields);
        assert_eq!(request.cwd, PathBuf::from("/tmp/odd\nname"));
        assert_eq!(request.raw, "a\tb");
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let line = "LIST\t/tmp";
        let mut fields = line.split('\t');
        fields.next();

        let request = parse_request(&mut fields);
        assert_eq!(request.raw, "");
        assert_eq!(request.width, DEFAULT_WIDTH);
        assert_eq!(request.max_rows, DEFAULT_MAX_ROWS);
    }
}
