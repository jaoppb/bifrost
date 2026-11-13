# Contributing to Bifrost

This guide gets a first change from clone to pull request. The rules behind it,
and the tool that enforces each one, are in [AGENTS.md](AGENTS.md), which is
canonical when the two differ.

## Setup

Bifrost builds inside a Nix flake dev shell, so every machine, git hook and
later CI run uses the same toolchain.

- Install [Nix](https://nixos.org/) with flakes enabled.
- Optionally install [direnv](https://direnv.net/), so `.envrc` enters the dev
  shell for you (`direnv allow` once).
- Enter the shell with `nix develop`, then install the git hooks once:

```bash
just setup
```

To run one recipe without entering the shell, prefix it:
`nix develop -c just check`. Recipes fail with that hint when run outside the
shell.

`gh` is part of the dev shell. The issue and Project recipes need it logged in
(`gh auth login`), and `just project-check` also needs the `project` scope
(`gh auth refresh -s project`).

## The checks

`just` is the only command surface; `just` alone lists the recipes, and
AGENTS.md section 6 describes each one.

| Recipe | Runs |
|--------|------|
| `just fmt` | Formats Rust and Markdown in place |
| `just check` | Format check, clippy, the dependency policy, the crate boundaries and the tests |

`just check` must pass with zero lint exceptions before a pull request is
opened: there are no `allow` or `expect` attributes, and a lint that does not
fit the code means the code is redesigned.

The git hooks run the same recipes. `pre-commit` checks formatting and clippy
when you change Rust, Markdown, TOML or lock files, `commit-msg` checks the
commit title, and `pre-push` runs `just check`.

## Issues

Every change starts from an issue. File it from the form for its type under
`.github/ISSUE_TEMPLATE/`, which applies the `type:` label. Then, at triage:

- set an `area:` label and the release milestone,
- set the parent issue, and add a native blocked-by link for each dependency,
- set Priority and Effort in the Project.

An issue with Effort 5 or more needs its analysis in the issue body before code
is written (SPDD, AGENTS.md section 5). The labels are checked against
`tools.toml`:

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

Each issue has exactly one `type:` label and at least one `area:` label. Only
epics carry `epic`.

## Branches and commits

Name the branch `<type>/<issue>`, for example `docs/13` or `chore/12`.

Commit titles are conventional commits that end with the issue number:

```text
<type>: <summary> (#<issue>)
```

For example, `docs: write the contributing guide (#13)`. The `commit-msg` hook
checks the conventional format with `convco`, and the issue number is checked in
review. Refer to a parent issue or an epic as `Refs #<n>`, and never close an
epic with a commit.

## Pull requests

- Title the pull request like a commit: `<type>: <summary> (#<issue>)`.
- Fill the template: `Closes #<issue>`, What, Notes and Test plan.
- Run `just check` first. There is no CI yet, so the local run is the gate.

The repository merges with merge commits only, and the pull request title and
body become the merge message. That is why every commit on the branch follows
the commit rule: all of them reach `main`.

A pull request that only says `Refs #<n>` is not linked to the issue, so move
its card by hand.
