---
name: spdd-generate
description: "Generate Bifrost code and tests from a REASONS Canvas structured prompt, executing its Operations in order under its Norms and Safeguards. Use when a canvas from spdd-reasons-canvas is confirmed and ready to implement, or to regenerate the code affected by a canvas update. Do not use without a canvas (use spdd-reasons-canvas), and do not use it to patch code that diverged from the canvas (update the canvas first with spdd-prompt-update)."
---

# SPDD Generate

Generate implementation code from a structured SPDD prompt (a REASONS Canvas).
Follow its Operations sequence, Norms and Safeguards strictly, within Bifrost's
rules (`AGENTS.md`).

**Input**: the canvas path, e.g.
`@spdd/prompt/GH-<n>-<timestamp>-[Feat]-<scope>-<description>.md`.

## Steps

1. **Get the canvas.** If no path was given, ask the user for one. Do NOT
   proceed without a valid canvas.

2. **Resolve the work item (REQUIRED)**
   - Take it from the `GH-<n>-` file-name prefix (or the header line). If the
     canvas has none, stop: fix the canvas first with `/spdd-prompt-update`.
   - Verify it with `gh issue view <n> --json number,title,state`. Not found →
     stop. Closed → warn and ask.
   - The canvas is an ephemeral working file: never stage it, commit it or name
     it by path in a commit, issue or pull request (write `#<n>`). If it does
     not exist, stop and run `/spdd-reasons-canvas #<n>`, which regenerates it
     from the analysis and the code already written.

3. **Read the whole canvas** and extract its sections:

   | Section | Usage |
   |---------|-------|
   | **R** Requirements | Business context, Definition of Done, out of scope |
   | **E** Entities | Types, stereotypes, placement |
   | **A** Approach | Strategy, errors, tests |
   | **S** Structure | crates, modules, implementations, dependencies, boundaries |
   | **O** Operations | **Execute in the defined order** |
   | **N** Norms | Change-specific standards on top of `AGENTS.md` |
   | **S** Safeguards | Non-negotiable constraints, exact error messages |

4. **Load the project rules and context**: `AGENTS.md` (the baseline norms every
   canvas inherits), the tool configs below, and the modules named in Structure
   together with their closest existing patterns.
   - Read the root `Cargo.toml` (workspace members, `[workspace.dependencies]`,
     `[workspace.lints]`)
   - Read `arch-lint.toml` (scopes and the `core-stays-platform-independent`
     deny list) and `clippy.toml` (thresholds)
   - Read `justfile` (the recipes) and `tools.toml` (the issue workflow)

   Generated code MUST follow the existing conventions.

5. **Validate the Operations sequence**
   - The dependency order is right (value objects and errors in `bifrost-core` →
     traits → use cases → adapters → daemon wiring → boundaries
     (`arch-lint.toml`) → docs → verify), and no operation uses a type defined
     by a later one
   - No dependency breaks the boundary rules:
     - `bifrost-core` uses no platform crate (Windows, macOS, Android, Wayland,
       D-Bus bindings) and neither the daemon nor the tools
     - `bifrostd` is built on `bifrost-core`; the tools under `tools/` check the
       workspace and never use the product crates
     - A new platform crate is added to the deny list in `arch-lint.toml` in the
       pull request that introduces it
     - The final crate layout and the per-OS adapter template are decided by
       spike #23 and written into `AGENTS.md` section 2 by #50; until then those
       are the only rules
   - Each operation is atomic, names its crate and module, and lists its tests.
     Together they cover all of Structure, and they match its implementations,
     dependencies and boundary rules.

   If you find a problem, report it and suggest a `/spdd-prompt-update` before
   proceeding. Do NOT re-plan the sequence: the Operations order is the designed
   order.

