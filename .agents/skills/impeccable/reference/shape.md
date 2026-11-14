# Shape

Discover what should be made and how it should work, then return a confirmed
design brief without code.

## Phase 1: Discovery interview

Do not write code or choose visual direction yet.

### Cadence

- Use the structured question tool when available; otherwise ask and stop.
- Ask two or three related questions per round, then wait. One round is the
  default; add a second only when the answers expose a material gap.
- Do not dump a questionnaire, repeat settled facts, or turn obvious facts into
  menus. Assert the likely reading and invite correction.
- A sparse prompt requires at least one answer round. A precise prompt may need
  only a compact confirmation.

### Round 1: purpose, people, and outcome

Choose the two or three questions that most change the result:

- What is this surface or feature for, and what problem must it solve?
- Who specifically reaches it, in what situation and state of mind?
- What is the primary thing they must understand or do? What would success look
  like?
- What is uniquely true here that a neighboring product or generic template
  could not claim?

### Round 2: material, behavior, and boundaries

Run only for material unresolved decisions:

- What real content, evidence, data, and assets must the experience carry? What
  are realistic minimum, typical, and maximum ranges?
- Which states and transitions matter: first-run, empty, loading, error,
  success, permissions, overflow, or expert use?
- What is the intended fidelity, breadth, and interactivity: exploration,
  production-ready screen, full flow, or broader surface?
- What must remain untouched? What would make the result feel wrong even if it
  looked polished?
- Which platform, framework, performance, accessibility, localization, or
  delivery constraints are binding?

Never ask for visual values or canned aesthetic lanes. Bifrost has no visual
design yet (`DESIGN.md` is not decided yet), so a brief states what the screen
does and for whom, and leaves the look to the platform's conventions
([desktop.md](desktop.md), [android.md](android.md)).

## Phase 2: Resolve the interaction

Check the brief against the surface table of `docs/SPEC.md`: a UI screen is a
thin client of the daemon, and the command line client reaches the same action
in `v0.2.0`. Settle composition and interaction only where they remain
materially open.

## Phase 3: Write the brief

Write the smallest useful brief:

1. **Job and audience:** who arrives, their context, and need.
2. **Outcome and proof:** primary task/action, success, real evidence, and
   product-specific truth.
3. **Selected direction:** the structural and interaction thesis, the sequence,
   and the implementation consequence.
4. **Scope and boundaries:** fidelity, breadth, interactivity, named target,
   what remains untouched, and explicit anti-goals.
5. **States and ranges:** realistic content/data ranges and material states.
6. **Interaction and layout:** hierarchy, topology, window or screen adaptation,
   affordances, feedback, and transitions; intent, not widget code.
7. **Constraints and open decisions:** platform, delivery, accessibility,
   localization, reusable components, and choices a builder must not invent.

Use three to five bullets when the task is settled; use the full structure only
for ambiguous, multi-screen, or standalone planning. Do not restate the
conversation.

## Confirm and stop

Present the brief for explicit confirmation or one correction round, then stop:
shape never writes code or a direction contract.

When no human or structured answer mechanism exists, mark assumptions plainly,
return the brief, and stop.
