# Operate mode depth

Every bifrost surface is an Operate surface: the user is in a task (pairing a
device, checking what is synced, setting a rule) and the tool should disappear
into it. This file is extended depth for the rules in `SKILL.md` and
[craft-floor.md](craft-floor.md).

## The product slop test

Familiarity is a feature here. The test is whether a user fluent in the
platform can trust the interface immediately or must pause at every subtly-off
component.

The failure mode is strangeness without purpose: over-decorated buttons,
mismatched controls, gratuitous motion, display fonts where labels belong,
invented affordances for standard tasks. The bar is earned familiarity.

## Typography

- One family is often right. A well-tuned sans carries headings, buttons,
  labels, body and data.
- Fixed scale. Use a tight ratio (1.125 to 1.2 between steps);
  exaggerated contrast creates noise.
- Prose keeps a comfortable measure. Data and compact UI can run denser.

## Color

Restrained is the floor.

- A state-rich vocabulary: hover, focus, pressed, disabled, selected, loading,
  error, warning, success. Standardize these.
- Accent color marks primary actions, current selection and state, not
  decoration.
- A second neutral layer for sidebars, toolbars and panels.

## Layout

- Responsive behavior is structural: collapse a sidebar, switch a navigation
  bar to a rail, reflow a list.

## Components

Every interactive component has default, hover (desktop), focus, pressed,
disabled, loading and error states, and a screen that depends on the daemon also
has a daemon-unreachable and an out-of-sync state. Do not ship half of them.

- A loading state, when the daemon is slow to answer, that does not shift the
  layout.
- Empty states that teach the interface, not "nothing here". Bifrost's first
  empty state is "no paired device yet": point to the pairing action.
- Consistent affordances: the same button shape, form-control vocabulary and
  icon style on every screen.

## Motion

- 150 to 250 ms on most transitions. Users are in flow.
- Motion conveys state: change, feedback, loading, reveal. Nothing else.
- Nothing animates on load.

## Avoid

- Decorative motion that does not convey state.
- Inconsistent component vocabulary across screens. If "Save" looks different in
  two places, one is wrong.
- Reinventing standard affordances for flavor.
- Heavy color or saturated accents on inactive states.
- A modal as the first thought. Exhaust inline and progressive alternatives
  first.

## Product permissions

- System fonts and familiar sans defaults.
- Standard navigation: a sidebar or tabs on the desktop, Material navigation on
  Android.
- Density, when users need it: many devices, many rules, many labels.
- Consistency over surprise. Delight is saved for moments, not screens.
