//! The binary's command surface: the primitives shell adapters depend on.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Tree(PathBuf);

impl Tree {
    fn new(tag: &str) -> Self {
        let base = std::env::temp_dir().join(format!("cdd-cli-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        Tree(fs::canonicalize(&base).unwrap())
    }

    fn dirs(&self, names: &[&str]) -> &Self {
        for name in names {
            fs::create_dir_all(self.0.join(name)).unwrap();
        }
        self
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn cdd(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cdd"))
        .args(args)
        .output()
        .expect("failed to run cdd")
}

fn list(cwd: &Path, query: &str) -> String {
    let output = cdd(&[
        "list",
        "--cwd",
        cwd.to_str().unwrap(),
        "--query",
        query,
        "--width",
        "80",
    ]);
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn version_and_help_are_available() {
    let version = cdd(&["--version"]);
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).starts_with("cdd "));

    let help = cdd(&["--help"]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("cdd init zsh"));
}

#[test]
fn an_unknown_option_fails_loudly() {
    let output = cdd(&["--nope"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown option"));
}

#[test]
fn init_emits_a_zsh_script_with_the_binary_path_baked_in() {
    let output = cdd(&["init", "zsh"]);
    assert!(output.status.success());

    let script = String::from_utf8(output.stdout).unwrap();
    assert!(script.contains("CDD_BIN="));
    assert!(
        !script.contains("@CDD_BIN@"),
        "the placeholder was not replaced"
    );
    assert!(script.contains(env!("CARGO_BIN_EXE_cdd")));
    assert!(script.contains("zle -N _cdd_accept_widget"));

    // The emitted script has to be valid zsh.
    let checked = Command::new("zsh")
        .arg("-n")
        .arg("-c")
        .arg(&script)
        .output()
        .expect("zsh is required to check the emitted script");
    assert!(
        checked.status.success(),
        "cdd init zsh emitted a script zsh cannot parse: {}",
        String::from_utf8_lossy(&checked.stderr)
    );
}

#[test]
fn init_rejects_a_shell_it_cannot_integrate_with() {
    let output = cdd(&["init", "fish"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("zsh"));
}

#[test]
fn list_answers_in_the_documented_wire_format() {
    let tree = Tree::new("wire");
    tree.dirs(&["components", "composables", "config"]);

    let response = list(&tree.0, "comp");
    let lines: Vec<&str> = response.lines().collect();

    assert_eq!(lines[0], "CDD\t1");
    assert!(lines[1].starts_with("BASE\t"));
    assert_eq!(lines.last(), Some(&"END"));

    let rows: Vec<&&str> = lines.iter().filter(|l| l.starts_with("R\t")).collect();
    assert_eq!(rows.len(), 2);
    assert!(rows[0].ends_with("\tcomponents"));
    assert!(rows[1].ends_with("\tcomposables"));
}

#[test]
fn every_record_stays_on_one_line_however_a_directory_is_named() {
    let tree = Tree::new("hostile");
    tree.dirs(&[
        "odd\nname",
        "tab\tname",
        "quote'name",
        "spaced name",
        "プロジェクト",
    ]);

    let response = list(&tree.0, "");
    let rows: Vec<&str> = response.lines().filter(|l| l.starts_with("R\t")).collect();
    assert_eq!(rows.len(), 5, "one line per directory: {response:?}");

    // The shell decodes these with ${(Q)}; check the encoding it will be given.
    assert!(
        response.contains("$'"),
        "control characters must be encoded"
    );
    assert!(response.contains("'spaced name'"));
    assert!(response.contains("プロジェクト"));
}

#[test]
fn list_reports_an_unreadable_directory_instead_of_failing() {
    let tree = Tree::new("perm");
    tree.dirs(&["locked"]);
    let locked = tree.0.join("locked");

    let mut perms = fs::metadata(&locked).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o000);
    fs::set_permissions(&locked, perms.clone()).unwrap();

    let response = list(&tree.0, "locked/");

    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&locked, perms).unwrap();

    assert!(response.contains("ERR\tPermission denied"), "{response:?}");
    assert!(
        !response.lines().any(|l| l.starts_with("R\t")),
        "{response:?}"
    );
}

#[test]
fn serve_answers_repeated_requests_on_one_process() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::Stdio;

    let tree = Tree::new("serve");
    tree.dirs(&["components", "composables", "config"]);

    let mut child = Command::new(env!("CARGO_BIN_EXE_cdd"))
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start cdd serve");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let cwd = tree.0.to_str().unwrap();

    let mut ask = |query: &str| {
        writeln!(stdin, "LIST\t{cwd}\t{query}\t80\t8").unwrap();
        stdin.flush().unwrap();

        let mut records = Vec::new();
        loop {
            let mut line = String::new();
            stdout.read_line(&mut line).unwrap();
            let line = line.trim_end().to_string();
            if line == "END" {
                return records;
            }
            records.push(line);
        }
    };

    let broad = ask("co");
    let narrow = ask("comp");
    assert_eq!(broad.iter().filter(|l| l.starts_with("R\t")).count(), 3);
    assert_eq!(narrow.iter().filter(|l| l.starts_with("R\t")).count(), 2);

    writeln!(stdin, "QUIT").unwrap();
    drop(stdin);
    assert!(child.wait().unwrap().success());
}
