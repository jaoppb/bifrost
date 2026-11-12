use std::fs;
use std::path::{Path, PathBuf};

use super::{Error, members_without_opt_in};

const OPTED_IN: &str = "[package]\nname = \"ok\"\n\n[lints]\nworkspace = true\n";

fn write(root: &Path, relative: &str, text: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn workspace(members: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path(),
        "Cargo.toml",
        &format!("[workspace]\nmembers = {members}\n"),
    );
    dir
}

#[test]
fn real_workspace_opts_in() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(
        members_without_opt_in(&root).unwrap(),
        Vec::<PathBuf>::new()
    );
}

#[test]
fn opted_in_member_passes() {
    let dir = workspace("[\"a\"]");
    write(dir.path(), "a/Cargo.toml", OPTED_IN);
    assert_eq!(
        members_without_opt_in(dir.path()).unwrap(),
        Vec::<PathBuf>::new()
    );
}

#[test]
fn member_without_lints_table_fails() {
    let dir = workspace("[\"a\"]");
    write(dir.path(), "a/Cargo.toml", "[package]\nname = \"a\"\n");
    let missing = members_without_opt_in(dir.path()).unwrap();
    assert_eq!(missing, vec![dir.path().join("a/Cargo.toml")]);
}

#[test]
fn member_with_workspace_false_fails() {
    let dir = workspace("[\"a\"]");
    write(dir.path(), "a/Cargo.toml", "[lints]\nworkspace = false\n");
    assert_eq!(members_without_opt_in(dir.path()).unwrap().len(), 1);
}

#[test]
fn member_with_own_lints_fails() {
    let dir = workspace("[\"a\"]");
    write(
        dir.path(),
        "a/Cargo.toml",
        "[lints.rust]\nunsafe_code = \"allow\"\n",
    );
    assert_eq!(members_without_opt_in(dir.path()).unwrap().len(), 1);
}

#[test]
fn glob_members_are_expanded() {
    let dir = workspace("[\"crates/*\"]");
    write(dir.path(), "crates/good/Cargo.toml", OPTED_IN);
    write(
        dir.path(),
        "crates/bad/Cargo.toml",
        "[package]\nname = \"bad\"\n",
    );
    let missing = members_without_opt_in(dir.path()).unwrap();
    assert_eq!(missing, vec![dir.path().join("crates/bad/Cargo.toml")]);
}

#[test]
fn unsupported_pattern_is_an_error() {
    let dir = workspace("[\"crates/**\"]");
    let error = members_without_opt_in(dir.path()).unwrap_err();
    assert!(matches!(error, Error::UnsupportedPattern(_)));
}

#[test]
fn missing_members_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "Cargo.toml", "[workspace]\n");
    let error = members_without_opt_in(dir.path()).unwrap_err();
    assert!(matches!(error, Error::NoMembers(_)));
}

#[test]
fn missing_member_manifest_is_an_error() {
    let dir = workspace("[\"ghost\"]");
    let error = members_without_opt_in(dir.path()).unwrap_err();
    assert!(matches!(error, Error::Read(..)));
}
