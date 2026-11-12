//! Problems that stop the checks from reaching a verdict.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// A problem that stops the checks from reaching a verdict.
#[derive(Debug)]
pub enum Error {
    /// `arch-lint.toml` could not be read.
    Read(PathBuf, io::Error),
    /// `arch-lint.toml` is not usable: bad TOML or a missing boundary rule.
    Config(String),
    /// arch-lint rejected its configuration or failed while running.
    ArchLint(String),
    /// `cargo metadata` could not run or its output could not be understood.
    Metadata(String),
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(path, source) => write!(out, "cannot read {}: {source}", path.display()),
            Self::Config(reason) => write!(out, "invalid arch-lint.toml: {reason}"),
            Self::ArchLint(reason) => write!(out, "arch-lint failed: {reason}"),
            Self::Metadata(reason) => write!(out, "cargo metadata failed: {reason}"),
        }
    }
}

impl std::error::Error for Error {}
