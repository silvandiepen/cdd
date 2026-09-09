//! The core query loop shared by every front end: the shell adapter, the
//! one-shot CLI and the fallback picker all go through this.

use std::path::{Path, PathBuf};

use crate::filesystem::{self, Cache};
use crate::matcher::{self, Score};
use crate::output::{self, Response, Row};
use crate::path::{self, Query};

/// How many rows the preview shows before collapsing into `+ N more`.
pub const DEFAULT_MAX_ROWS: usize = 8;
/// Assumed terminal width when the shell does not report one.
pub const DEFAULT_WIDTH: usize = 80;
/// Columns reserved for the selection marker in front of every row.
const MARKER_WIDTH: usize = 2;

/// What a front end asks for.
#[derive(Debug, Clone)]
pub struct Request {
    /// The text typed after `cdd`, exactly as it appears in the command buffer.
    pub raw: String,
    pub cwd: PathBuf,
    pub home: Option<PathBuf>,
    pub width: usize,
    pub max_rows: usize,
}

impl Request {
    pub fn new(raw: impl Into<String>, cwd: impl Into<PathBuf>) -> Self {
        Request {
            raw: raw.into(),
            cwd: cwd.into(),
            home: std::env::var_os("HOME").map(PathBuf::from),
            width: DEFAULT_WIDTH,
            max_rows: DEFAULT_MAX_ROWS,
        }
    }
}

/// Holds the cached listing across the keystrokes of one interaction.
#[derive(Debug, Default)]
pub struct Session {
    cache: Cache,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rank the directories under the typed path and build a renderable answer.
    pub fn resolve(&mut self, request: &Request) -> Response {
        let query = path::parse(&request.raw, &request.cwd, request.home.as_deref());

        let listing = match self.cache.get(&query.base) {
            Ok(listing) => listing,
            Err(error) => {
                return Response {
                    base: query.base,
                    error: Some(filesystem::describe(&error)),
                    ..Response::default()
                }
            }
        };

        let mut candidates: Vec<(Score, &str, PathBuf)> = Vec::new();

        // `.` and `..` are never returned by a directory read, but they are
        // valid destinations, so they join the candidate list on the same terms
        // as everything else once hidden entries are eligible.
        if query.wants_hidden() {
            for name in [".", ".."] {
                if let Some(score) = matcher::score(name, &query.filter) {
                    candidates.push((score, name, query.base.join(name)));
                }
            }
        }

        for entry in &listing.entries {
            if entry.name.starts_with('.') && !query.wants_hidden() {
                continue;
            }
            if let Some(score) = matcher::score(&entry.name, &query.filter) {
                candidates.push((score, &entry.name, entry.path.clone()));
            }
        }

        if query.filter.is_empty() {
            candidates.sort_by(|a, b| matcher::compare_names(a.1, b.1));
        } else {
            candidates.sort_by(|a, b| matcher::compare((&a.0, a.1), (&b.0, b.1)));
        }

        let total = candidates.len();
        let visible = total.min(request.max_rows.max(1));
        // A caller that does not know the width gets the default rather than a
        // budget of zero, which would render every row blank.
        let width = if request.width == 0 {
            DEFAULT_WIDTH
        } else {
            request.width
        };
        let name_width = width.saturating_sub(MARKER_WIDTH).max(1);

        let rows = candidates
            .into_iter()
            .take(visible)
            .map(|(_, name, target)| Row {
                insert: format!("{}{}/", query.typed_prefix, output::quote(name)),
                display: output::truncate(&output::sanitize(name), name_width),
                path: path::normalize(&target),
            })
            .collect();

        Response {
            exact: exact_destination(&query, &request.cwd, request.home.as_deref()),
            base: query.base,
            rows,
            hidden_matches: total - visible,
            error: None,
        }
    }

    /// Forget the cached listing.
    pub fn invalidate(&mut self) {
        self.cache.clear();
    }
}

