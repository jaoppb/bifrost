# Craft floor

Read this immediately before any UI edit, including small refinements, and build
without announcing the checklist. `PRODUCT.md` and the brief the person gave
override anything here except a line marked as a ban.

## Verify

Each of these is a check on the built result, not an intention. Run them
together in each batched look at the render.

- **Contrast:** body and placeholder text at least 4.5:1, large text and
  controls at least 3:1, in both light and dark themes. On colored surfaces tint
  secondary text from that hue; never plain gray.
- **Spacing:** tight groups, generous separation, more space above a heading
  than below it. Spacing comes from shared values, not per-widget numbers.
- **Type:** roles from the platform's scale, with obvious size and weight steps.
  Run the real copy, including long device names (and translated labels, if the
  UI is localized), and fix what clips or overflows.
- **Motion:** only to convey state. Honor the system's reduce-motion setting.
- **States:** default, hover (desktop), focus, pressed, disabled, loading,
  error, empty, and daemon unreachable or out of sync. Plus real content and
  working controls.
- **Input:** every action is reachable by keyboard on the desktop and by touch
  and the system Back gesture on Android. Focus is always visible.
- **Copy:** the product's own language. Controls name their action; errors name
  the problem and the recovery. Use the terms of `docs/SPEC.md`.
- **Coverage:** every requirement of the brief is present and findable within
  seconds.

## Refuse

These are defaults, not bans: the brief's own words can earn one. Reaching for
one when the choice is free means you were not deciding.

- Same-size cards of icon, heading and text as the screen structure. Nested
  cards are always wrong.
- A modal for a task that needs neither interruption nor protected focus.
- A kicker or eyebrow label above a heading. This one is a ban.
- Gradient text, and glass or blur as decoration.
- Hard offset shadows, and a colored border above 1 px on list items or alerts.
- Unicode glyphs or emoji standing in for an icon system. Icons come from one
  library in one stroke and weight.
- Light or dark picked by taste. Both themes ship, and the user's system setting
  decides.
- A display face in labels, buttons or data.
- A custom control where the platform has a standard one.

The floor holds the mechanics; it never picks the direction.
