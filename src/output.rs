//! Terminal-safe rendering and the binary-to-shell wire format.
//!
//! Directory names may contain newlines, tabs, quotes and emoji, so nothing
//! here assumes a path is a well-behaved single line of ASCII.

use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Protocol version, bumped whenever the shell adapter would need updating.
pub const PROTOCOL_VERSION: u32 = 1;

/// Replace anything that would corrupt a single rendered row.
///
/// Control characters, including the newlines and tabs that are legal in a
/// filename, become a replacement character so a row stays a row.
pub fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_control() { '\u{fffd}' } else { c })
        .collect()
}

/// Truncate to a display width, never splitting a grapheme cluster.
///
/// The ellipsis is included in the budget, so the result is always at most
/// `width` columns wide.
pub fn truncate(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    if text.width() <= width {
        return text.to_string();
    }
    if width == 1 {
        return "…".to_string();
    }

    let budget = width - 1;
    let mut out = String::new();
    let mut used = 0usize;

    for grapheme in text.graphemes(true) {
        let w = grapheme_width(grapheme);
        if used + w > budget {
            break;
        }
        out.push_str(grapheme);
        used += w;
    }

    out.push('…');
    out
}

/// A grapheme cluster occupies at least one column even when every code point
/// in it reports zero width.
fn grapheme_width(grapheme: &str) -> usize {
    let w: usize = grapheme.chars().map(|c| c.width().unwrap_or(0)).sum();
    w.max(1)
}

/// Quote a string so zsh's `${(Q)}` yields the original bytes back.
///
/// Names that need no quoting are emitted bare, which keeps `Tab` completions
/// looking like something the user would have typed themselves.
pub fn quote(text: &str) -> String {
    if text.is_empty() {
        return "''".to_string();
    }

    if text.chars().any(char::is_control) {
        return ansi_c_quote(text);
    }

    if !needs_quoting(text) {
        return text.to_string();
    }

    let mut out = String::with_capacity(text.len() + 2);
    out.push('\'');
    for c in text.chars() {
        if c == '\'' {
            // Close, escape, reopen — the only way out of a zsh single quote.
            out.push_str("'\\''");
        } else {
            out.push(c);
        }
    }
    out.push('\'');
    out
}

fn needs_quoting(text: &str) -> bool {
    // A leading one of these starts an expansion or an option, whatever follows.
    let starts_an_expansion = matches!(text.chars().next(), Some('~' | '-' | '=' | '#'));
    starts_an_expansion || !text.chars().all(is_bare)
}

/// Characters the shell passes through untouched inside a bare word.
fn is_bare(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(c, '_' | '-' | '.' | '/' | '+' | ',' | ':' | '@' | '%')
        // Printable non-ASCII has no special meaning to zsh.
        || (!c.is_ascii() && !c.is_control())
}

