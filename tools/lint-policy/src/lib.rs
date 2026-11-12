//! Fails when a workspace member does not opt in to the workspace lints.
//!
//! A member opts in with `[lints] workspace = true` in its own `Cargo.toml`.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A problem that stops the check from reaching a verdict.
#[derive(Debug)]
pub enum Error {
    /// A manifest could not be read.
    Read(PathBuf, io::Error),
    /// A manifest is not valid TOML.
    Parse(PathBuf, toml::de::Error),
    /// The root manifest has no `[workspace] members` list of strings.
    NoMembers(PathBuf),
    /// A member pattern uses glob syntax other than a trailing `/*`.
    UnsupportedPattern(String),
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(path, source) => write!(out, "cannot read {}: {source}", path.display()),
            Self::Parse(path, source) => write!(out, "cannot parse {}: {source}", path.display()),
            Self::NoMembers(path) => write!(out, "{} has no [workspace] members", path.display()),
            Self::UnsupportedPattern(pattern) => write!(
                out,
                "unsupported member pattern `{pattern}`; use a path or `dir/*`"
            ),
        }
    }
}

impl std::error::Error for Error {}

fn read_manifest(path: &Path) -> Result<toml::Table, Error> {
    let text =
        fs::read_to_string(path).map_err(|source| Error::Read(path.to_path_buf(), source))?;
    text.parse()
        .map_err(|source| Error::Parse(path.to_path_buf(), source))
}

fn is_glob(text: &str) -> bool {
    text.contains(['*', '?', '['])
}

fn expand(root: &Path, pattern: &str) -> Result<Vec<PathBuf>, Error> {
    let (parent, wildcard) = pattern
        .strip_suffix("/*")
        .map_or((pattern, false), |parent| (parent, true));
    if is_glob(parent) {
        return Err(Error::UnsupportedPattern(pattern.to_owned()));
    }
    if wildcard {
        return children(&root.join(parent));
    }
    Ok(vec![root.join(parent)])
}

fn children(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let read = |source| Error::Read(dir.to_path_buf(), source);
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).map_err(read)? {
        found.push(entry.map_err(read)?.path());
    }
    found.retain(|path| path.join("Cargo.toml").is_file());
    found.sort();
    Ok(found)
}

fn members(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let manifest_path = root.join("Cargo.toml");
    let manifest = read_manifest(&manifest_path)?;
    let patterns = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or(Error::NoMembers(manifest_path))?;
    let mut dirs = Vec::new();
    for pattern in patterns.iter().filter_map(toml::Value::as_str) {
        dirs.extend(expand(root, pattern)?);
    }
    Ok(dirs)
}

fn opts_in(manifest: &toml::Table) -> bool {
    manifest
        .get("lints")
        .and_then(|lints| lints.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true)
}

/// Returns the manifests of the members of the workspace at `root` that do not
/// set `[lints] workspace = true`.
///
/// # Errors
///
/// Returns an [`Error`] when the root manifest or a member manifest cannot be
/// read or parsed, or when a member pattern is not supported.
pub fn members_without_opt_in(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut missing = Vec::new();
    for dir in members(root)? {
        let path = dir.join("Cargo.toml");
        let manifest = read_manifest(&path)?;
        missing.extend((!opts_in(&manifest)).then_some(path));
    }
    Ok(missing)
}

#[cfg(test)]
mod tests;
