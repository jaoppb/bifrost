//! Command-line entry point: `bifrost-architecture [root]`.
//!
//! Exits with 0 when the boundaries hold, 1 when a rule is violated and 2
//! when the checks cannot run.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let root = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    match bifrost_architecture::check(&root) {
        Ok(violations) if violations.is_empty() => ExitCode::SUCCESS,
        Ok(violations) => report(&violations.join("\n"), 1),
        Err(error) => report(&error.to_string(), 2),
    }
}

fn report(text: &str, code: u8) -> ExitCode {
    let mut stderr = std::io::stderr().lock();
    let written = stderr
        .write_all(text.as_bytes())
        .and_then(|()| stderr.write_all(b"\n"));
    written.map_or_else(|_| ExitCode::from(code), |()| ExitCode::from(code))
}
