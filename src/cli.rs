//! Command line surface.
//!
//! The commands are primitives for shell adapters rather than a user-facing
//! toolbox, so they stay few and stable. Argument parsing is hand-rolled to
//! keep the dependency graph small.

use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::picker::{self, Outcome};
use crate::serve;
use crate::session::{Request, Session, DEFAULT_MAX_ROWS, DEFAULT_WIDTH};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = "\
cdd — an interactive alternative to cd

USAGE:
    cdd [PATH]              Pick a directory, starting from PATH
    cdd init zsh            Print the zsh integration; eval it from ~/.zshrc
    cdd list [OPTIONS]      Print ranked directory matches (for shell adapters)
    cdd pick [PATH]         Run the fallback picker, printing the choice to stdout
    cdd serve               Serve list requests on stdin/stdout, keeping a cache

LIST OPTIONS:
    --cwd <PATH>            Directory the query is relative to (default: $PWD)
    --query <TEXT>          The text typed after `cdd`
    --width <N>             Terminal width in columns (default: 80)
    --rows <N>              Maximum visible rows (default: 8)

OPTIONS:
    -h, --help              Print this help
    -V, --version           Print the version

ENVIRONMENT:
    CDD_MAX_ROWS            Maximum visible rows (default: 8)

Install the live experience with:

    eval \"$(cdd init zsh)\"
";

/// Parse and dispatch. Returns the process exit code.
pub fn run(args: impl Iterator<Item = OsString>) -> ExitCode {
    let args: Vec<String> = args.map(|a| a.to_string_lossy().into_owned()).collect();

    match dispatch(&args) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("cdd: {message}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(args: &[String]) -> Result<ExitCode, String> {
    match args.first().map(String::as_str) {
        None => pick(""),
        Some("-h" | "--help") => {
            print!("{HELP}");
            Ok(ExitCode::SUCCESS)
        }
        Some("-V" | "--version") => {
            println!("cdd {VERSION}");
            Ok(ExitCode::SUCCESS)
        }
        Some("init") => init(args.get(1).map(String::as_str)),
        Some("list") => list(&args[1..]),
        Some("serve") => serve::run()
            .map(|()| ExitCode::SUCCESS)
            .map_err(|e| e.to_string()),
        Some("pick") => pick(&joined(&args[1..])),
        Some(other) if other.starts_with('-') => Err(format!("unknown option: {other}")),
        Some(_) => pick(&joined(args)),
    }
}

/// Re-join the remaining arguments the way they were typed. The shell has
/// already split and unquoted them, so a single space is the right separator.
fn joined(args: &[String]) -> String {
    args.join(" ")
}

fn init(shell: Option<&str>) -> Result<ExitCode, String> {
    match shell {
        Some("zsh") => {
            let script = include_str!("../shell/zsh.zsh");
            let binary = std::env::current_exe()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|_| "cdd".to_string());
            print!(
                "{}",
                script.replace("@CDD_BIN@", &crate::output::quote(&binary))
            );
            Ok(ExitCode::SUCCESS)
        }
        Some(other) => Err(format!(
            "no integration for {other} yet; supported shells: zsh"
        )),
        None => Err("usage: cdd init zsh".to_string()),
    }
}

fn list(args: &[String]) -> Result<ExitCode, String> {
    let mut request = Request {
        raw: String::new(),
        cwd: current_dir()?,
        home: std::env::var_os("HOME").map(PathBuf::from),
        width: DEFAULT_WIDTH,
        max_rows: max_rows(),
    };

    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        let mut value = || {
            rest.next()
                .cloned()
                .ok_or_else(|| format!("{arg} requires a value"))
        };
        match arg.as_str() {
            "--cwd" => request.cwd = PathBuf::from(value()?),
            "--query" => request.raw = value()?,
            "--width" => request.width = parse_number(&value()?, "--width")?,
            "--rows" => request.max_rows = parse_number(&value()?, "--rows")?,
            other => return Err(format!("unknown option: {other}")),
        }
    }

    let response = Session::new().resolve(&request);
    let mut stdout = std::io::stdout();
    stdout
        .write_all(response.encode().as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(ExitCode::SUCCESS)
}

fn parse_number(value: &str, flag: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|_| format!("{flag} expects a number, got {value}"))
}

/// Run the fallback picker and print the chosen path to stdout.
///
/// The path is zsh-quoted, exactly like the paths in a `list` response, so the
/// shell function decodes it with `${(Q)}` and a name containing a newline
/// still arrives as a single line.
fn pick(raw: &str) -> Result<ExitCode, String> {
    match picker::pick(current_dir()?, raw.to_string()).map_err(|e| e.to_string())? {
        Outcome::Chosen(path) => {
            println!("{}", crate::output::quote(&path.to_string_lossy()));
            Ok(ExitCode::SUCCESS)
        }
        Outcome::Cancelled => Ok(ExitCode::FAILURE),
    }
}

fn current_dir() -> Result<PathBuf, String> {
    crate::path::working_directory().map_err(|e| format!("cannot read the current directory: {e}"))
}

fn max_rows() -> usize {
    std::env::var("CDD_MAX_ROWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_MAX_ROWS)
}
