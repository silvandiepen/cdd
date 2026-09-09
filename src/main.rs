use std::process::ExitCode;

fn main() -> ExitCode {
    cdd::cli::run(std::env::args_os().skip(1))
}
