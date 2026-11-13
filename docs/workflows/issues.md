# Issue workflow: Project setup

This page describes how the Bifrost Project is set up and operated. The rules
are in [AGENTS.md](../../AGENTS.md) section 7. This page repeats a rule only
where it needs the Project's context. Current state, such as which issue has
which priority, lives in the Project and is not copied here.

The GitHub API can create a Project's fields and read its views and workflows,
but it cannot set a view's grouping, sorting or sums, and it cannot create or
change a built-in workflow. Those are set by hand in the Project, and this page
says what to set.

Last verified against the Project: 2026-11-13, by reading the fields, views and
workflows back through the API.

## The Project

The Project is [Bifrost](https://github.com/users/jaoppb/projects/5), number 5
in `tools.toml`. It is linked to `jaoppb/bifrost`, and every issue of the
repository is an item, open or closed.

| Field | Kind | Values | Meaning |
|-------|------|--------|---------|
| Status | Single select | Backlog, Ready, In progress, In review, Done | Where the issue is in its life. The entry conditions are in AGENTS.md section 7 |
| Priority | Single select | P0, P1, P2, P3 | Order of work inside a milestone |
| Effort | Number | 1, 2, 3, 5, 8, 13 | Fibonacci points for the issue's own work, excluding its sub-issues. Epics have none |
| Milestone | Built in | `v0.1.0` to `v0.5.0` | The release |
| Parent issue | Built in | An epic or a story | The place in the tree |
| Sub-issues progress | Built in | Closed over total | Progress of an epic or a story |

The Status options must equal the `statuses` of the workflow in `tools.toml`, in
the same words, or `tools project` refuses to run.

Effort is also the gate for SPDD. An issue with Effort 5 or more gets its
analysis appended to the issue body before its card moves to In progress, so
Ready stays a check on fields only. The rule is in AGENTS.md section 5.

## Views

The views to set up:

| View | Layout | Filter | Use |
|------|--------|--------|-----|
| Board | Board, columns by Status | `-label:epic` | The work at a glance |
| Backlog | Table | `-label:epic is:open` | Everything open, to triage and to plan |
| Ready queue | Table | `status:Ready -label:epic` | What can be started now |
| Untriaged | Table | `no:priority -label:epic is:open` | Issues that still need a Priority |
| Epics | Table | `label:epic` | Outcomes, with sub-issue progress |

Set these by hand, because the API cannot:

- Backlog: group by Milestone, sort by Priority ascending, and show the Effort
  sum in the group headers.
- Ready queue: sort by Priority ascending.
- Board: nothing, the columns are already by Status.

A change to a view is a draft until you save it from the tab's menu
(**Save changes**).

Current state: the Project has only its default table view, named "View 1", with
no filter, grouping or sorting. The views above are not created yet.

## Built-in workflows

The API can read which workflows are on but cannot change them, so they are
enabled by hand on the Project's Workflows page.

| Workflow | Effect | Needed | State |
|----------|--------|--------|-------|
| Item added to project | Status becomes Backlog | Yes | Off |
| Item closed | Status becomes Done, including an issue closed as not planned | Yes | Off |
| Auto-add sub-issues to project | New sub-issues of items already in the Project are added | Yes | On |
| Pull request linked to issue | Status becomes In review. It fires for a closing keyword such as `Closes #n`; a `Refs #n` pull request is not linked | Optional | Off |
| Pull request merged | Status becomes Done | Optional: closing the issue already does it | Off |
| Auto-close issue | Closes an issue when its card reaches Done | No: leave it off | Off |

No built-in workflow moves a card from Backlog to Ready or back.
`just project-sync` does, by the rules of the workflow in `tools.toml`:

- `just project-check` prints every planned move and writes nothing. Run it
  first, and review the moves.
- `just project-sync` writes the moves. A second run proposes none.
- Both need `gh` logged in with the `project` scope.
- It moves Status only, and never demotes an issue that is In progress or In
  review.
- The Ready gate has no condition on the milestone, so a triaged and unblocked
  issue of a later release is Ready as well.

## Labels

| Family | Labels | Rule |
|--------|--------|------|
| type | `type:feat`, `type:fix`, `type:chore`, `type:docs`, `type:refactor`, `type:spike` | Exactly one per issue. It comes from the issue form chosen |
| area | `area:tooling`, `area:ci`, `area:docs`, `area:core`, `area:transport`, `area:clipboard`, `area:notifications`, `area:files`, `area:linux`, `area:ui`, `area:android`, `area:macos`, `area:windows` | At least one. Set at triage |
| level | `epic` | Epics only |

The full list, with descriptions, is the label block in AGENTS.md and
CONTRIBUTING.md, checked against `tools.toml` by `just repo-check`. The default
GitHub labels (`bug`, `enhancement` and the like) are listed as deprecated in
`tools.toml` and removed by `just repo-sync`.

## Milestones

Each milestone is a release, declared with its scope in `tools.toml` under
`[github.milestones]`; `just repo-check` compares them with GitHub, so they are
not listed here.

## Merging

A pull request lands as a merge commit and nothing else. The settings are in
`tools.toml` under `[github.repository.settings]`, and `just repo-sync` applies
them:

- Merge commit on, squash and rebase off.
- Merge commit title is the pull request title, and its message is the pull
  request body.
- The branch is deleted on merge.

Every commit of the branch reaches `main`, so a commit says `Refs #n` for a
parent issue and `Closes #n` only for the issue the branch finishes.

## Keeping this page true

The structure drifts without a check, and none of these checks runs in
`just check`, because they need a login and network. When the Project changes,
change this page and the date above. Run these by hand after reorganising issues
and before a release:

- `just repo-check` for the settings, labels and milestones, and the label lists
  in the docs.
- `just issues-check` for the issue structure. It prints one line per violation.
- `just project-check` for the Status of each card.
