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
icon. The numbers below come from those builds. egui was first built at 0.33.3
(16 MB, 395 crates, 0.2 s) and rebuilt at 0.37.0, the current release, whose
default features now include the wgpu renderer and AccessKit. The 0.37 API also
dropped `run_simple_native`, which the first spike used.

| Toolkit | Version | Release binary | Crates in `Cargo.lock` | RSS after start | Window mapped | Window on Wayland |
|---------|---------|----------------|------------------------|-----------------|---------------|-------------------|
| iced | 0.14.0 | 23 MB | 382 | 76 MB | 0.7 s | native |
| egui (eframe) | 0.37.0 | 29 MB | 436 | 73 MB | 0.4 s | native |
| Slint | 1.18.1 | 27 MB | 561 (includes `ksni`) | 89 MB | 0.3 s | native |
| Tauri | tauri 2.12.5, wry 0.57.2 | 11 MB | 421 | 139 MB (main process only) | 0.6 s | native |

The tray is a separate question, because only Tauri has one built in. The
StatusNotifierItem protocol is what Hyprland bars (Waybar and others) read, and
Hyprland itself has no tray. Tauri registered its icon with the watcher. The
`ksni` crate 0.3.7 (84 crates, 6 MB binary) was put next to iced and egui in a
second round of spikes, in each of them, with a window that carries a
text field and a button.

That round also tested what a settings UI does on Hyprland, on a Wayland output
at scale 1.2, with iced 0.14.0 and egui 0.37.0:

| Check | iced | egui |
|-------|------|------|
| Window sharp at the output's physical resolution (a `grim` capture 940 pixels wide for a window 784 logical pixels wide) | yes | yes |
| Application id read by Hyprland window rules (`class` in `hyprctl clients`) | set through `platform_specific.application_id` | set through `with_app_id` |
| Typing, paste with non-ASCII text and copy, through `ydotool` and `wl-copy` | works | works |
| Tray calls over D-Bus, as a bar makes them | `Activate` shows the window, and the menu `Event` runs Show and Quit | `Activate` runs the handler; the menu was not called |
| Hide the window to the tray and show it again | works in `iced::daemon` mode: the window is closed and reopened, the process stays up (86 MB open, 78 MB closed) | not working with `ViewportCommand::Visible(false)`, which leaves the window mapped; closing and reopening a window in one process was not found in `eframe` |
| Accessibility | no AccessKit in the dependency tree | AccessKit and `accesskit_unix` are in the tree by default |

The hide result looks like a winit limit on Wayland, since iced's
`window::Mode::Hidden` also left its window mapped, and iced works only because
its daemon mode can close and open windows. Other ways for egui (minimising,
more than one viewport, starting `run_native` again) were not tried.

Two checks decided more than the numbers:

- **The lint policy.** The workspace forbids `unsafe_code` and `missing_docs`
  and allows no exceptions ([AGENTS.md](../../AGENTS.md) section 3). Slint's
  generated code carries `allow(unsafe_code)` and `allow(missing_docs)`, and
  rustc refuses both under `forbid` (error E0453), so a crate that includes it
  cannot opt in to the workspace lints. The generated code of Tauri's
  `generate_context!` and the code of iced and egui (0.37.0 too) compiled under
  the same lints with clippy pedantic and nursery on, apart from lints in the
  spike's own code.
