# Bifrost agent rules

Bifrost is peer-to-peer clipboard sync, notification sharing and file transfer
across Linux, Android, macOS and Windows, over Tailscale, written in Rust. This
file is the canonical rulebook for humans and agents.
[CONTRIBUTING.md](CONTRIBUTING.md) is the shorter guide for a first change, and
[docs/workflows/issues.md](docs/workflows/issues.md) describes how the GitHub
Project is set up.

## 1. Purpose and how to use this file

- Section numbers are stable. New sections are appended and existing ones are
  never renumbered, because other documents and skills cite them by number.
- This file is canonical: other documents summarise a rule and link here.
  Every rule here names its enforcer or is marked `review-only`.
- Status values: `enforced now` (a check runs in `just check` or in a git hook
  today), `enforced by hand` (a command checks it, but it needs a login or
  network, so it is not part of `just check`) or `review-only` (no tool checks
  it, with the reason).
- Never claim a check that does not run. Where an enforcer is missing, the
  status says so.
- There are no lint exceptions: no `allow` or `expect` attributes and no
  arch-lint suppressions anywhere (the suppressions are review-only, section
  3). Code is redesigned to fit, or the rule is
  changed for everyone in a pull request that says why.

## 2. Architecture

The workspace has `crates/core` (`bifrost-core`, platform independent),
`crates/daemon` (`bifrostd`, built on the core) and two repository tools under
`tools/` (`bifrost-architecture` and `lint-policy`) that check the workspace and
are not part of the product. The final crate layout and the per-OS adapter
template are decided by spike #23, and #50 writes them into this section. Until
then these are the only architecture rules.

- `bifrost-core` is platform independent: it uses no platform crate (Windows,
  macOS, Android, Wayland and D-Bus bindings) and neither the daemon nor
  the tools.
- The tools check the workspace and never use the product crates.
- A new platform crate is added to the deny list in `arch-lint.toml` in the pull
  request that introduces it.

| Architecture rule | Enforcer | Status |
|-------------------|----------|--------|
| `bifrost-core` source uses no platform crate and neither the daemon nor the tools | arch-lint rule `core-stays-platform-independent`, run by `just arch` | enforced now |
| `bifrost-core` does not depend on those crates, directly or through another crate | `bifrost-architecture` reads the resolved Cargo graph from `cargo metadata` against the same deny list, run by `just arch` | enforced now |
| The tools do not use `bifrost-core` or `bifrostd` | arch-lint rule `tools-do-not-use-the-product`, run by `just arch` | enforced now for source; the Cargo graph is not checked |
| A new platform crate is added to the deny list | Code review | review-only: no tool knows which crates are platform crates |

## 3. Rust practices

The toolchain is pinned in `rust-toolchain.toml` and read by the flake, so every
environment builds with the same compiler. Lints are declared once in
`[workspace.lints]` in `Cargo.toml`, and each member opts in with
`[lints] workspace = true`.

| Practice | Enforcer | Status |
|----------|----------|--------|
| Every workspace member opts in to the workspace lints | `lint-policy` test `real_workspace_opts_in`, run by `just test` | enforced now |
| Clippy `all`, `pedantic`, `nursery` and `cargo` groups are denied, and warnings are errors | `just clippy` (`-D warnings`) | enforced now |
| No `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!` or `unreachable!`, no indexing or string slicing, no `as` conversions | clippy lints at `forbid` in `Cargo.toml`; `clippy.toml` allows only `unwrap`, `expect`, `panic`, indexing, printing and `dbg!` in tests, and the rest apply there too | enforced now |
| `unsafe` code is forbidden | rustc lint `unsafe_code` at `forbid` | enforced now |
| `///` docs on public items and module docs, `Debug` on public types, no needless `pub` | rustc lints `missing_docs`, `missing_debug_implementations` and `unreachable_pub` at `forbid` | enforced now |
| No printing or `dbg!` outside tests | clippy `print_stdout`, `print_stderr` and `dbg_macro` at `forbid` | enforced now |
| No floating point or unchecked integer arithmetic | clippy `float_arithmetic` and `arithmetic_side_effects` at `forbid` | enforced now |
| Functions of at most 25 lines and 3 parameters, cognitive complexity at most 8, no boolean flags in signatures or structs, identifiers of three or more characters | clippy `too_many_lines`, `too_many_arguments`, `cognitive_complexity`, `fn_params_excessive_bools`, `struct_excessive_bools` and `min_ident_chars`, with thresholds in `clippy.toml` | enforced now |
| No lint exceptions: no `allow` or `expect` attributes | clippy `allow_attributes` and `allow_attributes_without_reason` at `forbid` | enforced now for attributes; arch-lint suppressions are not scanned |
| Dependencies pass the policy: permissive licenses, no advisories or yanked versions, no wildcard versions, crates.io only | `just deny` (`cargo deny`), part of `just lint` | enforced now |
| Shared dependency versions live in `[workspace.dependencies]`, and a 0.x dependency that breaks between releases is pinned exactly | Code review of manifests | review-only: no manifest check exists |
| One error type per module boundary, never a leaked foreign error | Code review | review-only: mapping quality needs judgement |
| Never log secrets, tokens or clipboard, notification or file content | Code review | review-only: content cannot be linted |
| Code is formatted with `rustfmt` | `just fmt-check`, run by the pre-commit hook | enforced now |

