# 0003: Wire protocol, transport, encoding and payloads

Status: proposed

Issue: [#21](https://github.com/jaoppb/bifrost/issues/21)

## Context

Two paired devices ([ADR 0002](0002-trust-and-pairing.md)) exchange small
control messages (clipboard items, notifications, pairing and capability
messages) and large payloads (files, big clipboard images). The channel runs
inside a Tailscale WireGuard tunnel, whose MTU is 1280. Every peer runs the
same Rust core, and the Android app is a Kotlin shell around it
([#23](https://github.com/jaoppb/bifrost/issues/23)), so no second language
implements the wire. Devices update at different times: a phone that hasn't
updated talks to a desktop that has.

KDE Connect shows that a control channel of typed packets with capability
lists, plus a separate connection for payloads, works well. Being wire
compatible with it would give ready-made clients, but its trust model is
trust-on-first-use with an accept prompt, which ADR 0002 rejects, and its
discovery is UDP broadcast, which doesn't cross a tailnet.

## Decision

**Protocol.** A new protocol of bifrost's own. It borrows KDE Connect's shape
(typed packets, capability lists, a separate stream for payloads) but not its
wire format.

**Transport.** QUIC through `quinn` and `rustls`, one connection per pair of
peers. Stream 0 is the control channel, and each payload travels on a stream of
its own, so a large file doesn't delay clipboard messages. A custom `rustls`
verifier checks the peer's key against the pins of ADR 0002, in both
directions.

**Encoding.** Control messages are CBOR. A receiver ignores fields it doesn't
know, so a new optional field needs no version bump. The schema is the Rust
types, documented in [the protocol outline](../design/protocol.md).

**Versions and capabilities.** The first message on stream 0 is a Hello
carrying the protocol major version and a set of named capabilities (for
example `clipboard.text`, `notify.mirror`, `file.v1`). A different major closes
the connection with a "version mismatch" error the user sees. Within the same
major, each side sends only message kinds in the capability set both of them
advertise, and anything else is a protocol error. Platform limits show up as
capabilities a device doesn't advertise: a Mac doesn't advertise
`notify.capture`.

**Large payloads.** The sender puts an Offer on stream 0 with the name, size,
BLAKE3 hash and mime type. After the receiver's Accept, the bytes go on a new
stream into a temp file, which is hash-checked and then renamed into place
atomically. A dropped connection fails the transfer and deletes the partial
file. Resume is a later `file.resume` capability (planned).

## Consequences

- Clipboard and notification messages keep flowing during a large transfer,
  and QUIC connection migration covers many network switches on a phone.
- Tailnet ACLs must allow UDP on the bifrost port. A tailnet that allows only
  TCP breaks the transport, and there is no TCP fallback.
- Only bifrost clients can connect: no existing KDE Connect client works.
- Adding an optional field is free; removing a field or changing its meaning
  needs a new capability or a new major version.
- A transfer interrupted beyond what connection migration covers starts over
  from zero.
- The crypto provider for `rustls` is ring. `aws-lc-rs` also passes the
  license check, but it builds AWS-LC from C with cmake, which is harder on
  musl, Android and Windows.
- Bifrost sets quinn's transport itself:
  - larger stream and connection flow-control windows than quinn's defaults;
  - cubic congestion control;
  - quinn's MTU defaults, never an initial MTU above 1200;
  - an idle timeout and keep-alive chosen on purpose, since the idle timeout
    is how a peer that died without closing is noticed.
- CBOR byte fields are byte strings (`serde_bytes` with ciborium), never
  arrays of integers. minicbor is ruled out: its BlueOak-1.0.0 license is not
  on the policy's allow list.
- A timed-out handshake with a peer that `tailscale ping` reaches points at the
  tailnet policy, and the CLI says so.
- Small messages during a large transfer can wait a few hundred milliseconds on
  a busy link, so nothing user-facing assumes a tighter bound.

## Rejected options

- **KDE Connect wire compatibility.** It conflicts with the trust model of
  ADR 0002, ties bifrost to its packet schema and limits, and its discovery
  doesn't cross a tailnet.
- **TCP with TLS and length-prefixed framing.** Simple and easy to debug, but
  bifrost would do its own multiplexing, and each payload connection would pay
  a new TLS handshake.
- **gRPC over HTTP/2 (tonic).** Heavy (hyper, tower, prost, and protoc in the
  Nix shell), with head-of-line blocking over TCP.
- **Protobuf.** The best-known rules for schema evolution, but it needs protoc
  and generated code that must pass `forbid(missing_docs)`, and no second
  language needs the schema.
- **postcard.** Not self-describing: every field change breaks old peers.
- **Capabilities without a version, or a version without capabilities.** The
  first gives no clean "too old" cut-off; the second forces all devices to
  update together and has no way to express platform limits.
- **Resumable transfers in v1.** Persisted partial state and cleanup rules do
  not fit the first release.
- **Chunks on the control channel.** Chunks would compete with clipboard
  messages, which defeats separate streams.

## Evidence

Two throwaway programs were built on 2026-11-15, outside the repository. One
was the protocol prototype of [ADR 0002](0002-trust-and-pairing.md). The other
was a QUIC check, run between two hosts on the same tailnet. Both hosts were on
Wi-Fi on the same LAN, and Tailscale reported a direct path, not a DERP relay.
Both programs used quinn 0.11.12 and rustls 0.23 with the ring provider.

**Prototype, on loopback.**

- **Hello.** A Hello carrying the major version and capabilities was exchanged.
  A Hello with an extra unknown field was accepted, and a faked major of 2
  closed the connection with "version mismatch".
- **CBOR.** A CBOR control message went through. Framing was a big-endian u32
  length, then one CBOR map with frames capped at 64 KiB. With ciborium,
  `Vec<u8>` becomes a CBOR array of integers unless it is marked
  `serde_bytes`, so the schema has to pin byte strings.
- **File.** A 1 GiB file went through Offer, Accept and its own stream. It was
  checked with BLAKE3, written to a temp file, renamed into place, and
  identical by `cmp`.
- **Dropped transfer.** Killing the sender with SIGKILL at 678 MB failed the
  transfer and deleted the partial file. The receiver noticed only when the
  30 s idle timeout fired. A graceful close is noticed at once.
- **Flow control.** With quinn's default windows, one stream reached about
  30–60 MiB/s even on loopback. With a 32 MiB stream window and a 64 MiB
  connection window it reached about 120–290 MiB/s. The host was heavily
  loaded, so these are lower bounds.

**Over the tailnet.** Logs are `udp-*` and `UDP-REPORT.md`.

- **Plain UDP.** Datagrams on a fixed high port arrived in both directions
  with no loss.
- **QUIC handshake.** 3–12 ms, and a 64-byte echo took about 4 ms when idle.
- **Bulk on one stream.** 200–400 Mbit/s, which is 35–60 % of kernel TCP over
  the same tunnel. The default 1.25 MB stream window capped the slow direction
  at about 70–90 Mbit/s once the RTT under load rose to about 115 ms. An 8 MB
  window doubled it.
- **Congestion control.** BBR was faster but lost far more packets, and quinn
  marks it as experimental. Cubic stays.
- **Small messages during a transfer.** A 64-byte echo every 50 ms on a second
  stream was never starved. On a quiet link it took 8–11 ms p50 and 20–50 ms
  p99. On a busy Wi-Fi link p99 reached 100–480 ms, from queueing below QUIC.
  Raising the control stream's priority helped only some of the time.
- **MTU.** QUIC works with the 1280 tailnet MTU on quinn's defaults (initial
  MTU 1200, discovery on), which settled at 1230 bytes. That is below the
  ceiling over IPv6 inside the tunnel (1232). An initial MTU set too high was
  recovered by black-hole detection, at a throughput cost.
- **Tailnet policy.** The node's compiled packet filter, read with
  `tailscale debug netmap`, allows TCP, UDP and ICMP between the user's own
  devices: the default policy. A restricted tailnet needs a grant such as
  `{"src": ["autogroup:member"], "dst": ["autogroup:self"], "ip": ["udp:<port>"]}`.
  A rule that allows only TCP on the port silently blocks QUIC, and it shows up
  as a handshake timeout, not as a refusal.
- **Prototype end to end.** The prototype moved a 1 GiB file between the two
  hosts at about 210–440 Mbit/s, counting hashing, the disk write and fsync on
  the receiver. On this path, with an RTT of about 4 ms, raised windows and
  default windows were within noise of each other. The windows matter on
  loopback and once congestion inflates the RTT.
- **Outer network blocking UDP.** In that case Tailscale falls back to a DERP
  relay and QUIC still runs inside the tunnel, only slower. So for reachability
  the risk is the tailnet policy, not the network.

## Unverified

- **A DERP-relayed path, a path through NAT, and a phone on mobile data.**
  Only the best case was measured: a direct path on one LAN. Forcing a relay
  would have meant changing Tailscale or firewall settings.
- **A restricted tailnet policy.** The grant above comes from reading the
  compiled filter and Tailscale's policy syntax, not from a blocked
  experiment.
- **QUIC to the tailnet's IPv6 addresses.**
- **One connection per pair of peers when both dial at once.** It needs a
  tie-break rule, for example the lower key fingerprint wins.
- **Android, macOS and Windows.** Only Linux x86_64 was built and run.
