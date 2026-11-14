# Typeset

Typography carries information, hierarchy, and voice. Improve it inside the
established look; do not replace the identity unless the user asked to.

---

## Mode

Every bifrost surface is Operate (see `SKILL.md`): stability, scanability, and
measure come first. A single well-tuned family and a fixed role scale are often
right. Follow [desktop.md](desktop.md) or [android.md](android.md), including
platform scaling and accessibility behavior.

Preserve confirmed families and improve their use.

## Two isolated assessments

When a sub-agent tool is available and permitted, run these independently;
otherwise run them yourself in this order.

1. **Typographic assessment:** inspect representative screens and styles. Answer
   every question below with a file and a value:
   - **Authority and fit:** Which faces, weights, and roles are established? Do
     they fit the product and established look, or are they unexamined defaults?
     Is every family necessary?
   - **Hierarchy:** Can heading, body, label, metadata, and data roles be
     distinguished at a glance? Are adjacent sizes or weights too close to carry
     different jobs?
   - **Scale and consistency:** Is there a deliberate role scale, or a
     collection of arbitrary values? Do repeated roles stay identical across
     screens and states?
   - **Reading:** Does body copy stay within a comfortable 45–75 character
     measure? Are line height, paragraph rhythm, contrast, and tracking tuned to
     the actual face, width, language, and surface?
   - **Stress:** What happens with long headings, localization expansion, zoom,
     narrow containers, missing weights, and font fallback?
   - **Delivery:** Are only used assets loaded? Do fallback metrics, loading
     strategy, and variable-font settings avoid invisible text and disruptive
     reflow?
2. **Source scan:** read the font, size and weight values in the target files
   (iced `Font` and text sizes, Android text styles) and check them against the
   roles you named. Look for sizes and weights that bypass the shared values. A
   clean scan is a floor, not proof of good typography.

## Set the system

Before editing, state:

- the roles the interface needs;
- the intended contrast between those roles;
- the reading measure and density;
- which existing faces and weights are authoritative;
- any performance, localization, or accessibility constraints.

Use the fewest roles and families that make the hierarchy unmistakable. Combine
size, weight, space, and tone deliberately instead of asking size alone to do
all the work. Role names and tokens should describe purpose rather than values.

## Apply

- Keep body copy comfortably readable and scalable. Use the platform's body
  size as the floor (Material's Body Medium on Android, the desktop toolkit's
  default text size on the desktop) unless a dense role, platform convention, or
  user setting justifies otherwise.
- Keep prose in the 45–75ch range. Tune line height inversely with measure:
  wider lines generally need more leading.
- Compensate light text on dark surfaces on all three perceptual axes: slightly
  more line height, a touch more tracking, and one step more weight when the
  face needs it.
- Tune line height to the face, width, language, and contrast, not a universal
  ratio.
- Keep repeated roles consistent across screens and states.
- Use numeric, tabular, code, and label features when their content benefits.
- Bundle only used font assets and weights, and keep a system fallback.
- Keep dense product surfaces spatially predictable.
- Preserve user font settings and platform text scaling.
- Use paragraph spacing or first-line indentation as the primary paragraph
  rhythm; combining both usually double-marks the boundary.

Do not make type decorative at the expense of comprehension, or introduce a
second family without a clear role it alone can perform.

## Line breaks

- Break a heading by hand only between phrases, and check the break at every
  width the heading renders at. A hard break that lands mid-line in a narrow
  window is worse than the default wrap.
- In a column too narrow to wrap well, edit the copy or widen the column.
- An uneven break can carry meaning; make it deliberate and keep it rare.

## Verify

- Primary, secondary, body, and metadata roles are recognizable without reading
  the copy.
- Long text remains comfortable across relevant widths and languages.
- The typography belongs to the product and its established look.
- Loading does not create disruptive reflow or invisible text.
- Text scaling, focus, contrast, and narrow windows remain usable.
- The final source scan has no unexplained findings.

Answer each item with rendered or source evidence, then rerun the scan. Do not
substitute a bare “yes” for verification.

When the hierarchy holds, hand off to `/impeccable polish`.
