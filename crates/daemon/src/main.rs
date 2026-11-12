//! The Bifrost background daemon.
//!
//! Placeholder: it does nothing yet.

use std::process::ExitCode;

/// Name the daemon reports for itself.
fn daemon_name() -> String {
    format!("{}d", bifrost_core::product_name().to_lowercase())
}

fn main() -> ExitCode {
    if daemon_name().is_empty() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::daemon_name;

    #[test]
    fn daemon_name_is_bifrostd() {
        assert_eq!(daemon_name(), "bifrostd");
    }
}
