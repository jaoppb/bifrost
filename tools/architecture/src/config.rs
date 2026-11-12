//! Reads the crates `bifrost-core` must not reach out of `arch-lint.toml`.
//!
//! The deny list is written once, as source paths, in the `[[restrict-use]]`
//! rule named [`CORE_RULE`]. The Cargo-graph check derives the crate names
//! from it, so the two guards cannot drift apart.

use crate::error::Error;

/// Name of the `[[restrict-use]]` rule that guards `bifrost-core`.
pub const CORE_RULE: &str = "core-stays-platform-independent";

const GLOB_SUFFIX: &str = "::**";

/// Returns the crates denied to `bifrost-core`, as written in the source
/// paths: `objc2_foundation::**` gives `objc2_foundation`.
///
/// # Errors
///
/// Returns [`Error::Config`] when the file is not TOML, the core rule or its
/// `deny` list is missing, or an entry is not a `<crate>::**` glob with a plain crate name.
pub fn denied_crates(config: &str) -> Result<Vec<String>, Error> {
    let table: toml::Table = config
        .parse()
        .map_err(|source: toml::de::Error| Error::Config(source.to_string()))?;
    let rule = table
        .get("restrict-use")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_table)
        .find(|rule| rule.get("name").and_then(toml::Value::as_str) == Some(CORE_RULE))
        .ok_or_else(|| Error::Config(format!("no [[restrict-use]] rule named `{CORE_RULE}`")))?;
    let deny = rule
        .get("deny")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| Error::Config(format!("rule `{CORE_RULE}` has no `deny` list")))?;
    deny.iter().map(crate_pattern).collect()
}

fn crate_pattern(entry: &toml::Value) -> Result<String, Error> {
    let path = entry
        .as_str()
        .ok_or_else(|| Error::Config(format!("`deny` of `{CORE_RULE}` holds a non-string")))?;
    path.strip_suffix(GLOB_SUFFIX)
        .filter(|name| !name.contains(['*', ':']))
        .map(str::to_owned)
        .ok_or_else(|| {
            Error::Config(format!(
                "`{path}` must be a plain crate name followed by `{GLOB_SUFFIX}`"
            ))
        })
}

/// Tells whether a Cargo package name is one of the denied crates. Cargo
/// treats `-` and `_` in a name as the same character, so both sides are
/// compared with `-` written as `_`.
#[must_use]
pub fn is_denied(denied: &[String], package: &str) -> bool {
    let package = package.replace('-', "_");
    denied.iter().any(|name| name.replace('-', "_") == package)
}
