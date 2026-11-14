# Linux desktop platform

For the settings and pairing UI and the tray on the Linux desktop, decided in
[ADR 0001](../../../../docs/adr/0001-process-model.md): iced for the UI and
`ksni` for the StatusNotifierItem tray, on Hyprland first. Android has its own
UI in [android.md](android.md). The macOS and Windows UIs are decided when those
milestones start.

## What the UI is

The UI is a thin client of `bifrostd` over a local socket. It holds no state of
its own, and the daemon applies every change. The UI can be closed, crash or be
absent without stopping sync, so a screen never assumes it is the only way to do
something: the command line client reaches the same actions. Show what the
daemon reports, and show the daemon being unreachable as a state of its own.

## The desktop slop test

Would a user of a Wayland desktop trust this window, or trip on a ported
website? The tells are web-shaped controls (hover-only affordances, giant
cards), a window that ignores the user's theme and scale, and a tray menu that
needs the window to be open.

## Layout & structure

- **Windows resize.** Design for a narrow window and a wide one. The sidebar or
  tab bar collapses; content reflows instead of stretching.
- **Tiling compositors set the size.** Never assume a fixed window size or
  position, and never request one the user cannot change.
- **The window class is stable.** The application id is what window rules match
  (`class` in `hyprctl clients`), so it never changes between releases.
- **Closing the window leaves the tray.** The window closing is not the app
  quitting; Quit is an explicit tray action.

## Input

- **Everything works from the keyboard.** Visible focus, a logical tab order,
  and a shortcut for the common actions.
- **Pointer targets are comfortable.** Do not shrink controls to fit more on
  screen; density comes from fewer elements.
- **Paste and typing take any script**, including non-ASCII text, in every text
  field.

## Typography

- **Aim for the system font and size.** iced draws its own widgets, so the UI
  must implement and verify that it follows the user's font setting rather than
  bundling a face.
- Text scales with the output scale. The ADR 0001 spikes ran at 1.2 and 1.5, so
  check sharpness and clipping at fractional scales.

## Color & theming

- **The UI must follow the system light or dark preference** (iced does not do
  it for you; implement and verify it) and test both. Dark is not an
  inverted light theme.
- Semantic roles for surface, text, accent, border and the status colors; no raw
  hex in widgets.

## Tray

- The tray speaks StatusNotifierItem, which Hyprland bars read; Hyprland itself
  has no tray.
- The menu carries Show settings and Quit, which ADR 0001 exercised over D-Bus,
  and the daemon's status and pause, which the specification plans for the tray.
  No entry depends on the window already being open.
- Status is never color only: pair the icon state with text in the menu.

## Accessibility

iced has no accessibility library, so a screen reader may not work with the UI
(ADR 0001 records this cost). Do not rely on a screen reader for the audit:
state the gap, keep every action available in the command line client, and make
focus, contrast and text size right.

## Verifying the build

- **Capture the real window with `grim`** on the Wayland output, never a
  browser, at the output's scale. Say which scale produced the evidence.
- **Check both themes and a narrow window** in the same pass.
- **Check the tray over D-Bus** (`Activate` and the menu events) when no bar is
  running, and say which one produced the evidence.