- **The dependency policy.** `cargo deny check licenses` with the repository
  `deny.toml` rejected these licenses in the spikes: BSL-1.0 (two crates,
  `clipboard-win` and `error-code`, reached through iced, egui and Slint on
  Windows only), Unlicense (`ksni`, which Slint pulls in too), CC0-1.0 (iced),
  OFL-1.1 and Ubuntu-font-1.0 (egui's bundled fonts, at 0.33.3 and 0.37.0),
  MPL-2.0 (seven crates in Tauri) and, for Slint, a choice of GPL-3.0-only, the
  Slint royalty-free license or a commercial one. The policy allows permissive
  licenses only, so each of these (several are not permissive: MPL-2.0 is weak
  copyleft and the font licenses are not code licenses) needs an exception added
  to `deny.toml` with a reason, in the pull request that adds the dependency.
  Slint is the only option whose license has to be chosen, and the terms of its
  royalty-free license were read from its license text, not tested.

## Decision

**Process model.** A headless daemon, a command line client and a separate
settings UI, with the clients talking to the daemon over local IPC. All three
are planned; only a placeholder `bifrostd` crate exists.

- `bifrostd` runs in the user's graphical session and owns the clipboard,
  notifications and the network. On Linux it is started as a systemd user unit
  tied to the graphical session. It runs with no UI installed.
- The command line client and the UI are thin clients. They hold no state of
  their own, and the daemon is the only writer of pairing state and the only
  process that applies settings. Settings come from the config file, which the
  daemon reads and reloads, and from commands that the daemon writes into it.
- The UI process hosts the tray. On Linux the tray speaks StatusNotifierItem.
  Closing the settings window leaves the daemon untouched, and with iced it
  closes the window while the UI process stays up for the tray (`iced::daemon`).

**Local IPC.** A local socket owned by the user: a Unix domain socket under
`$XDG_RUNTIME_DIR` on Linux, a user-private directory to be chosen on macOS
(which defines no `XDG_RUNTIME_DIR`), and a named pipe on Windows, through one
cross-platform crate (`interprocess`, 2.4.8 is current). On Linux the socket is
a file path, never an abstract name: a spike with `interprocess` created the
socket file with mode `rwxr-xr-x`, inside a runtime directory of mode
`rwx------`, so the private directory is what keeps other users out. An abstract
socket has no file and no permissions at all. The directory permission is meant
to be the only authentication on Linux, on the assumption that nothing else of
the user connects. The message format and the commands are left to the protocol
design and to the crate layout spike
([#23](https://github.com/jaoppb/bifrost/issues/23)). This record fixes only
that the daemon is the server and that the clients are interchangeable.

**UI toolkit.** iced for the settings and pairing UI, with `ksni` for the tray
on Linux. The tray for macOS and Windows is chosen when those milestones start.
Tauri 2 is the fallback if the unverified points below fail for iced.

**First Linux release.** `v0.2.0` ships the daemon and the command line client
only. Pairing, devices, file sending and status go through the command line, and
rules also through a config file, as the surfaces table in the specification
says. The UI and the tray come in a later release, on the toolkit above.

## Consequences

- The daemon has a public local API from its first release, so the command line
  client and the UI cannot reach around it, and the API is a contract that
  later releases keep compatible.
- The UI can crash, be absent or be replaced without stopping clipboard sync.
  The command line client works over SSH or in a session with no tray.
- Pairing by code or QR has to work as a command line flow in `v0.2.0`, which
  the protocol spikes ([#21](https://github.com/jaoppb/bifrost/issues/21)) must
  allow.
- The UI is written in Rust and can pass the workspace lints (a one-window iced
  spike did), so there is one
  language, one toolchain and one `just check` for the product. The price is a
  tray crate of its own and a UI that draws its own widgets instead of using the
  system's.
- iced has no accessibility library, so a screen reader may not work with the
  UI, and none was tried with either toolkit. egui would not have this cost.
- Android is outside this decision: its UI is Kotlin, in the Android milestone.
- Dependency exceptions the iced route will need: BSL-1.0, CC0-1.0, Unlicense
  (`ksni`) and the licenses of the fonts it bundles, which were not checked,
  each added in the pull request that introduces the UI.

## Rejected options

- **One application with a tray.** One binary and no IPC contract, but the
  daemon would stop with the window, there would be no headless or SSH use, the
  toolkit would sit inside the core's process, and a command line client would
  need a second path into the engine anyway.
- **Slint.** Its generated code cannot compile under `forbid(unsafe_code)` and
  `forbid(missing_docs)`, and the repository allows no exception. It is also the
  the one with the most crates (561), and its license has to be chosen
  among GPL-3.0, a royalty-free license with attribution, and a paid one.
- **Tauri 2.** It compiled under the lint policy, has the smallest binary, a
  built-in tray and a native Wayland window. It is rejected as the default for
  the cost of its web stack: a second language and a frontend toolchain beside
  `just` and the Nix shell, WebKitGTK as a runtime dependency on Linux (139 MB
  resident at start for the main process, against 76 MB for the bare iced spike;
  the two spikes differ in content, so this is soft evidence), seven MPL-2.0
  crates, and a frontend that cannot be checked by clippy. It remains the
  fallback.
- **egui.** At 0.37.0 it compiled under the lints, rendered sharply, took
  keyboard input and clipboard, and has AccessKit, which the iced spike's
  dependency tree did not. It is rejected as the tray host on one negative
  result: hiding with `ViewportCommand::Visible(false)` left the window mapped,
  and no way to reopen a closed window was found. Other ways were not tried (see
  Unverified). A tray that shows and hides the settings window would otherwise
  need a tray process of its own that starts the UI on demand. Its bundled fonts
  need OFL-1.1 and Ubuntu-font-1.0 exceptions, while the fonts of iced were not
  checked, so that point is asymmetric. The two toolkits are otherwise close,
  and egui is the choice to revisit if iced's missing accessibility matters more
  than hosting the tray in the UI process, or if a separate tray process is
  accepted.
- **A UI in the first Linux release.** It would put the toolkit, packaging and
  tray risk on the critical path of `v0.2.0`, for users who run Hyprland and are
  comfortable with a command line and a config file.
- **A tray inside the daemon.** It would tie the daemon to a toolkit or a tray
  crate and break the rule that it runs without a UI.

## Unverified

These were not tested, or were tested only in part, and rest on documentation
or on the spikes' limits. The decision is revisited if one of them fails for
iced.

- Results observed in the spikes with no log kept, so they are observations to
  repeat and not records: the second-round figures (RSS of 73 MB and 0.4 s to
  map for egui 0.37, 86 and 78 MB for iced open and closed, the tray calls, hide
  and show, and the captures, which later ones overwrote); the socket mode
  `rwxr-xr-x` in a `rwx------` directory; the transient systemd unit run; the
  `cargo deny` license results for every toolkit; the clippy results for egui
  0.37 and iced; and the egui 0.33.3 figures, whose lock file was overwritten at
  0.37. The typing and paste results are kept in the spikes' stdout logs.
- Other ways to hide an egui window on Wayland (minimising, several viewports,
  calling `run_native` again) were not tried, so "egui cannot host the tray" is
  true of the tested approach only.
- The Tauri fallback has no hide-to-tray, input or clipboard evidence: only its
  tray registration and the deprecation warning were seen. The fonts iced
  bundles were not checked for licenses.
- Sharpness at the scale 1.5 of the second monitor: only the output at scale
  1.2 was captured, and the capture was read by eye.
- Input methods (an IME) and drag and drop, in any toolkit. Typing and
  clipboard were tested with synthetic key events, and the first click into
  the iced field only focused its window.
- Tray clicks from a real bar. Activation and menu entries were called over
  D-Bus, which is the interface a bar uses, not through a bar.
- A screen reader. egui ships AccessKit and iced does not, and neither was
  tried with a screen reader.
- CJK text. This machine has no Japanese font, so neither toolkit's fallback
  was seen.
- The systemd user unit was tested as a transient unit tied to
  `graphical-session.target`, where `WAYLAND_DISPLAY` was already imported into
  the user manager. Whether other Hyprland setups import it is unknown.
- Posting notifications from a headless `bifrostd` on macOS and Windows, where
  the issue says an application identity (a signed bundle on macOS) is needed.
- The Tauri tray went through `libayatana-appindicator`, which warned that it
  is deprecated, so the Tauri fallback tray is not settled either.
- Every claim about macOS and Windows: tray crates, the named pipe through
  `interprocess`, the macOS socket location, the default access of a Windows
  named pipe and the packaging. Only a Linux target was built.
- The Slint license terms, read from their text only.
- The first-round figures (window, memory, Wayland and tray registration) were
  read from `hyprctl -j clients`, `/proc/<pid>/status` and `busctl --user
  get-property org.kde.StatusNotifierWatcher /StatusNotifierWatcher
  org.kde.StatusNotifierWatcher RegisteredStatusNotifierItems` while each spike
  ran. Those outputs were not kept as logs, so they are observations to repeat,
  not records. Build times were recorded but not compared, because the spike
  builds ran at once, and the RSS is one sample taken at window start.
