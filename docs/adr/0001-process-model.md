# 0001: Process model, local IPC, UI toolkit and first Linux release

Status: accepted

Issue: [#24](https://github.com/jaoppb/bifrost/issues/24)

## Context

Bifrost has to read and write the clipboard and post notifications, and on every
system that means running inside the user's graphical session: a Windows
service, for example, cannot reach the clipboard. The product needs a program
that stays running, a command line to drive it, and later a UI for pairing and
settings with a tray. The first system is Linux on Hyprland
([docs/SPEC.md](../SPEC.md)).

The spike answered four questions: the process model, the local IPC, the UI
toolkit and what the first Linux release ships. The toolkits were compared with
throwaway builds on 2026-11-14, kept outside the repository, on Hyprland 0.57.2
(Wayland, two monitors at scales 1.5 and 1.2) with a StatusNotifier watcher
running. Each build was one window with a button, and Tauri also had a tray
icon. The numbers below come from those builds.

| Toolkit | Version | Release binary | Crates in `Cargo.lock` | RSS after start | Window mapped | Window on Wayland |
|---------|---------|----------------|------------------------|-----------------|---------------|-------------------|
| iced | 0.14.0 | 23 MB | 382 | 76 MB | 0.7 s | native |
| egui (eframe) | 0.33.3 | 16 MB | 395 | 79 MB | 0.2 s | native |
| Slint | 1.18.1 | 27 MB | 561 | 89 MB | 0.3 s | native |
| Tauri (wry) | 2.12.5 (0.57.2) | 11 MB | 421 | 139 MB | 0.6 s | native |

The tray is a separate question, because only Tauri has one built in. The
StatusNotifierItem protocol is what Hyprland bars (Waybar and others) read, and
Hyprland itself has no tray. Tauri registered its icon with the watcher. The
`ksni` crate 0.3.7 (84 crates, 6 MB binary) registered an item too, and it works
next to iced, egui or Slint.

Two checks decided more than the numbers:

- **The lint policy.** The workspace forbids `unsafe_code` and `missing_docs`
  and allows no exceptions ([AGENTS.md](../../AGENTS.md) section 3). Slint's
  generated code carries `allow(unsafe_code)` and `allow(missing_docs)`, and
  rustc refuses both under `forbid` (error E0453), so a crate that includes it
  cannot opt in to the workspace lints. The generated code of Tauri's
  `generate_context!` and the code of iced and egui compiled under the same
  lints with clippy pedantic and nursery on, apart from lints in the spike's own
  code.
- **The dependency policy.** `cargo deny check licenses` with the repository
  `deny.toml` rejected these licenses in the spikes: BSL-1.0 (two crates,
  `clipboard-win` and `error-code`, reached through iced, egui and Slint on
  Windows only), CC0-1.0 (iced), OFL-1.1 and Ubuntu-font-1.0 (egui's bundled
  fonts), MPL-2.0 (seven crates in Tauri) and, for Slint, a choice of
  GPL-3.0-only, the Slint royalty-free license or a commercial one. A permissive
  license is added per dependency, with a reason, in the pull request that adds
  it. A UI that is MIT or Apache-2.0 and ships on four systems fits the Slint
  royalty-free terms only with their attribution requirement, and Slint is also
  the only option whose license has to be chosen.

## Decision

**Process model.** A headless daemon, a command line client and a separate
settings UI, with the clients talking to the daemon over local IPC. All three
are planned and none is built.

- `bifrostd` runs in the user's graphical session and owns the clipboard,
  notifications and the network. On Linux it is started as a systemd user unit
  tied to the graphical session. It runs with no UI installed.
- The command line client and the UI are thin clients. They hold no state of
  their own, and the daemon is the only writer of settings and pairing state.
- The UI process hosts the tray. On Linux the tray speaks StatusNotifierItem.
  Closing the settings window leaves the daemon untouched.

**Local IPC.** A local socket owned by the user: a Unix domain socket under
`$XDG_RUNTIME_DIR` on Linux and macOS, and a named pipe on Windows, through one
cross-platform crate (`interprocess`, 2.4.8 is current). The socket's
permissions are the only authentication, which is enough because the socket is
private to the user. The message format and the commands are left to the
protocol design and to the crate layout spike
([#23](https://github.com/jaoppb/bifrost/issues/23)). This record fixes only
that the daemon is the server and that the clients are interchangeable.

**UI toolkit.** iced for the settings and pairing UI, with `ksni` for the tray
on Linux. The tray for macOS and Windows is chosen when those milestones start.
Tauri 2 is the fallback if the unverified points below fail for iced.

**First Linux release.** `v0.2.0` ships the daemon and the command line client
only. Pairing, status and rules go through the command line and a config file.
The UI and the tray come in a later release, on the toolkit above.

## Consequences

- The daemon has a public local API from its first release, so the command line
  client and the UI cannot reach around it, and the API is a contract that
  later releases keep compatible.
- The UI can crash, be absent or be replaced without stopping clipboard sync.
  The command line client works over SSH or in a session with no tray.
- Pairing by code or QR has to work as a command line flow in `v0.2.0`, which
  the protocol spikes ([#21](https://github.com/jaoppb/bifrost/issues/21)) must
  allow.
- The UI is written in Rust and passes the workspace lints, so there is one
  language, one toolchain and one `just check` for the product. The price is a
  tray crate of its own and a UI that draws its own widgets instead of using the
  system's.
- Android is outside this decision: its UI is Kotlin, in the Android milestone.
- Dependency exceptions the iced route will need: BSL-1.0, CC0-1.0 and the
  licenses of whatever fonts are bundled, each added in the pull request that
  introduces the UI.

## Rejected options

- **One application with a tray.** One binary and no IPC contract, but the
  daemon would stop with the window, there would be no headless or SSH use, the
  toolkit would sit inside the core's process, and a command line client would
  need a second path into the engine anyway.
- **Slint.** Its generated code cannot compile under `forbid(unsafe_code)` and
  `forbid(missing_docs)`, and the repository allows no exception. It is also the
  largest of the four (561 crates, 27 MB), and its license has to be chosen
  among GPL-3.0, a royalty-free license with attribution, and a paid one.
- **Tauri 2.** It compiled under the lint policy, has the smallest binary, a
  built-in tray and a native Wayland window. It is rejected as the default for
  the cost of its web stack: a second language and a frontend toolchain beside
  `just` and the Nix shell, WebKitGTK as a runtime dependency on Linux (139 MB
  resident at start against 76 MB for iced), seven MPL-2.0 crates, and a
  frontend that cannot be checked by clippy. It remains the fallback.
- **egui.** It started fastest (0.2 s) and compiled under the lints. It is
  immediate mode, which suits tools and debug panels better than forms with
  validation, and it bundles fonts under OFL-1.1 and Ubuntu-font-1.0. It was
  tested at 0.33.3 while 0.37 is current.
- **A UI in the first Linux release.** It would put the toolkit, packaging and
  tray risk on the critical path of `v0.2.0`, for users who run Hyprland and are
  comfortable with a command line and a config file.
- **A tray inside the daemon.** It would tie the daemon to a toolkit or a tray
  crate and break the rule that it runs without a UI.

## Unverified

These were not tested on Hyprland and rest on documentation or on the spikes'
limits. The decision is revisited if one of them fails for iced.

- Whether the windows render sharply at the fractional scales 1.5 and 1.2.
  Only that the windows map natively on Wayland was checked, not their pixels.
- Keyboard input, input methods, and clipboard and drag and drop inside any of
  the toolkits.
- Tray clicks and menus. Only that an item registers with the watcher was
  checked, for Tauri and for `ksni`, not an activation.
- Hiding the iced window and keeping the process alive as a tray host, and
  whether the iced window can set the application id Hyprland rules match on
  (the iced, egui and Slint spikes mapped with an empty class).
- Accessibility (screen readers) in iced.
- Every claim about macOS and Windows: tray crates, the named pipe through
  `interprocess` and the packaging. The Linux builds are the only ones made.
- The systemd user unit and the local socket were not built, and
  `interprocess` was not compiled.
- Build times were not recorded, because the five builds ran at once, and the
  RSS is one sample taken at window start.
