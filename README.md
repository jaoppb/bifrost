# bifrost

Peer-to-peer clipboard sync, notification sharing and file transfer across
Linux, Android, macOS and Windows, over Tailscale.

## Status

Bifrost is pre-alpha. The repository holds the development environment, the
quality gates and the workflow rules; no feature is built yet, so there is
nothing to install or run. The first target is Linux on Hyprland (milestone
`v0.2.0`).

## Platforms

Every cell is planned. None is built. The
[product specification](docs/SPEC.md) is the source of this table and explains
each limit.

| Platform | Clipboard | History integration | Notifications | File transfer | Milestone |
|----------|-----------|---------------------|---------------|---------------|-----------|
| Linux (Hyprland first) | planned (GNOME not planned) | planned (Vicinae) | planned | planned | `v0.2.0` |
| Android | planned (limited: sends on demand) | not planned (open) | planned | planned | `v0.3.0` |
| macOS | planned | not planned (open) | planned (shows only, captures none) | planned | `v0.4.0` |
| Windows | planned | not planned (open) | planned | planned | `v0.5.0` |

Devices reach each other over [Tailscale](https://tailscale.com/). The
milestones and their scope are listed in `tools.toml`.

## Quickstart (development)

Install [Nix](https://nixos.org/) with flakes enabled, then:

```bash
nix develop     # or `direnv allow`, to enter the dev shell automatically
just setup      # install the git hooks
just check      # format, lint and test
```

[CONTRIBUTING.md](CONTRIBUTING.md) covers the workflow, and
[AGENTS.md](AGENTS.md) holds the rules every change follows.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT), at your option.
