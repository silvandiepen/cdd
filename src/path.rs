//! Turning the text the user has typed after `cdd` into a base directory and a
//! filter query.
//!
//! The last, incomplete path segment is the query. Everything before it is a
//! path that is resolved the way a shell would resolve it: `~` is the home
//! directory, `..` walks the logical parent (like zsh's default `cd`, which
//! does not chase symlinks), and anything else is relative to the cwd.

use std::path::{Component, Path, PathBuf};

/// A parsed `cdd` argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    /// The text before the final segment, verbatim as typed, including the
    /// trailing `/`. Kept unmodified so `Tab` can rebuild the command line
    /// without re-quoting what the user already wrote.
    pub typed_prefix: String,
    /// The directory whose children are listed.
    pub base: PathBuf,
    /// The final segment with shell quoting removed. This is the filter.
    pub filter: String,
}

impl Query {
    /// Hidden directories become eligible as soon as the filter starts with a dot,
    /// which is how the shell already behaves for globbing.
    pub fn wants_hidden(&self) -> bool {
        self.filter.starts_with('.')
    }
}

/// Parse the raw argument text of a `cdd` invocation.
pub fn parse(raw: &str, cwd: &Path, home: Option<&Path>) -> Query {
    let split = raw.rfind('/').map(|i| i + 1).unwrap_or(0);
    let (typed_prefix, last) = raw.split_at(split);

    let base = if typed_prefix.is_empty() {
        cwd.to_path_buf()
    } else {
        resolve(typed_prefix, cwd, home)
    };

    Query {
        typed_prefix: typed_prefix.to_string(),
        base,
        filter: unquote(last),
    }
}

/// Resolve a (possibly quoted, possibly relative) path fragment to an absolute path.
pub fn resolve(text: &str, cwd: &Path, home: Option<&Path>) -> PathBuf {
    let unquoted = unquote(text);
    let expanded = expand_tilde(&unquoted, home);
    let joined = if expanded.is_absolute() {
        expanded
    } else {
        cwd.join(expanded)
    };
    normalize(&joined)
}

/// Expand a leading `~` or `~/…`. `~user` is deliberately not supported; the
/// shell owns user-database expansion.
fn expand_tilde(text: &str, home: Option<&Path>) -> PathBuf {
    let Some(home) = home else {
        return PathBuf::from(text);
    };
    match text {
        "~" => home.to_path_buf(),
        _ if text.starts_with("~/") => home.join(&text[2..]),
        _ => PathBuf::from(text),
    }
}

/// The directory the shell believes it is in.
///
/// `$PWD` is preferred over the physical working directory so that a symlinked
/// directory keeps the path the user actually sees — the same logical model
/// zsh's `cd` uses. It is only trusted when it really names the current
/// directory, so a stale or forged value cannot redirect navigation.
pub fn working_directory() -> std::io::Result<PathBuf> {
    let physical = std::env::current_dir()?;
    let logical = std::env::var_os("PWD").map(PathBuf::from);
    Ok(prefer_logical(logical, physical))
}

fn prefer_logical(logical: Option<PathBuf>, physical: PathBuf) -> PathBuf {
    match logical {
        Some(logical) if logical.is_absolute() && same_directory(&logical, &physical) => logical,
        _ => physical,
    }
}

fn same_directory(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Lexically normalize a path: collapse `.` and resolve `..` without touching
/// the filesystem. This matches zsh's default logical `cd`, where `cd ..` from
/// a symlinked directory returns to the directory you came from.
pub fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    let mut popped_depth: usize = 0;

    for component in p.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                out.push(component.as_os_str());
                popped_depth = 0;
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if popped_depth > 0 {
                    out.pop();
                    popped_depth -= 1;
                } else if out.has_root() {
                    // `/..` is `/`.
                } else {
                    out.push("..");
                }
            }
            Component::Normal(part) => {
                out.push(part);
                popped_depth += 1;
            }
        }
    }

    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
}