/// The directory the typed text names outright, if it names one.
///
/// This is what makes `cdd ..`, `cdd ~/Projects` and `cdd /usr/local` behave
/// exactly like `cd` even when the filter also produced matches.
fn exact_destination(query: &Query, cwd: &Path, home: Option<&Path>) -> Option<PathBuf> {
    let typed = format!("{}{}", query.typed_prefix, query.filter);
    if typed.is_empty() {
        return None;
    }
    let resolved = path::resolve(&typed, cwd, home);
    filesystem::is_directory(&resolved).then_some(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct Temp(PathBuf);

    impl Temp {
        fn new(tag: &str) -> Self {
            let base =
                std::env::temp_dir().join(format!("cdd-session-{tag}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            fs::create_dir_all(&base).unwrap();
            Temp(fs::canonicalize(&base).unwrap())
        }

        fn dirs(&self, names: &[&str]) -> &Self {
            for name in names {
                fs::create_dir_all(self.0.join(name)).unwrap();
            }
            self
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn request(raw: &str, cwd: &Path) -> Request {
        Request {
            raw: raw.to_string(),
            cwd: cwd.to_path_buf(),
            home: Some(cwd.to_path_buf()),
            width: DEFAULT_WIDTH,
            max_rows: DEFAULT_MAX_ROWS,
        }
    }

    fn displays(response: &Response) -> Vec<String> {
        response.rows.iter().map(|r| r.display.clone()).collect()
    }

    #[test]
    fn filters_the_current_directory_while_typing() {
        let t = Temp::new("filter");
        t.dirs(&["components", "composables", "config", "core"]);
        let mut session = Session::new();

        assert_eq!(
            displays(&session.resolve(&request("", &t.0))),
            vec!["components", "composables", "config", "core"]
        );
        assert_eq!(
            displays(&session.resolve(&request("co", &t.0))),
            vec!["core", "config", "components", "composables"]
        );
        assert_eq!(
            displays(&session.resolve(&request("comp", &t.0))),
            vec!["components", "composables"]
        );
    }

    #[test]
    fn browses_a_nested_path_by_its_completed_segments() {
        let t = Temp::new("nested");
        t.dirs(&["src/components", "src/composables", "src/config", "other"]);
        let mut session = Session::new();

        let response = session.resolve(&request("src/co", &t.0));
        assert_eq!(response.base, t.0.join("src"));
        assert_eq!(
            displays(&response),
            vec!["config", "components", "composables"]
        );
    }

    #[test]
    fn tab_insert_rebuilds_the_typed_argument() {
        let t = Temp::new("insert");
        t.dirs(&["src/components"]);
        let mut session = Session::new();

        let response = session.resolve(&request("src/co", &t.0));
        assert_eq!(response.rows[0].insert, "src/components/");

        // And browsing that completion lists its children.
        let response = session.resolve(&request("src/components/", &t.0));
        assert_eq!(response.base, t.0.join("src/components"));
    }

    #[test]
    fn a_name_needing_quotes_is_inserted_quoted() {
        let t = Temp::new("quoted-insert");
        t.dirs(&["src/my dir"]);
        let mut session = Session::new();

        let response = session.resolve(&request("src/my", &t.0));
        assert_eq!(response.rows[0].insert, "src/'my dir'/");
    }

    #[test]
    fn hidden_directories_appear_only_for_a_dotted_filter() {
        let t = Temp::new("hidden");
        t.dirs(&[".git", ".github", "src"]);
        let mut session = Session::new();

        assert_eq!(displays(&session.resolve(&request("", &t.0))), vec!["src"]);

        let dotted = displays(&session.resolve(&request(".g", &t.0)));
        assert_eq!(dotted, vec![".git", ".github"]);
    }

    #[test]
    fn dot_dot_is_offered_and_resolves_to_the_parent() {
        let t = Temp::new("parent");
        t.dirs(&["src"]);
        let mut session = Session::new();

        let response = session.resolve(&request("..", &t.0));
        assert_eq!(response.rows[0].display, "..");
        assert_eq!(response.rows[0].path, path::normalize(&t.0.join("..")));
        assert_eq!(response.exact, Some(path::normalize(&t.0.join(".."))));
    }

    #[test]
    fn an_exactly_typed_directory_is_reported_alongside_the_matches() {
        let t = Temp::new("exact");
        t.dirs(&["src", "srcfoo"]);
        let mut session = Session::new();

        let response = session.resolve(&request("src", &t.0));
        assert_eq!(response.exact, Some(t.0.join("src")));
        assert_eq!(displays(&response), vec!["src", "srcfoo"]);

        assert_eq!(session.resolve(&request("nope", &t.0)).exact, None);
    }

    #[test]
    fn tilde_paths_are_browsable() {
        let t = Temp::new("tilde");
        t.dirs(&["Projects"]);
        let mut session = Session::new();

        let response = session.resolve(&request("~/Pro", &t.0));
        assert_eq!(displays(&response), vec!["Projects"]);
        assert_eq!(response.rows[0].insert, "~/Projects/");
    }

    #[test]
    fn no_matches_yields_an_empty_result_not_an_error() {
        let t = Temp::new("nomatch");
        t.dirs(&["src"]);
        let mut session = Session::new();

        let response = session.resolve(&request("zzz", &t.0));
        assert!(response.rows.is_empty());
        assert!(response.error.is_none());
        assert_eq!(response.hidden_matches, 0);
    }

    #[test]
    fn an_unreadable_base_directory_reports_an_error_and_no_rows() {
        let t = Temp::new("perm");
        t.dirs(&["locked"]);
        let locked = t.0.join("locked");
        let mut perms = fs::metadata(&locked).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o000);
        fs::set_permissions(&locked, perms.clone()).unwrap();

        let mut session = Session::new();
        let response = session.resolve(&request("locked/", &t.0));
        assert_eq!(response.error.as_deref(), Some("Permission denied"));
        assert!(response.rows.is_empty());

        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
        fs::set_permissions(&locked, perms).unwrap();
    }

    #[test]
    fn overflowing_matches_are_counted_not_dropped_silently() {
        let t = Temp::new("overflow");
        let names: Vec<String> = (0..12).map(|i| format!("dir{i:02}")).collect();
        t.dirs(&names.iter().map(String::as_str).collect::<Vec<_>>());

        let mut session = Session::new();
        let response = session.resolve(&request("dir", &t.0));
        assert_eq!(response.rows.len(), DEFAULT_MAX_ROWS);
        assert_eq!(response.hidden_matches, 4);
    }

    #[test]
    fn an_unknown_terminal_width_does_not_blank_the_rows() {
        let t = Temp::new("zerowidth");
        t.dirs(&["components"]);

        let mut session = Session::new();
        let mut req = request("comp", &t.0);
        req.width = 0;
        assert_eq!(displays(&session.resolve(&req)), vec!["components"]);
    }

    #[test]
    fn long_names_are_truncated_to_the_terminal_width() {
        let t = Temp::new("width");
        t.dirs(&["a-very-long-directory-name-indeed"]);

        let mut session = Session::new();
        let mut req = request("a", &t.0);
        req.width = 12;
        let response = session.resolve(&req);
        assert_eq!(response.rows[0].display, "a-very-lo…");
        // The full path stays intact for navigation.
        assert_eq!(
            response.rows[0].path,
            t.0.join("a-very-long-directory-name-indeed")
        );
    }

    #[test]
    fn names_with_newlines_stay_navigable() {
        let t = Temp::new("newline");
        t.dirs(&["odd\nname"]);

        let mut session = Session::new();
        let response = session.resolve(&request("odd", &t.0));
        assert_eq!(response.rows[0].display, "odd\u{fffd}name");
        assert_eq!(response.rows[0].path, t.0.join("odd\nname"));

        let encoded = response.encode();
        assert_eq!(encoded.lines().count(), 4);
    }
}
