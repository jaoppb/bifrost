//! Runs arch-lint on the source files with the rules of `arch-lint.toml`.

use std::path::Path;

use arch_lint::Session;

use crate::error::Error;

/// Returns one line per arch-lint violation found under `root`.
///
/// # Errors
///
/// Returns [`Error::ArchLint`] when arch-lint rejects the configuration or
/// fails while running.
pub fn findings(root: &Path, config: &str) -> Result<Vec<String>, Error> {
    let session = Session::from_config_str(root, config)
        .map_err(|source| Error::ArchLint(source.to_string()))?;
    let result = session
        .run()
        .map_err(|source| Error::ArchLint(source.to_string()))?;
    Ok(result
        .violations
        .iter()
        .map(|found| {
            format!(
                "{}:{}: {} {}",
                found.location.file.display(),
                found.location.line,
                found.code,
                found.message
            )
        })
        .collect())
}
