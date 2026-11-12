//! Architecture checks for the Bifrost workspace.
//!
//! Two guards protect the crate boundaries. arch-lint reads the source files
//! and applies the rules of `arch-lint.toml`. A second check reads the Cargo
//! dependency graph and applies the same deny list to what `bifrost-core`
//! depends on, because the source rules have gaps (glob imports, re-exports,
//! macros, crates that are only dependencies).

pub mod config;
pub mod error;
pub mod graph;
pub mod source;

#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;

pub use error::Error;

/// The arch-lint configuration, relative to the workspace root.
pub const CONFIG_FILE: &str = "arch-lint.toml";

/// Runs both guards on the workspace rooted at `root`.
///
/// Returns one line per violation; an empty list means the boundaries hold.
///
/// # Errors
///
/// Returns [`Error`] when the configuration cannot be read, when arch-lint
/// fails, or when `cargo metadata` cannot describe the workspace.
pub fn check(root: &Path) -> Result<Vec<String>, Error> {
    let path = root.join(CONFIG_FILE);
    let config = fs::read_to_string(&path).map_err(|source| Error::Read(path, source))?;
    let denied = config::denied_crates(&config)?;
    let mut violations = source::findings(root, &config)?;
    violations.extend(graph::findings(root, &denied)?);
    Ok(violations)
}
