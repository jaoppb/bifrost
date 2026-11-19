# 0002: Trust model, pairing and device keys

Status: proposed

Issue: [#21](https://github.com/jaoppb/bifrost/issues/21)

## Context

Two bifrost devices have to decide whether to trust each other before any
clipboard item, notification or file crosses between them. The specification
says devices are paired explicitly and must be on the same tailnet
([docs/SPEC.md](../SPEC.md)), and [ADR 0001](0001-process-model.md) requires
pairing to work as a command line flow in `v0.2.0`, with a QR code later on
Android.

WireGuard already encrypts traffic between tailnet nodes, and on desktops the
Tailscale LocalAPI `WhoIs` call maps a connecting address to a node and a user.
Two facts make that insufficient as the trust root:

- A tailnet can be shared with other people, and every node on it passes
  WireGuard. Being on the tailnet says nothing about being one of the user's
  bifrost devices.
- On Android, another app probably cannot reach the Tailscale app's LocalAPI
  ([#22](https://github.com/jaoppb/bifrost/issues/22)), so a check built on
  `WhoIs` has nothing to call there.

The options were weighed in a review on 2026-11-15, and the throwaway prototype
tested them (see Evidence).

## Decision

**Device keys.** Each device has its own long-term keypair. A device's identity
is its public key and nothing else: no tailnet node ID, hostname or generated
UUID. In v1 the private key is a file of mode `0600` in bifrost's state
directory, written only by `bifrostd`, and the pinned peer keys sit next to it.
A lost key means a new device that pairs again. The OS keyring becomes the
default later, with an automatic migration from the file (planned).

**Trust.** Pairing pins the peer's public key, and every connection is mutually
authenticated against the pins, the same way on every system. Tailscale only
carries the traffic: tailnet identity is not a trust signal. Trust is pairwise:
each device keeps its own list of pinned peers, and nothing propagates between
devices.

**Pairing.** Device A runs `bifrost pair` and shows a one-time code, valid for
about two minutes. The user types it on device B. A PAKE run over the
connection authenticates the exchange of public keys with that code, so a
man-in-the-middle gets one online guess. A code allows one attempt and is
burned on failure. On Android, a QR code carries the same code plus A's
address.

**Unknown peers.** A connection whose key is not pinned is refused at the
handshake, unless a pairing window is open. The window accepts only the PAKE
exchange. There is no inbound pairing request or prompt.

**Unpairing.** `bifrost unpair` deletes the local pin right away; that part
does not depend on the peer. A reachable peer gets an Unpaired message and drops
its own pin. An unreachable one is refused on its next connection and marks the
device "unpaired by remote".

## Consequences

- One trust check works on all four systems and survives a node re-auth, a new
  tailnet or a move off Tailscale, because it doesn't depend on Tailscale.
- Bifrost runs its own key management and a second encryption layer on top of
  WireGuard.
- A user with four devices runs up to six pairings, and a lost device has to be
  unpaired on each of the others. Until group trust exists, a forgotten device
  stays trusted.
- Group trust can be added later as a negotiated capability plus a migration of
  the peer records ([ADR 0003](0003-protocol-and-transport.md)). v1 reserves no
  field for it.
- In v1 the private key is protected only by file permissions, so any process
  running as the user can read it.
- The address B connects to during pairing comes from
  [#22](https://github.com/jaoppb/bifrost/issues/22).

## Rejected options

- **Tailnet identity only.** Record the peer's node and user, and check
  `WhoIs` on every connection. Rejected because it has no check to run on
  Android, admits anyone who shares the tailnet, and loses the pairing when a
  node is re-created.
- **Pinned key and `WhoIs` together.** Two checks that behave differently on
  each system, with no good rule for a node ID that changes while the key still
  matches.
- **Group trust in v1** (pair once, join a "my devices" group). One ceremony per
  device and one-step revocation, but it needs a replicated, signed membership
  log with conflict rules, and a compromised member could add devices.
- **A reserved endorsement field for later group trust.** A field nobody tests
  until it's needed, whose shape would be guessed now.
- **Compare a short authentication string.** Two confirmations, and users
  confirm strings they haven't read.
- **Exchange fingerprints by hand.** Strong, but painful on Android, and done
  twice for each pair.
- **Inbound pairing requests from unknown devices.** Anyone on the tailnet
  could spam prompts, and a headless daemon has nowhere good to show them.
- **Unpair that waits for the peer.** It can't remove a lost or dead device.
- **OS keyring in v1.** On Hyprland a Secret Service provider is not guaranteed
  to run, so a headless daemon could fail to start.

## Evidence

A throwaway prototype was built on 2026-11-15, outside the repository, as one
binary with a listener role and a connector role. It used Rust 1.96.0, quinn
0.11.12, rustls 0.23.46 (ring provider, TLS 1.3 only), spake2 0.4.0, ciborium
0.2.2 and blake3 1.8.7. Each result below was seen on loopback. The same
binary, as a static musl build, was then run between two hosts on one tailnet,
over a direct path on Wi-Fi. There, pairing took 0.16 s including the PAKE.
Reconnecting with the pinned keys worked with either host dialling, and six
1 GiB transfers arrived with verified hashes.

- **Raw public keys.** rustls 0.23 supports RFC 7250 raw public keys, and they
  work through quinn 0.11 unchanged. Each side presents its Ed25519 key as a
  `SubjectPublicKeyInfo`, and nothing else: there is no X.509 certificate, no
  certificate generation and no host name check. The pinned identity is that
  `SubjectPublicKeyInfo`. One verifier type serves as both the client and the
  server certificate verifier, and it accepts a key only if it is pinned.
- **Key file.** The key file and the pin file were created with mode `0600`,
  and the prototype refuses to load a key file with any other mode.
- **Pairing.** A 6-digit code, valid for 120 s, authenticated a SPAKE2
  exchange. Each side then sent a key-confirmation MAC over the TLS exporter,
  the listener's key and the connector's key. This binds the PAKE to that TLS
  session and to those two keys. Pairing used a separate ALPN
  (`bifrost-pair/1`), and normal sessions used `bifrost/1`.
- **Wrong code.** A wrong code was refused and the window closed: a retry with
  the right code found nothing listening, and no pin was written.
- **Refusing unpinned keys.** An unpinned key was refused at the handshake in
  both directions, with TLS alert 49.
- **Crates and lints.** `cargo deny check` passed with the repository's
  `deny.toml` (advisories, bans, licenses and sources), and the crate built
  under `forbid(unsafe_code)` and `forbid(missing_docs)`. The PAKE
  alternatives `pake-cpace` 0.1.7, `cpace` 0.1.0 and `pakery-cpace` 0.6.2 also
  pass the license check, but none was integrated.

The prototype also showed four details the decision above has to absorb:

- **Pin commit asymmetry.** The listener pins after checking the connector's
  MAC, before the connector has checked the listener's. If the last message is
  lost, only one side has pinned. Pairing needs a final acknowledgement, or a
  pending pin that the first good session confirms.
- **Anyone can burn the code.** During the window, any tailnet node can burn
  the code with one wrong guess. That blocks pairing (one guess in 10⁶ cannot
  break in), and the user has to start pairing again.
- **The window weakens the handshake refusal.** rustls gives the certificate
  verifier no ALPN, so a daemon that keeps serving pinned peers while a window
  is open has to let unknown keys through the handshake. It then closes any
  `bifrost/1` connection whose key is unpinned, which happens after the
  handshake, not during it.
- **A refused client looks connected at first.** In TLS 1.3 the client
  finishes before the server checks the client's key. A refused client sees
  its connect succeed, then "connection lost, TLS alert 49" on first use. The
  CLI has to report that as "not trusted by the peer".

Logs: `proto-01` to `proto-12` and `PROTO-REPORT.md` in the spike directory,
kept outside the repository.

## Unverified

- What happens on a Hyprland setup with no Secret Service provider once the
  keyring becomes the default.
- Everything about Android key storage (Keystore or app-private storage).
- spake2 0.4.0 is an old pre-1.0 release, and it pulls duplicate `rand_core`
  and `getrandom` versions. `pakery-cpace` was checked only for its license.
  The PAKE crate is chosen in the pairing story.
- Two pairings started at once, and two peers dialling each other at once
  (one connection per pair of peers was not enforced).
