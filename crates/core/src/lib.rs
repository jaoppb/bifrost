//! Platform-independent core of Bifrost.
//!
//! Placeholder: the final crate layout is decided by the workspace layout spike.

/// Name of the product, shared by every binary and log line.
#[must_use]
pub const fn product_name() -> &'static str {
    "Bifrost"
}

#[cfg(test)]
mod tests {
    use super::product_name;

    #[test]
    fn product_name_is_bifrost() {
        assert_eq!(product_name(), "Bifrost");
    }
}
