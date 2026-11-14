# Init flow

`init` captures durable product truth in `PRODUCT.md` at the repository root. It
does not invent a visual world and does not write `DESIGN.md`: bifrost has no UI
to document yet, so visual design is a non-goal until the UI exists.

## Step 1: Load current state

- **No `PRODUCT.md`:** explore, interview, and write it.
- **`PRODUCT.md` exists:** ask what product knowledge is stale or missing; do
  not reopen confirmed fields without a reason.

Never silently overwrite an existing file. Update it instead of creating a
competing authority.

## Step 2: Explore the project

Before asking, read enough to avoid making the person repeat known facts:

- `docs/SPEC.md`: the goal, the users and devices, the support matrix, the UI
  surfaces and the accepted limits. It is the source of product truth.
- `docs/adr/`: the decisions that bind the UI, starting with ADR 0001 (a
  headless `bifrostd`, a command line client and a separate settings UI over a
  local socket, with iced and `ksni` on the Linux desktop).
- `README.md` and `AGENTS.md` for the current status and the rules.

Treat repository evidence as a hypothesis, not approval. Everything in the
documents marked planned or open stays marked that way.

## Step 3: Interview for product truth

Ask the person with the AskUserQuestion tool, one focused round at a time (at
most three questions), and wait for the answer. Ask only about material gaps the
documents do not answer. Confirm inferences. The person is in this session:
never substitute inferred answers for a real round, and record undecided facts
as undecided.

Start with what most changes future UI decisions:

1. Who is the primary user, in what situation, and what do they do on a UI
   screen instead of the command line?
2. Which surfaces get a UI first, on which device, and what must stay possible
   without one?
3. What tone should the product's own words have (labels, errors, pairing
   prompts), and which terms are fixed?

Do not ask for colors, typography, visual references or a style during init. If
the person volunteers a binding visual constraint, record it without expanding
it.

### What belongs here

- users, jobs, workflows, purpose, success, and operating context;
- capabilities, constraints, terminology, platforms, and accessibility;
- confirmed voice and brand commitments.

### What does not belong here

- visual worlds, palettes, typography, components, or screen concepts;
- invented testimonials, benchmarks, pricing, licensing, or release claims;
- anything the specification marks open, stated as decided.

## Step 4: Write PRODUCT.md

Write only confirmed facts and explicitly marked open decisions. Omit irrelevant
sections rather than filling them with generic prose.

```markdown
# Product

## Platforms
[Linux desktop (iced and ksni), Android (Kotlin); macOS and Windows UIs are
decided at their milestones.]

## Users
[Primary users, their situation, and job.]

## Product Purpose
[What the product does, why it exists, and what success means.]

## Surfaces
[Each UI surface, which devices have it, and what stays in the command line.]

## Voice
[The tone of the product's own words and its fixed terms.]

## Constraints
[Confirmed technical and product constraints, and open decisions.]

## Product Principles
[Three to five durable principles derived from confirmed answers; no visual
recipes.]

## Accessibility & Inclusion
[Known needs or the required standard, including the toolkit limits.]
```

Preserve useful headings when updating. Write it before any UI work.

### Completion gate

Before resuming any design work, verify that `PRODUCT.md` exists at the
repository root and contains the confirmed product record. If the file is
absent, init is incomplete. Do not substitute interview notes or later design
prose for the file.

## Step 5: Hand off

Tell the person which facts are still open and where the UI work goes next:
`/impeccable shape` for a screen brief, then the platform reference and
`/impeccable polish` or `/impeccable audit` on the built result.
