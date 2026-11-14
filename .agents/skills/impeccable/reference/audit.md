# Audit

Run systematic **technical** quality checks on a bifrost UI (the Linux desktop
UI and tray, or the Android app) and generate a report. Don't fix issues;
document them for other commands to address.

This is a code-level audit, not a design critique. Audit from source (iced and
`ksni` on the desktop, Kotlin and Compose on Android) and from captures of the
running UI. Score against the platform reference: [desktop.md](desktop.md) or
[android.md](android.md), both when the target spans them. Read them before
scoring.

## Diagnostic Scan

Run checks across 5 dimensions. Score each 0-4 using the criteria below.

### 1. Accessibility

**Check for**:

- **Missing names**: interactive elements without accessible names, roles or
  state (Android: TalkBack semantics). On the desktop, iced has no
  accessibility library (ADR 0001), so do not score a screen reader: report the
  gap once and check the rest below.
- **Focus and reading order**: illogical traversal, unreachable controls, focus
  lost on navigation, no visible focus
- **Keyboard**: a desktop action with no keyboard path
- **Text scaling**: fixed sizes that ignore the system font setting (Android:
  px instead of sp); layouts that clip or overlap at large sizes
- **Touch targets** (Android): below 48 dp, or crammed without spacing
- **Reduced motion ignored**: large movement with no crossfade alternative
- **Contrast**: text or controls failing contrast in light or dark

**Score 0-4**: 0=Unusable without a mouse or screen, 1=Major gaps, 2=Partial
(names exist, order or scaling breaks), 3=Good (minor gaps), 4=Excellent
(named, ordered, keyboard complete, scales cleanly)

### 2. Performance

**Check for**:

- **Slow startup**: heavy work before the first frame, or the UI blocking on
  the daemon
- **Main-thread work**: synchronous calls to the daemon or the disk in the draw
  or input path; dropped frames
- **Unvirtualized lists**: long lists of devices or rules with no lazy layout
- **Wasted rendering**: needless redraws (iced `view`) or recompositions
  (Compose); missing keys
- **Weight**: unused dependencies and bundled assets

**Score 0-4**: 0=Janky everywhere, 1=Major problems, 2=Partial, 3=Good,
4=Excellent (fast start, smooth, lean)

### 3. Appearance & Theming

**Check for**:

- **Hard-coded colors**: raw hex instead of theme roles (Material color roles on
  Android, the iced theme palette on the desktop)
- **Broken dark theme**: missing variants, poor contrast, a quick invert
- **Ignored system preference**: the theme does not follow the system light or
  dark setting
- **Dynamic Color** (Android 12+): no static fallback scheme

**Score 0-4**: 0=Hard-coded everything, 1=Minimal tokens, 2=Partial, 3=Good
(minor hard-coded values), 4=Excellent (semantic throughout, both themes
first-class)

### 4. Platform Conformance (CRITICAL)

Score against the loaded platform reference, including its slop test.
**Check for**:

- **Broken system behavior**: predictive Back hijacked (Android); a window that
  ignores resizing or the compositor's tiling (desktop)
- **Inset violations**: content under system bars, the cutout or the keyboard
  (Android)
- **Off-platform navigation**: custom global nav, an overloaded bottom bar on a
  tablet, iOS patterns on Android
- **Web-shaped controls**: hover-dependent affordances, custom toggles,
  card-heavy layouts
- **Tray**: a menu entry that needs the window open, or status shown by color
  only
- **Icon drift**: mixed icon sets instead of one library
- **System drift**: repeated patterns that conflict with the platform or the
  product

**Score 0-4**: 0=Web port (nothing native), 1=Heavy violations (3-4 kinds),
2=Some (1-2 noticeable), 3=Mostly conformant (subtle issues), 4=Fully native (a
fluent user trusts every screen)

### 5. Adaptivity

**Check for**:

- **Stretched layouts**: a wide window or tablet rendering a scaled-up narrow
  layout
- **Narrow windows**: clipping when a tiling compositor gives a small window
- **Output scale**: blur or clipping at fractional scales (1.2, 1.5)
- **Orientation and IME** (Android): landscape clipping, inputs hidden behind
  the keyboard
- **Multi-window and foldables** (Android): layouts that break in split screen
  or on posture change

**Score 0-4**: 0=One size only, 1=Major breakage, 2=Partial, 3=Good (minor edge
cases), 4=Excellent (adapts across windows, scales and orientations)

## Generate Report

### Audit Health Score

| # | Dimension | Score | Key Finding |
|---|-----------|-------|-------------|
| 1 | Accessibility | ? | [most critical issue or "--"] |
| 2 | Performance | ? | |
| 3 | Appearance & Theming | ? | |
| 4 | Platform Conformance | ? | |
| 5 | Adaptivity | ? | |
| **Total** | | **??/20** | **[Rating band]** |

**Rating bands**: 18-20 Excellent (minor polish), 14-17 Good (address weak
dimensions), 10-13 Acceptable (significant work needed), 6-9 Poor (major
overhaul), 0-5 Critical (fundamental issues)

### Platform Conformance Verdict

**Start here.** Pass/fail: does this read as a native UI or a ported website?
List specific violations. Be brutally honest.

### Executive Summary

- Audit Health Score: **??/20** ([rating band])
- Total issues found (count by severity: P0/P1/P2/P3)
- Top 3-5 critical issues
- Recommended next steps

### Detailed Findings by Severity

Tag every issue with **P0-P3 severity**:

- **P0 Blocking**: Prevents task completion. Fix immediately
- **P1 Major**: Significant difficulty or platform-guideline violation. Fix
  before release
- **P2 Minor**: Annoyance, workaround exists. Fix in next pass
- **P3 Polish**: Nice-to-fix, no real user impact. Fix if time permits

For each issue, document:

- **[P?] Issue name**
- **Location**: Screen, file, line
- **Category**: Accessibility / Performance / Theming / Conformance / Adaptivity
- **Impact**: How it affects users
- **Guideline**: The Material or platform rule it violates (if applicable)
- **Recommendation**: How to fix it
- **Suggested command**: One of `/impeccable` clarify, colorize, distill,
  layout, onboard, polish, quieter, shape, typeset

### Patterns & Systemic Issues

Identify recurring problems that indicate systemic gaps rather than one-off
mistakes:

- "Hard-coded colors appear in 15+ screens, should use theme roles"
- "Touch targets consistently below 48 dp in the device list rows"

### Positive Findings

Note what's working well: good practices to maintain and replicate.

## Recommended Actions

List recommended commands in priority order (P0 first, then P1, then P2):

1. **[P?] `/command-name`**: Brief description (specific context from audit
   findings)
2. **[P?] `/command-name`**: Brief description (specific context)

**Rules**: Only recommend the commands listed above. Map findings to the most
appropriate command. End with `/impeccable polish` as the final step if any
fixes were recommended.

After presenting the summary, tell the user:

> You can ask me to run these one at a time, all at once, or in any order you
> prefer.
>
> Re-run `/impeccable audit` after fixes to see your score improve.