## 4. Tests

Tests run with `cargo nextest`, and doc tests run with `cargo test --doc`. Both
are in `just test`.

- A test name says the behaviour, for example
  `a_use_of_a_platform_crate_in_core_fails`.
- A check that fails a build ships with a test that fails on a violation and
  passes on a clean input, as `bifrost-architecture` and `lint-policy` do, plus
  a test that runs it on the real workspace.

| Test rule | Enforcer | Status |
|-----------|----------|--------|
| The unit, integration and doc tests pass | `just test`, part of `just check` | enforced now |
| The architecture and lint-policy checks hold on the real workspace | tests `the_real_workspace_holds_its_boundaries` and `real_workspace_opts_in`, run by `just test` | enforced now |
| Behaviour-describing test names | Code review | review-only |
| A new check has a failing and a passing test | Code review | review-only: no tool knows which code is a check |

## 5. Workflow and documentation

- Every change is tied to one GitHub issue, and a branch finishes at most one.
- **Branches** are named `<type>/<issue>`, for example `docs/13`, where `<type>`
  is the word of the issue's `type:` label.
- **Commits** are conventional: `<type>: <summary> (#<issue>)`, with a
  conventional type such as `feat`, `fix`, `chore`, `docs` or `refactor`. The
  `convco` types have no `spike`, so a `type:spike` issue commits as `docs`.
  Mention the epic only as `Refs #<n>`, and never close an epic with a commit.
- **Pull requests** are titled like a commit and filled from
  `.github/pull_request_template.md`: `Closes #<issue>`, What, Notes and Test
  plan. A pull request that only says `Refs #<n>` is not linked to the issue, so
  its card is moved by hand.
- **Merging** creates a merge commit and nothing else: squash and rebase are
  off, the pull request title and body become the merge message, and the branch
  is deleted on merge. Every commit of the branch therefore reaches `main`, so
  each one follows the commit rule.
- **SPDD.** An issue with Effort 5 or more follows Structured Prompt-Driven
  Development before code is written. Its analysis is appended to the issue body
  and is the gate: no code before the analysis exists. Its canvas is a local
  working file under `spdd/`, which is gitignored, so no canvas is committed and
  no committed text or GitHub text names one by path (write `#<n>` in prose).
  Effort is read at triage, and the analysis may raise it. An issue below Effort
  5 has no SPDD artifacts, and spikes are exempt. The skills that implement
  these steps are installed by #14.
- **Documentation** is Markdown that passes `rumdl`: prose is at most 80 columns
  and `just fmt` wraps it. Tables, code blocks and links are not wrapped.
- Documentation describes what exists. A feature that is not built is marked
  planned, and a rule is never written before its check runs or before it is
  marked `review-only`.

| Workflow rule | Enforcer | Status |
|---------------|----------|--------|
| Commit titles are conventional commits | lefthook `commit-msg` hook runs `convco`; git-generated titles (merge, revert, fixup, squash, amend) are skipped | enforced now, locally |
| Every change is tied to one issue, and a branch finishes at most one | Code review | review-only: no check reads the issue of a change |
| The `(#<issue>)` suffix, the branch name and `Closes #` in the pull request | Code review | review-only: no check reads them |
| Code and Markdown are formatted and linted before a commit, and the full gate runs before a push | lefthook `pre-commit` (`just fmt-check`, `just clippy`) and `pre-push` (`just check`), installed by `just setup` | enforced now, locally; `--no-verify` skips them, and there is no CI yet |
| Merge commits only, the pull request title and body as the merge message, the branch deleted on merge | `[github.repository.settings]` in `tools.toml`, compared by `just repo-check` | enforced by hand |
| Markdown is formatted and linted | `rumdl` through `just fmt-check` | enforced now |
| An issue with Effort 5 or more has its analysis before code | Code review | review-only: the analysis lives in the issue body, which `just check` cannot reach |
| No canvas committed or named by path | `spdd/` in `.gitignore` for the files; code review for the text | enforced now for files; review-only for text |
| Documentation describes only what exists | Code review | review-only |

## 6. Commands

`just` is the only command surface. Hooks call these recipes and never `cargo`
directly. Recipes run only inside the flake dev shell (`nix develop`, or direnv
with `.envrc`) and fail with a clear message outside it.

| Recipe | Purpose |
|--------|---------|
| `just` | Show the available recipes |
| `just setup` | Install the git hooks (worktrees share one hooks directory) |
| `just fmt` | Format Rust and Markdown in place |
| `just fmt-check` | Fail if Rust or Markdown is not formatted |
| `just clippy` | Run clippy on the whole workspace, warnings as errors |
| `just lint` | Run clippy, then the dependency policy and the crate boundaries |
| `just arch` | Check the crate boundaries: arch-lint on the sources and the Cargo graph |
| `just deny` | Check advisories, licenses, bans and sources with `cargo deny`; the advisory database is fetched first and the cached copy is used when that fails |
| `just test` | Run the unit, integration and doc tests |
| `just check` | Run every check: format, lint and tests |
| `just issues-check` | Audit the open issues against the workflow of `tools.toml` (milestones, labels, blockers); needs `gh` logged in |
| `just repo-check` | Compare the repository settings, labels, milestones and label lists in the docs with `tools.toml`, and write nothing; needs `gh` logged in |
| `just repo-sync` | Make the repository settings, labels and milestones match `tools.toml`; writes to GitHub |
| `just project-check` | Print the planned Project Status moves and write nothing; exits 1 when there are any; needs the `project` scope |
| `just project-sync` | Write those moves to the Project |

