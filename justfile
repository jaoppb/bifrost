set shell := ["bash", "-euo", "pipefail", "-c"]

# Recipes run only inside the flake dev shell (`nix develop`, or direnv with `.envrc`), so local runs,
# hooks and CI use the same toolchain. `eager` makes just evaluate this guard before any recipe, so it
# covers every recipe, new ones included; `just --list` evaluates nothing and works anywhere.
# flake.nix sets BIFROST_DEVSHELL.
eager _devshell := if env("BIFROST_DEVSHELL", "") == "" { error("run just inside the flake dev shell: 'nix develop -c just <recipe>', or enter it with 'nix develop' or 'direnv allow'") } else { "" }

# Show the available recipes.
default:
    @just --list

# Install the git hooks (worktrees share one hooks directory, so one run covers them all).
setup:
    lefthook install

# Format Rust and Markdown in place.
fmt:
    cargo fmt --all
    rumdl fmt .

# Fail if Rust or Markdown is not formatted.
fmt-check:
    cargo fmt --all --check
    rumdl check .

# Run clippy on the whole workspace (warnings are errors).
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run clippy, then check the dependency policy and the crate boundaries.
lint: clippy
    just deny
    just arch

# Check the crate boundaries: arch-lint on the sources and a guard on the Cargo graph.
arch:
    cargo run -q -p bifrost-architecture -- .

# Check advisories, licenses, bans and sources. The advisory database is fetched first; if that fails
# (offline, flaky network) the check uses the cached copy, and fails if there is none.
deny:
    cargo deny fetch || echo 'warning: could not update the advisory database, using the cached copy' >&2
    cargo deny --workspace --offline check

# Run the unit, integration and doc tests.
test:
    cargo nextest run --workspace --all-features
    cargo test --workspace --all-features --doc

# Run every check: format, lint and tests.
check: fmt-check lint test
