# Bifrost product specification

This is the source for what bifrost does, for whom, on which system and what it
deliberately does not do. The README platform table is derived from it, and
review keeps the two in sync. Architecture and protocol design are not here: the
spikes in the [open questions](#open-questions) answer them.

Nothing described here is built yet. The matrix states the **target**: what
bifrost aims to support at each milestone. The README states the **status**.

## Goal

A person with several devices copies on one, sees the same clipboard on the
others, gets the notifications of one device on the others and sends a file from
one to another, with no cloud account and no server of bifrost's own. Devices
talk to each other directly over [Tailscale](https://tailscale.com/).

## Users and devices

- The user is one person using their own devices: a Linux desktop, an Android
  phone, a Mac, a Windows PC, in any combination.
- Devices are paired explicitly and must be on the same tailnet. Once paired,
  they find each other automatically, so nobody types an address. How discovery
  works is an [open question](#open-questions).
- Linux comes first, on Hyprland. The systems land in this order: Linux
  (`v0.2.0`), Android (`v0.3.0`), macOS (`v0.4.0`), Windows (`v0.5.0`).

## Capabilities

- **Clipboard sync.** What is copied on one device can be pasted on the others.
- **Clipboard history integration.** Synced items appear in the clipboard
  history of the system, instead of bifrost keeping a history of its own. The
  [list of integrations](#clipboard-history-integrations) starts with Vicinae.
- **Notification mirroring.** A notification of one device is shown on the
  others with its icon and title, and a source indicator says which device it
  came from. Rules decide which notifications are mirrored.
- **Notification capture.** A device reads its own notifications to send them
  to the others. Mirroring is the receiving side and capture is the sending
  side, because some systems only allow the first.
- **File sending.** The user sends a file from one device to another.

## Support matrix

Each cell is **supported**, **limited** (with the limit) or **not planned**,
followed by the milestone. No cell is built yet.

| Capability | Linux (Hyprland first) | Android | macOS | Windows |
|------------|------------------------|---------|-------|---------|
| Clipboard sync | supported, `v0.2.0`; GNOME is not planned | limited: receives automatically, sends on demand, `v0.3.0` | supported, `v0.4.0` | supported, `v0.5.0` |
| Clipboard history integration | supported through Vicinae, `v0.2.0` | not planned, open: [#54][i54] | not planned, open: [#55][i55] | not planned, open: [#56][i56] |
| Notification mirroring | supported, `v0.2.0` | supported, `v0.3.0` | supported, `v0.4.0` | supported, `v0.5.0` |
| Notification capture | supported, `v0.2.0` | supported, `v0.3.0` | not planned | supported, `v0.5.0` |
| File sending | supported, `v0.2.0` | supported, `v0.3.0` | supported, `v0.4.0` | supported, `v0.5.0` |

### Accepted limits

Version 1 accepts these operating system limits instead of working around them.

- **Android sends its clipboard on demand.** Android does not let an app read
  the clipboard in the background. The device receives automatically, and the
  user sends the current clipboard by hand.
- **macOS captures no notifications.** macOS has no public API to read the
  notifications of other apps, so a Mac shows mirrored notifications and sends
  none.
- **GNOME clipboard sync is not planned.** GNOME has no clipboard protocol for
  background programs, so clipboard sync covers Hyprland first and GNOME is out
  for now.

### Clipboard history integrations

1. **Vicinae** on Linux, the first integration.

An integration for another system is added only once its issue
([#54][i54], [#55][i55], [#56][i56]) shows a public way to do it.

## Privacy

These are target rules: nothing enforces them yet.

- Content that its source marks as concealed or sensitive, such as a password
  manager's copies, is never synced. There is no override in version 1.
- What leaves a device is decided on the sending side, never by the receiver.
  A device does not send what its own rules or the rule above exclude, and it
  does not rely on the receiver to drop it.

## Non-goals

- Syncing with other people's devices: version 1 is one person and their own
  devices, pending [#57][i57].
- Any server, account or cloud relay of bifrost's own.
- What the [accepted limits](#accepted-limits) rule out.
- Architecture and protocol design, and UI design: they belong to the spikes.

## Open questions

- How are devices discovered over Tailscale, and what limits does each system
  put on it? [#22](https://github.com/jaoppb/bifrost/issues/22)
- How do devices pair and trust each other, and what protects the wire?
  [#21](https://github.com/jaoppb/bifrost/issues/21)
- What is the crate layout and the adapter template for each system?
  [#23](https://github.com/jaoppb/bifrost/issues/23)
- Which technology builds the desktop shell and the settings UI?
  [#24](https://github.com/jaoppb/bifrost/issues/24)
- Can Android, macOS and Windows integrate with a clipboard history?
  [#54][i54], [#55][i55], [#56][i56]
- Can bifrost sync with other people's devices?
  [#57][i57]

[i54]: https://github.com/jaoppb/bifrost/issues/54
[i55]: https://github.com/jaoppb/bifrost/issues/55
[i56]: https://github.com/jaoppb/bifrost/issues/56
[i57]: https://github.com/jaoppb/bifrost/issues/57