## 7. Issue workflow

The workflow is declared in `tools.toml`
(`[workflow] extends = "builtin:cosmo"`) and the Project is set up as
[docs/workflows/issues.md](docs/workflows/issues.md) says.

- **Hierarchy.** Epic, story and task, as native GitHub sub-issues.
  - An epic is a closeable outcome that lives inside one milestone and carries
    the `epic` label.
  - A story is one issue with one pull request, and the SPDD unit when its
    Effort is 5 or more.
  - A task is a sub-issue of a story, only for a story split across several pull
    requests.
  - The sub-issue tree is the only record of the level: no label says story or
    task.
- **Milestones.** A milestone is a release (`v0.1.0` to `v0.5.0`), declared in
  `tools.toml`. Every open issue has exactly one.
- **Labels.** Exactly one `type:` label and at least one `area:` label per
  issue, and `epic` on epics. The list below is checked against `tools.toml`.
- **Creation.** File the issue from the form for its type, which applies the
  `type:` label. Then, at triage, set the `area:` label, the milestone and the
  parent, record each dependency as a native blocked-by link, and set Priority
  and Effort in the Project.
- **Priority.** P0 unblocks the critical path now. P1 is a must-have for its
  milestone. P2 is a should-have, off the critical path. P3 is low, whenever.
- **Effort.** Fibonacci points 1, 2, 3, 5, 8 and 13 for the issue's own work,
  not its sub-issues. Epics are not estimated. An issue above 8 is split before
  it can be Ready. Effort 5 or more is the gate for SPDD (section 5).
- **Statuses.** Status, Priority and Effort live only in the Project. Blocked is
  the native blocked-by link, never a label.

<!-- labels:start -->

- `epic` — A closeable outcome made of stories
- `type:feat` — New behaviour or capability
- `type:fix` — Something is broken
- `type:chore` — Tooling, CI, housekeeping, dependencies
- `type:docs` — Documentation, guides, specifications
- `type:refactor` — Restructure without changing behaviour
- `type:spike` — Time-boxed research that answers a question
- `area:tooling` — Flake, Nix, just, hooks, linters, Cargo workspace
- `area:ci` — GitHub Actions: checks, builds and releases
- `area:docs` — Rulebook, specification, ADRs and guides
- `area:core` — Domain, sync engine, rules engine and daemon
- `area:transport` — Tailscale integration, discovery, pairing and wire protocol
- `area:clipboard` — Clipboard sync and clipboard history integrations
- `area:notifications` — Notification capture, rules and mirroring
- `area:files` — File transfer between devices
- `area:linux` — Linux adapters and packaging (Hyprland first)
- `area:ui` — Settings UI, tray and command line client
- `area:android` — Android app, Kotlin shell and bindings
- `area:macos` — macOS adapters, bundle and distribution
- `area:windows` — Windows adapters, packaging and distribution

<!-- labels:end -->

| Status | Entry condition | Moved by |
|--------|-----------------|----------|
| Backlog | A new issue, a reopened issue that does not pass the Ready gate, or a Ready issue that stops passing it | `just project-sync`, the Project's built-in workflows, or the person triaging |
| Ready | Type, area, milestone, Priority and Effort (at most 8) are set and every blocker is closed. A task of an open story waits until the story has started | `just project-sync`, or the person triaging |
| In progress | The person or agent starts work (for Effort 5 or more, the analysis is appended to the issue body first) | That person or agent |
| In review | A pull request that closes the issue is open | `just project-sync`, the built-in workflow, or the person |
| Done | The issue is closed, completed or not planned | `just project-sync`, the built-in workflow, or the person |

`just project-sync` writes Status only, and never demotes an issue that is In
progress or In review for a new blocker.

| Issue workflow rule | Enforcer | Status |
|---------------------|----------|--------|
| The label list above and in CONTRIBUTING.md lists exactly the labels of `tools.toml`, and each issue form carries its label and title prefix | `just repo-check` | enforced by hand |
| Labels, milestones and merge settings on GitHub match `tools.toml` | `just repo-check`, and `just repo-sync` to apply | enforced by hand |
| Open issues have a milestone, one `type:` label, an `area:` label, a parent (epics excepted), a Project card and no blocker cycle | `just issues-check` | enforced by hand |
| The Ready gate and the Status of each card | `just project-check` and `just project-sync` | enforced by hand |
| Hierarchy levels, Priority meaning, dependency links and the SPDD gate | Code review | review-only: GitHub state cannot be reached from `just check`, and no CI exists yet |
