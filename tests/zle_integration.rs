//! Runs the interactive zsh acceptance suite.
//!
//! The defining behaviour of `cdd` happens while the command line is still
//! being edited, so it can only be verified by driving a real zsh line editor.
//! `tests/zle.zsh` does that over a pty; this test is the entry point that
//! makes `cargo test` cover it.

use std::path::PathBuf;
use std::process::Command;

#[test]
fn the_zsh_line_editor_integration_behaves_as_specified() {
    let scratch = std::env::temp_dir().join(format!("cdd-zle-it-{}", std::process::id()));

    let output = Command::new("zsh")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/zle.zsh"))
        .arg(env!("CARGO_BIN_EXE_cdd"))
        .arg(&scratch)
        .output()
        .expect("zsh is required to verify the shell integration");

    let report = String::from_utf8_lossy(&output.stdout);
    let errors = String::from_utf8_lossy(&output.stderr);
    let _ = std::fs::remove_dir_all(&scratch);

    assert!(
        output.status.success(),
        "interactive zsh acceptance suite failed\n{report}\n{errors}"
    );
    // Guard against the harness silently doing nothing.
    assert!(
        report.contains("FAIL=0") && !report.contains("PASS=0"),
        "the acceptance suite did not report results\n{report}\n{errors}"
    );
}