6. **Generate the code, one operation at a time, in order**

   a. Read the operation spec: responsibility, representation, signatures,
   errors, behaviour and tests.

   b. **Apply the Norms** (`AGENTS.md` and the canvas Norms):
   - Newtypes with validating constructors; private fields; no needless `pub`
   - Functions of at most 25 lines and 3 parameters, cognitive complexity at
     most 8, no boolean flags in signatures or structs, identifiers of three or
     more characters
   - No `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`,
     indexing, string slicing or `as` conversions; no `unsafe`; no floating
     point or unchecked integer arithmetic
   - One error type per module boundary, never a leaked foreign error
   - `///` docs on public items and module docs; `Debug` on public types
   - No printing or `dbg!` outside tests; never log secrets, tokens or
     clipboard, notification or file content

   c. **Enforce the Safeguards**:
   - Use the exact error names and messages (do not modify them)
   - Apply the domain, data and security constraints exactly as written
   - **Zero lint exceptions**: never write `#[allow(...)]`, `#[expect(...)]` and
     arch-lint suppressions (`AGENTS.md` section 1). If code cannot fit a rule,
     stop and report it. The fix is the canvas, not an exemption.

   d. **Write the tests the operation lists**:
   - Test names say the behaviour, for example
     `a_use_of_a_platform_crate_in_core_fails`
   - Tests run with `cargo nextest` and doc tests with `cargo test --doc`; both
     are in `just test`
   - A check that fails a build ships with a test that fails on a violation and
     passes on a clean input, plus a test that runs it on the real workspace

   e. Write the code in the crate and module the operation names.

   Do NOT deviate from the Operations, add features, methods or fields that are
   not specified, or change the Safeguard messages.

7. **Batch validation**

   After ALL the code is generated, run `just check` (fmt-check (rustfmt and
   rumdl), clippy with -D warnings, cargo deny, arch-lint, nextest and doc
   tests).

   a. Fix failures by changing the code to fit the rule, never by adding
   exceptions.

   b. Cross-check every Definition of Done item, and check that the error names
   and messages match the Safeguards exactly.

   c. Check that placement matches Structure, that no forbidden dependency was
   introduced, and that
   `grep -rnE '#!?\[(allow|expect)\b' crates tools --include='*.rs'` prints
   nothing.

   If a failure cannot be fixed without deviating from the canvas, stop and
   propose a `/spdd-prompt-update`.

8. **Report**: the work item and the canvas, the files created or changed and
   their responsibilities, any deviations or assumptions, the `just check`
   result, and the Definition of Done checklist (pass or fail per item). Remind
   the user: run `just project-check`, then `just project-sync`, after opening
   the PR (→ In review) and after the merge (→ Done). The pull request body
   names the issue with `Closes #<n>`.

## Review and iteration loop

> **When reality diverges, fix the prompt first, then update the code.**

If the code was already changed by hand (a review fix) and more operations
remain, first bring the canvas in line with it (`/spdd-reasons-canvas #<n>`,
stale canvas), so that generation does not undo the change.

1. Identify the incorrect or missing behaviour.
2. Trace it to a section: wrong interpretation → Requirements; missing type or
   relationship → Entities; flawed strategy → Approach; wrong placement →
   Structure; wrong detail → Operations; missing standard → Norms; missing
   constraint → Safeguards.
3. Update the canvas with `/spdd-prompt-update`.
4. Regenerate only the affected components.
5. Commit the code and its tests, referencing the issue (`Closes #<n>` in the
   pull request, `(#<n>)` in each commit title (`Refs #<n>` for an epic)). The
   canvas is never part of a commit.

## Markdown output

Everything this skill writes to Markdown must pass `just fmt-check`.

## Guardrails

- Do NOT generate code before reading the entire canvas and `AGENTS.md`
- Do NOT generate code for a canvas without a resolved work item
- Do NOT re-plan, reorder or skip operations
- Do NOT change signatures, field names, variant names or error messages from
  the specification
- Do NOT add features, types or fields that are not specified
- Do NOT add lint exceptions of any kind
- Do NOT patch code directly when issues are found: update the canvas first
- Always verify against the Definition of Done, and always finish with
  `just check` passing
- Never stage the canvas: commit only code and tests
