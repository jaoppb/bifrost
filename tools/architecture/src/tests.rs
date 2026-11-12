use std::fs;
use std::path::Path;

use crate::config::{denied_crates, is_denied};
use crate::{CONFIG_FILE, Error, check};

const REAL_CONFIG: &str = include_str!("../../../arch-lint.toml");

fn write(root: &Path, relative: &str, text: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn manifest(name: &str, dependencies: &str) -> String {
    format!(
        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{dependencies}"
    )
}

/// A workspace with `bifrost-core` and the real `arch-lint.toml`.
fn workspace(core_dependencies: &str, core_source: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(root, CONFIG_FILE, REAL_CONFIG);
    write(
        root,
        "Cargo.toml",
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/core\"]\n",
    );
    write(
        root,
        "crates/core/Cargo.toml",
        &manifest("bifrost-core", core_dependencies),
    );
    write(root, "crates/core/src/lib.rs", core_source);
    dir
}

fn add_crate(root: &Path, name: &str, dependencies: &str) {
    write(
        root,
        &format!("deps/{name}/Cargo.toml"),
        &manifest(name, dependencies),
    );
    write(root, &format!("deps/{name}/src/lib.rs"), "");
}

#[test]
fn the_real_workspace_holds_its_boundaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(check(&root).unwrap(), Vec::<String>::new());
}

#[test]
fn a_clean_core_passes() {
    let dir = workspace("", "pub fn answer() -> u8 { 42 }\n");
    assert_eq!(check(dir.path()).unwrap(), Vec::<String>::new());
}

#[test]
fn a_use_of_a_platform_crate_in_core_fails() {
    let dir = workspace("", "use windows::Win32::Foundation::HWND;\n");
    let violations = check(dir.path()).unwrap();
    assert_eq!(violations.len(), 1, "{violations:?}");
    assert!(violations[0].starts_with("crates/core/src/lib.rs:1: "));
}

#[test]
fn an_inline_path_to_a_platform_crate_in_core_fails() {
    let dir = workspace(
        "",
        "pub fn probe() { zbus::connection::Builder::session(); }\n",
    );
    assert_eq!(check(dir.path()).unwrap().len(), 1);
}

#[test]
fn a_use_of_the_daemon_in_core_fails() {
    let dir = workspace("", "use bifrostd::run;\n");
    assert_eq!(check(dir.path()).unwrap().len(), 1);
}

#[test]
fn a_tool_using_the_product_fails() {
    let dir = workspace("", "");
    write(
        dir.path(),
        "tools/probe/src/lib.rs",
        "use bifrost_core::product_name;\n",
    );
    write(dir.path(), "tools/probe/Cargo.toml", &manifest("probe", ""));
    assert_eq!(check(dir.path()).unwrap().len(), 1);
}

#[test]
fn a_platform_crate_as_a_dependency_of_core_fails() {
    let dir = workspace("windows = { path = \"../../deps/windows\" }\n", "");
    add_crate(dir.path(), "windows", "");
    let violations = check(dir.path()).unwrap();
    assert_eq!(violations.len(), 1, "{violations:?}");
    assert!(violations[0].contains("bifrost-core must not depend on `windows`"));
}

#[test]
fn a_platform_crate_reached_through_another_crate_fails() {
    let dir = workspace("helper = { path = \"../../deps/helper\" }\n", "");
    add_crate(
        dir.path(),
        "helper",
        "objc2-foundation = { path = \"../objc2-foundation\" }\n",
    );
    add_crate(dir.path(), "objc2-foundation", "");
    let violations = check(dir.path()).unwrap();
    assert_eq!(violations.len(), 1, "{violations:?}");
    assert!(violations[0].ends_with("(bifrost-core -> helper -> objc2-foundation)"));
}

#[test]
fn a_platform_crate_outside_the_core_closure_passes() {
    let dir = workspace("", "");
    add_crate(dir.path(), "windows", "");
    assert_eq!(check(dir.path()).unwrap(), Vec::<String>::new());
}

#[test]
fn a_workspace_without_core_cannot_be_checked() {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), CONFIG_FILE, REAL_CONFIG);
    write(
        dir.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"deps/other\"]\n",
    );
    add_crate(dir.path(), "other", "");
    assert!(matches!(check(dir.path()), Err(Error::Metadata(_))));
}

#[test]
fn a_missing_config_cannot_be_checked() {
    let dir = tempfile::tempdir().unwrap();
    assert!(matches!(check(dir.path()), Err(Error::Read(..))));
}

#[test]
fn the_deny_list_becomes_cargo_names() {
    let denied = denied_crates(REAL_CONFIG).unwrap();
    assert!(denied.contains(&"windows-sys".to_owned()));
    assert!(denied.contains(&"objc2-*".to_owned()));
    assert!(is_denied(&denied, "objc2-app-kit"));
    assert!(is_denied(&denied, "windows"));
    assert!(!is_denied(&denied, "windows-future"));
    assert!(!is_denied(&denied, "serde"));
}

#[test]
fn a_config_without_the_core_rule_is_rejected() {
    assert!(matches!(
        denied_crates("preset = \"minimal\"\n"),
        Err(Error::Config(_))
    ));
}

#[test]
fn a_deny_entry_that_is_not_a_crate_glob_is_rejected() {
    let config =
        "[[restrict-use]]\nname = \"core-stays-platform-independent\"\ndeny = [\"windows\"]\n";
    assert!(matches!(denied_crates(config), Err(Error::Config(_))));
}
