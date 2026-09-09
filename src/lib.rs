//! `cdd` — an interactive alternative to `cd`.
//!
//! The crate is split so that everything a shell adapter cannot do itself lives
//! here: path parsing, directory enumeration, ranking, terminal-safe
//! serialization and the fallback picker. Changing the working directory stays
//! with the shell, because a child process cannot move its parent.

pub mod cli;
pub mod filesystem;
pub mod matcher;
pub mod output;
pub mod path;
pub mod picker;
pub mod serve;
pub mod session;