/// Remove one level of shell quoting, tolerantly.
///
/// The input is text that is still being typed, so it is routinely unbalanced:
/// an unterminated quote takes the rest of the string literally rather than
/// being treated as an error.
pub fn unquote(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            }
            '\'' => {
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    out.push(c);
                }
            }
            '"' => {
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => {
                            // Inside double quotes only these are escapes.
                            match chars.next() {
                                Some(n @ ('"' | '\\' | '$' | '`')) => out.push(n),
                                Some(n) => {
                                    out.push('\\');
                                    out.push(n);
                                }
                                None => out.push('\\'),
                            }
                        }
                        _ => out.push(c),
                    }
                }
            }
            _ => out.push(c),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cwd() -> PathBuf {
        PathBuf::from("/Users/example/app")
    }

    fn home() -> PathBuf {
        PathBuf::from("/Users/example")
    }

    fn parsed(raw: &str) -> Query {
        parse(raw, &cwd(), Some(&home()))
    }

    #[test]
    fn empty_query_lists_the_current_directory() {
        let q = parsed("");
        assert_eq!(q.base, cwd());
        assert_eq!(q.filter, "");
        assert_eq!(q.typed_prefix, "");
    }

    #[test]
    fn final_segment_is_the_filter() {
        let q = parsed("src/co");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/src"));
        assert_eq!(q.filter, "co");
        assert_eq!(q.typed_prefix, "src/");
    }

    #[test]
    fn trailing_slash_lists_that_directory() {
        let q = parsed("src/components/");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/src/components"));
        assert_eq!(q.filter, "");
    }

    #[test]
    fn tilde_expands_to_home() {
        assert_eq!(parsed("~/Projects").base, home());
        assert_eq!(parsed("~/Projects").filter, "Projects");
        assert_eq!(parsed("~/").base, home());
        assert_eq!(resolve("~", &cwd(), Some(&home())), home());
    }

    #[test]
    fn absolute_paths_are_kept() {
        let q = parsed("/Users/exa");
        assert_eq!(q.base, PathBuf::from("/Users"));
        assert_eq!(q.filter, "exa");
    }

    #[test]
    fn parent_segments_are_resolved_logically() {
        assert_eq!(parsed("../").base, PathBuf::from("/Users/example"));
        assert_eq!(parsed("../../").base, home().parent().unwrap());
        assert_eq!(parsed("./").base, cwd());
    }

    #[test]
    fn dot_dot_is_a_filter_not_a_base() {
        let q = parsed("..");
        assert_eq!(q.base, cwd());
        assert_eq!(q.filter, "..");
        assert!(q.wants_hidden());
    }

    #[test]
    fn the_working_directory_follows_the_shell_when_it_agrees() {
        // /tmp is a symlink to /private/tmp on macOS, which is exactly the
        // case the logical path exists to preserve.
        let physical = std::fs::canonicalize("/tmp").unwrap();

        assert_eq!(
            prefer_logical(Some(PathBuf::from("/tmp")), physical.clone()),
            PathBuf::from("/tmp")
        );

        // A PWD naming somewhere else, or not naming a path at all, is ignored.
        assert_eq!(
            prefer_logical(Some(PathBuf::from("/usr")), physical.clone()),
            physical
        );
        assert_eq!(
            prefer_logical(Some(PathBuf::from("relative")), physical.clone()),
            physical
        );
        assert_eq!(prefer_logical(None, physical.clone()), physical);
    }

    #[test]
    fn normalize_does_not_escape_the_root() {
        assert_eq!(normalize(Path::new("/../..")), PathBuf::from("/"));
        assert_eq!(normalize(Path::new("/a/../b")), PathBuf::from("/b"));
    }

    #[test]
    fn quoting_is_removed_from_both_parts() {
        let q = parsed("'my dir'/comp");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/my dir"));
        assert_eq!(q.filter, "comp");

        let q = parsed(r"my\ dir/co mp");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/my dir"));
        assert_eq!(q.filter, "co mp");
    }

    #[test]
    fn unbalanced_quotes_are_tolerated_while_typing() {
        assert_eq!(unquote("'my di"), "my di");
        assert_eq!(unquote("\"my di"), "my di");
        assert_eq!(unquote("a\\"), "a");
    }

    #[test]
    fn double_quote_escapes_follow_shell_rules() {
        assert_eq!(unquote(r#""a\"b""#), "a\"b");
        assert_eq!(unquote(r#""a\nb""#), r"a\nb");
    }

    #[test]
    fn unicode_and_metacharacters_survive() {
        let q = parsed("プロ/コン");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/プロ"));
        assert_eq!(q.filter, "コン");

        let q = parsed("'a;b&c'/d");
        assert_eq!(q.base, PathBuf::from("/Users/example/app/a;b&c"));
    }
}
