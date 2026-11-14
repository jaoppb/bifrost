# Product

This is the product record that UI work follows (`/impeccable`, see
[AGENTS.md](AGENTS.md)). The source of truth is
[docs/SPEC.md](docs/SPEC.md), and the UI decisions are in
[ADR 0001](docs/adr/0001-process-model.md). Nothing described here is built yet:
only a placeholder `bifrostd` crate exists. Where the specification marks
something planned or open, it stays marked that way here.

## Platforms

- **Linux desktop**, on Hyprland first: the settings and pairing UI in iced and
  the tray through `ksni` (StatusNotifierItem), decided in ADR 0001.
- **Android**: a Kotlin app, decided in the Android milestone (`v0.3.0`);
  Material 3 is the working assumption until then.
- macOS and Windows UIs are decided when those milestones start (`v0.4.0` and
  `v0.5.0`).

## Users

One person using their own devices: a Linux desktop, an Android phone, a Mac or
a Windows PC, in any combination. Someone comfortable with a command line, on
Hyprland first.

The primary user pairs once and then forgets bifrost is there. Most of the time
the daemon runs headless and the UI is closed. A person opens the UI to pair or
remove a device, to see that sync is running, or to find out why it is not, in
the moments the command line is a worse fit.

Sharing with other people's devices is out of scope for version 1
([#57](https://github.com/jaoppb/bifrost/issues/57) is open).

## Product Purpose

Peer-to-peer clipboard sync, notification mirroring and file sending across
one person's devices, over Tailscale, with no cloud account and no server of
bifrost's own. Success is that it disappears: what is copied on one device can
be pasted on the others, and the person rarely needs to look at bifrost.

## Surfaces

The UI is a thin client of `bifrostd` over a local socket and holds no state of
its own. The first Linux release (`v0.2.0`) ships the daemon and the command
line client only. The UI and the tray come in a later release. The product
owner puts pairing, devices and tray status first (decided in the `#16`
interview, not yet in the specification).

| Surface | What the person does there | Command line (`v0.2.0`) | UI and tray |
|---------|----------------------------|-------------------------|-------------|
| Pairing | Pairs a device and removes a paired one | yes | first UI release |
| Devices | Sees which paired devices are reachable | yes | first UI release |
| Status | Sees whether sync is running and pauses it | yes | first UI release, in the tray |
| Rules | Chooses which notifications are mirrored and what is never sent | config file and command | later |
| File sending | Sends a file to a device | yes | later |

Everything the UI does stays possible without it. Android has its own UI, with
the same surfaces decided in its milestone.

## Voice

Plain and calm. Short sentences, controls named by their action, and errors that
name the problem and the next step. The words are the specification's: say
"device", not "peer" or "node", and keep Tailscale internals out of screens
unless they explain a failure. No jargon, no marketing tone, no promise of a
feature that is not built.

## Constraints

- The UI never holds state or applies a change itself: the daemon is the only
  writer of pairing state and the only process that applies settings.
- The UI can be absent or crash without stopping sync, so the daemon being
  unreachable is a state a screen shows.
- Target privacy rules, not enforced yet: content marked concealed or sensitive
  is never synced, and what leaves a device is decided on the sending side.
- Android sends its clipboard on demand, because it cannot read it in the
  background. The on-demand send is an obvious one-step action.
- macOS shows mirrored notifications and sends none.
- Pairing by code or QR works as a command line flow first.
- Open: discovery ([#22](https://github.com/jaoppb/bifrost/issues/22)), pairing
  and trust ([#21](https://github.com/jaoppb/bifrost/issues/21)) and the crate
  layout ([#23](https://github.com/jaoppb/bifrost/issues/23)). Screens that
  depend on them are not designed here.
- Visual design is not decided: there is no `DESIGN.md`, and the UI follows the
  platform's conventions until one exists.

## Product Principles

- **It disappears.** The best state is that the person forgets bifrost; a screen
  earns its place by a task the command line does worse.
- **The daemon is the truth.** A screen shows what the daemon reports and never
  guesses; unreachable and out of sync are states, not blanks.
- **Nothing is only in the UI.** Every action is reachable from the command
  line.
- **Say what happened.** Errors and status name the problem and the next step in
  the product's own words.

## Accessibility & Inclusion

- iced has no accessibility library, so a screen reader may not work with the
  desktop UI (ADR 0001 records this cost, and none was tried). Keyboard access,
  visible focus, contrast in light and dark, and the system text size are
  required, and every action stays available in the command line client.
- Android accessibility (named controls, 48 dp touch targets, scalable text and
  the system Back gesture) follows Material 3 as a working assumption, to be
  confirmed in the `v0.3.0` milestone.
- Status is never color only.
