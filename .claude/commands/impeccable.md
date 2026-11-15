---
name: /impeccable
id: impeccable
category: Development
description: "Use for any UI work in Bifrost: shape a screen, polish, audit, lay out, typeset, colorize or clarify the settings and pairing UI, the Linux tray (iced and ksni) or the Android app, and to initialise or update PRODUCT.md."
---

# Impeccable for Bifrost

Design guidance for Bifrost's UI, adapted from `pbakaus/impeccable` (see
`NOTICE.md`). Bifrost's UI is small: a settings and pairing UI and a tray on
the Linux desktop (iced and `ksni`, decided in ADR 0001), and an Android app in
Kotlin. There is no web app, so this skill has no browser tooling
and no engine binary.

Core principles:

- Finish the job. No hedging, no shortcuts: the deliverable is complete, except
  assets the person must provide.
- Every Bifrost surface is an **Operate** surface: the person is completing a
  task (pairing a device, checking what is synced, setting a rule). Scanability,
  consistency, native expectations and the real usage scene outrank expression.
  Brand lives in precise details. See
  `.agents/skills/impeccable/reference/operate.md`.
- Verify by looking at the rendered result at the sizes it ships at, and batch
  the checks: fix what one look shows before looking again.

## Setup

1. Read `PRODUCT.md` at the repository root. It is the authority on users,
   devices, surfaces and voice. If it is missing, run `init` first and do not
   invent product facts. Read `docs/SPEC.md` (the UI surfaces and the accepted
   limits) and `docs/adr/0001-process-model.md` when the work touches a surface.
2. Read the platform reference for the target:
   `.agents/skills/impeccable/reference/desktop.md` for the Linux settings UI
   and tray, `.agents/skills/impeccable/reference/android.md` for the Android
   app. Read both when the work spans them.
3. Load the request's playbook from the Commands table. Inspect the target and
   its neighbours before editing.
4. Read `.agents/skills/impeccable/reference/craft-floor.md` immediately before
   any UI edit, including small refinements. Do not load it for planning-only
   work.

## How to design

- **The brief wins.** Honor `PRODUCT.md` and the brief the person gave.
  Redirecting a clear brief toward your taste is failure.
- **Refinement preserves; redesign replaces.** Refinement keeps the incumbent
  look, behavior and copy, and everything outside scope. Ask before replacing
  factual copy or adding claims.
- **Thin clients.** The UI holds no state of its own: the daemon applies every
  change, and the command line client reaches the same actions. A screen shows
  what the daemon reports, including the daemon being unreachable.
- **No visual system yet.** `DESIGN.md` is not decided yet. Follow the
  platform's conventions (Material 3 on Android as a working assumption until
  `v0.3.0`, the iced theme and system preferences on the desktop) instead of
  inventing a visual world.
- **Never claim an unbuilt feature.** `docs/SPEC.md` marks what is planned; UI
  copy and mockups follow it.

## Commands

| Command | Description | Reference |
|---|---|---|
| `init` | Capture durable product context in `PRODUCT.md` | [init.md](../../.agents/skills/impeccable/reference/init.md) |
| `shape [feature]` | Plan UX and UI before writing code | [shape.md](../../.agents/skills/impeccable/reference/shape.md) |
| `audit [target]` | Technical quality checks: accessibility, performance, theming, conformance, adaptivity | [audit.md](../../.agents/skills/impeccable/reference/audit.md) |
| `polish [target]` | Final quality pass before shipping | [polish.md](../../.agents/skills/impeccable/reference/polish.md) |
| `quieter [target]` | Tone down loud or overstimulating UI | [quieter.md](../../.agents/skills/impeccable/reference/quieter.md) |
| `distill [target]` | Strip to essence, remove complexity | [distill.md](../../.agents/skills/impeccable/reference/distill.md) |
| `colorize [target]` | Add strategic color to a monochrome UI | [colorize.md](../../.agents/skills/impeccable/reference/colorize.md) |
| `typeset [target]` | Improve type hierarchy | [typeset.md](../../.agents/skills/impeccable/reference/typeset.md) |
| `layout [target]` | Fix spacing, rhythm and hierarchy | [layout.md](../../.agents/skills/impeccable/reference/layout.md) |
| `clarify [target]` | Improve UX copy, labels and error messages | [clarify.md](../../.agents/skills/impeccable/reference/clarify.md) |

Routing:

- **No argument:** list the commands above and ask what the person wants; never
  auto-run one.
- **Explicit or clearly implied command:** load its reference and follow it. Ask
  once if two commands fit.
- **Otherwise:** treat the request as general UI work, with the setup above.
  A missing `PRODUCT.md` routes through `init` before a new surface; a narrow
  refinement of existing code proceeds on the incumbent implementation, and
  offers `init` afterward.
