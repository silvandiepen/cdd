//! Directory enumeration.
//!
//! Only the one directory the current path segment points at is ever read.
//! There is no recursive scan, and symlink targets are checked but never
//! walked.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// One navigable child directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Display and match name. Lossy only for names that are not valid UTF-8,
    /// which cannot happen on APFS but is handled rather than rejected.
    pub name: String,
    /// The native path used for navigation. Never derived from `name`.
    pub path: PathBuf,
}

/// The children of a directory, plus the stamp used to detect staleness.
#[derive(Debug, Clone)]
pub struct Listing {
    pub base: PathBuf,
    pub entries: Vec<Entry>,
    stamp: Option<SystemTime>,
}

impl Listing {
    /// True when the directory has not been modified since it was read.
    fn still_fresh(&self, base: &Path) -> bool {
        self.base == base && self.stamp == modified_at(base)
    }
}

fn modified_at(base: &Path) -> Option<SystemTime> {
    fs::metadata(base).ok()?.modified().ok()
}

/// Read the directories directly inside `base`.
///
/// A symlink is included when it resolves to a directory; the target itself is
/// not scanned. Entries that cannot be stat'd are skipped rather than failing
/// the whole listing, because an unreadable child does not stop the parent from
/// being browsable.
pub fn list(base: &Path) -> io::Result<Listing> {
    let stamp = modified_at(base);
    let mut entries = Vec::new();

    for entry in fs::read_dir(base)? {
        let Ok(entry) = entry else { continue };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        let is_dir = if file_type.is_symlink() {
            // Follows exactly one link, which is what makes a symlinked
            // directory navigable without scanning its tree.
            fs::metadata(entry.path())
                .map(|m| m.is_dir())
                .unwrap_or(false)
        } else {
            file_type.is_dir()
        };

        if !is_dir {
            continue;
        }

        entries.push(Entry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
        });
    }

    Ok(Listing {
        base: base.to_path_buf(),
        entries,
        stamp,
    })
}

/// Holds the listing of the directory currently being browsed.
///
/// Typing more filter characters never re-reads the filesystem; only moving to
/// a different base directory, or that directory changing underneath us, does.
#[derive(Debug, Default)]
pub struct Cache {
    current: Option<Listing>,
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the listing for `base`, reusing the cached one when it is still valid.
    pub fn get(&mut self, base: &Path) -> io::Result<&Listing> {
        let reusable = self.current.as_ref().is_some_and(|l| l.still_fresh(base));
        if !reusable {
            self.current = Some(list(base)?);
        }
        Ok(self.current.as_ref().expect("just populated"))
    }

    /// Drop the cached listing.
    pub fn clear(&mut self) {
        self.current = None;
    }
}

/// A short, actionable description of why a directory could not be read.
pub fn describe(error: &io::Error) -> String {
    match error.kind() {
        io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
        io::ErrorKind::NotFound => "No such directory".to_string(),
        _ => {
            // `NotADirectory` is not stable across the Rust versions we target.
            let raw = error.raw_os_error();
            if raw == Some(20) {
                "Not a directory".to_string()
            } else {
                error.to_string()
            }
        }
    }
}

/// Whether a path is a directory we could change into.
pub fn is_directory(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs as unix_fs;

    /// A throwaway directory tree that removes itself.
    struct Temp(PathBuf);

    impl Temp {
        fn new(tag: &str) -> Self {
            let base = std::env::temp_dir().join(format!(
                "cdd-test-{tag}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&base);
            fs::create_dir_all(&base).unwrap();
            Temp(base)
        }

        fn dir(&self, name: &str) -> PathBuf {
            let p = self.0.join(name);
            fs::create_dir_all(&p).unwrap();
            p
        }

        fn file(&self, name: &str) -> PathBuf {
            let p = self.0.join(name);
            fs::write(&p, b"x").unwrap();
            p
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn names(listing: &Listing) -> Vec<String> {
        let mut n: Vec<String> = listing.entries.iter().map(|e| e.name.clone()).collect();
        n.sort();
        n
    }

    #[test]
    fn lists_directories_and_ignores_files() {
        let t = Temp::new("dirs");
        t.dir("alpha");
        t.dir("beta");
        t.file("gamma.txt");

        assert_eq!(names(&list(&t.0).unwrap()), vec!["alpha", "beta"]);
    }

    #[test]
    fn includes_hidden_directories_so_the_caller_can_filter_them() {
        let t = Temp::new("hidden");
        t.dir(".git");
        t.dir("src");

        assert_eq!(names(&list(&t.0).unwrap()), vec![".git", "src"]);
    }

    #[test]
    fn a_symlink_to_a_directory_is_navigable() {
        let t = Temp::new("symlink");
        let target = t.dir("real");
        let file = t.file("file.txt");
        unix_fs::symlink(&target, t.0.join("link")).unwrap();
        unix_fs::symlink(&file, t.0.join("filelink")).unwrap();
        unix_fs::symlink(t.0.join("missing"), t.0.join("broken")).unwrap();

        assert_eq!(names(&list(&t.0).unwrap()), vec!["link", "real"]);
    }

    #[test]
    fn unicode_and_spaced_names_round_trip() {
        let t = Temp::new("unicode");
        t.dir("プロジェクト");
        t.dir("my dir");
        t.dir("emoji 📁");

        let listing = list(&t.0).unwrap();
        let found = names(&listing);
        assert!(found.contains(&"プロジェクト".to_string()));
        assert!(found.contains(&"my dir".to_string()));
        assert!(found.contains(&"emoji 📁".to_string()));

        for entry in &listing.entries {
            assert!(
                entry.path.is_dir(),
                "{:?} should be usable as a path",
                entry.path
            );
        }
    }

    #[test]
    fn an_unreadable_directory_reports_a_short_reason() {
        let t = Temp::new("perm");
        let locked = t.dir("locked");
        let mut perms = fs::metadata(&locked).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o000);
        fs::set_permissions(&locked, perms.clone()).unwrap();

        let error = list(&locked).unwrap_err();
        assert_eq!(describe(&error), "Permission denied");

        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
        fs::set_permissions(&locked, perms).unwrap();
    }

    #[test]
    fn a_missing_directory_reports_a_short_reason() {
        let t = Temp::new("missing");
        let error = list(&t.0.join("nope")).unwrap_err();
        assert_eq!(describe(&error), "No such directory");
    }

    #[test]
    fn the_cache_survives_filter_changes_and_notices_new_directories() {
        let t = Temp::new("cache");
        t.dir("one");
        let mut cache = Cache::new();

        let first = cache.get(&t.0).unwrap().entries.len();
        assert_eq!(first, 1);

        // Nothing changed on disk, so the cached listing is reused.
        assert_eq!(cache.get(&t.0).unwrap().entries.len(), 1);

        // mtime has a coarse resolution on some filesystems; make the change
        // unambiguous before asserting the cache refreshes.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        t.dir("two");
        assert_eq!(cache.get(&t.0).unwrap().entries.len(), 2);
    }
}
