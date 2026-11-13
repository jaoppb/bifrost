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

# Run clippy, then check the dependency policy, the crate boundaries and the skill mirrors.
lint: clippy
    just deny
    just arch
    just skills-check

# Check the crate boundaries: arch-lint on the sources and a guard on the Cargo graph.
arch:
    cargo run -q -p bifrost-architecture -- .

# Check advisories, licenses, bans and sources. The advisory database is fetched first; if that fails
# (offline, flaky network) the check uses the cached copy, and fails if there is none.
deny:
    cargo deny fetch || echo 'warning: could not update the advisory database, using the cached copy' >&2
    cargo deny --workspace --offline check

# Generate the Claude command mirrors in .claude/commands from the skills in .agents/skills (the only place a skill is edited).
# The directory holds mirrors only: any other .md file in it is deleted.
sync-skills:
    mkdir -p .claude/commands
    find .claude/commands -name '*.md' -delete
    for dir in .agents/skills/*/; do \
        name=$(basename "$dir"); \
        just _skill-mirror "$name" > ".claude/commands/$name.md"; \
    done

# Fail if a mirror in .claude/commands is missing, stale or has no skill; run `just sync-skills` to fix it.
skills-check:
    #!/usr/bin/env bash
    set -euo pipefail
    shopt -s nullglob
    status=0
    for dir in .agents/skills/*/; do
        name=$(basename "$dir")
        if ! diff -q <(just _skill-mirror "$name") ".claude/commands/$name.md" > /dev/null 2>&1; then
            echo "stale or missing mirror: .claude/commands/$name.md" >&2
            status=1
        fi
    done
    for file in .claude/commands/*.md; do
        [ -d ".agents/skills/$(basename "$file" .md)" ] || { echo "mirror without a skill: $file" >&2; status=1; }
    done
    [ "$status" -eq 0 ] || { echo "run 'just sync-skills'; skills are edited only under .agents/skills" >&2; exit 1; }

# Print the command mirror of one skill: the skill with its frontmatter turned into a slash command.
_skill-mirror name:
    @awk -v name='{{name}}' 'NR == 1 { next } !done && /^name:/ { next } !done && /^description:/ { desc = $0; next } !done && /^---$/ { done = 1; print "---"; print "name: /" name; print "id: " name; print "category: Development"; print desc; print "---"; next } done { print }' '.agents/skills/{{name}}/SKILL.md'

# Run the unit, integration and doc tests.
test:
    cargo nextest run --workspace --all-features
    cargo test --workspace --all-features --doc

# Run every check: format, lint and tests.
check: fmt-check lint test

# Audit open GitHub issues against the workflow rules of tools.toml.
issues-check:
    tools issues check

# Audit the GitHub repository settings, labels and milestones against tools.toml (writes nothing).
repo-check:
    tools repo check

# Make the GitHub repository settings, labels and milestones match tools.toml (writes to GitHub).
repo-sync:
    tools repo sync

# Print the planned Project Status moves without writing (exits 1 when there are any).
project-check:
    tools project check

# Write the planned Project Status moves to the GitHub Project.
project-sync:
    tools project sync