fn ansi_c_quote(text: &str) -> String {
    let mut out = String::from("$'");
    for c in text.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            c if c.is_control() => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// Escape a field for the tab-separated request line the shell sends.
///
/// The response uses zsh quoting instead, because only the shell side needs to
/// decode that direction and `${(Q)}` does it exactly.
pub fn escape_field(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

/// Inverse of [`escape_field`].
pub fn unescape_field(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

/// One row of the preview, already safe to print.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// Absolute path to navigate to.
    pub path: std::path::PathBuf,
    /// Replacement text for the `cdd` argument when this row is completed with Tab.
    pub insert: String,
    /// Single-line, width-fitted text to display.
    pub display: String,
}

/// The full answer to one `list` request.
#[derive(Debug, Clone, Default)]
pub struct Response {
    pub base: std::path::PathBuf,
    /// Set when the typed text itself already names an existing directory.
    pub exact: Option<std::path::PathBuf>,
    pub rows: Vec<Row>,
    /// Matches beyond the visible rows.
    pub hidden_matches: usize,
    pub error: Option<String>,
}

impl Response {
    /// Serialize in the line-oriented format the shell adapter parses.
    ///
    /// Paths are zsh-quoted so they survive newlines and metacharacters;
    /// `display` is already sanitized to one line and needs no quoting.
    pub fn encode(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("CDD\t{PROTOCOL_VERSION}\n"));
        out.push_str(&format!("BASE\t{}\n", quote_path(&self.base)));
        if let Some(exact) = &self.exact {
            out.push_str(&format!("EXACT\t{}\n", quote_path(exact)));
        }
        for row in &self.rows {
            out.push_str(&format!(
                "R\t{}\t{}\t{}\n",
                quote_path(&row.path),
                quote(&row.insert),
                row.display
            ));
        }
        if self.hidden_matches > 0 {
            out.push_str(&format!("MORE\t{}\n", self.hidden_matches));
        }
        if let Some(error) = &self.error {
            out.push_str(&format!("ERR\t{}\n", sanitize(error)));
        }
        out.push_str("END\n");
        out
    }
}

fn quote_path(path: &Path) -> String {
    quote(&path.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_characters_never_reach_a_row() {
        assert_eq!(sanitize("a\nb"), "a\u{fffd}b");
        assert_eq!(sanitize("a\tb"), "a\u{fffd}b");
        assert_eq!(sanitize("plain"), "plain");
    }

    #[test]
    fn truncation_respects_the_width_budget() {
        assert_eq!(truncate("components", 20), "components");
        assert_eq!(truncate("components", 10), "components");
        assert_eq!(truncate("components", 6), "compo…");
        assert_eq!(truncate("components", 1), "…");
        assert_eq!(truncate("components", 0), "");
    }

    #[test]
    fn truncation_never_splits_a_grapheme() {
        // A family emoji is one cluster made of several code points.
        let name = "a👨‍👩‍👧‍👦b";
        for width in 1..12 {
            let cut = truncate(name, width);
            assert!(cut.width() <= width, "{cut:?} exceeds {width}");
            assert!(name.starts_with(cut.trim_end_matches('…')));
        }
    }

    #[test]
    fn wide_characters_are_measured_by_columns() {
        assert_eq!(truncate("プロジェクト", 6), "プロ…");
    }

    #[test]
    fn quoting_leaves_ordinary_names_alone() {
        assert_eq!(quote("components"), "components");
        assert_eq!(quote("src/components/"), "src/components/");
        assert_eq!(quote("プロジェクト"), "プロジェクト");
    }

    #[test]
    fn quoting_protects_metacharacters() {
        assert_eq!(quote("my dir"), "'my dir'");
        assert_eq!(quote("a;b&c"), "'a;b&c'");
        assert_eq!(quote("it's"), r"'it'\''s'");
        assert_eq!(quote("~weird"), "'~weird'");
        assert_eq!(quote("-dash"), "'-dash'");
        assert_eq!(quote(""), "''");
    }

    #[test]
    fn quoting_encodes_newlines_in_names() {
        assert_eq!(quote("a\nb"), "$'a\\nb'");
        assert_eq!(quote("a\tb"), "$'a\\tb'");
    }

    #[test]
    fn request_fields_round_trip() {
        for original in [
            "plain",
            "with\ttab",
            "with\nnewline",
            r"back\slash",
            r"tricky\\ttab",
        ] {
            assert_eq!(unescape_field(&escape_field(original)), original);
        }
    }

    #[test]
    fn a_response_is_always_one_record_per_line() {
        let response = Response {
            base: std::path::PathBuf::from("/tmp"),
            exact: None,
            rows: vec![Row {
                path: std::path::PathBuf::from("/tmp/a\nb"),
                insert: "a\nb/".to_string(),
                display: sanitize("a\nb"),
            }],
            hidden_matches: 3,
            error: None,
        };

        let encoded = response.encode();
        assert_eq!(encoded.lines().count(), 5);
        assert!(encoded.contains("R\t$'/tmp/a\\nb'\t$'a\\nb/'\ta\u{fffd}b\n"));
        assert!(encoded.ends_with("MORE\t3\nEND\n"));
    }
}
