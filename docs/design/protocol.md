# Protocol outline

This outline describes how two bifrost devices talk. The reasons are in
[ADR 0002](../adr/0002-trust-and-pairing.md) (trust and pairing) and
[ADR 0003](../adr/0003-protocol-and-transport.md) (protocol and transport).
Nothing here is built yet. Message names and fields are planned, and the Rust
types will be the schema once they exist.

## Layers

| Layer | What it is |
|-------|------------|
| Tunnel | Tailscale WireGuard, MTU 1280. Bifrost does not configure it. |
| Transport | QUIC through quinn, one connection per pair of peers. |
| Security | TLS 1.3 through rustls. Each side presents an RFC 7250 raw public key (Ed25519 `SubjectPublicKeyInfo`), checked against the pins. |
| Channel selection | ALPN `bifrost/1` for sessions, `bifrost-pair/1` for pairing. |
| Framing | On a bidirectional stream: a big-endian `u32` length, then one CBOR map. A frame is at most 64 KiB. |
| Messages | A CBOR map with a `t` key holding the message kind. Receivers ignore unknown fields. |

## Device identity and state

- The identity is the device's Ed25519 public key, as a `SubjectPublicKeyInfo`.
  It is shown as a fingerprint: the first 16 hex digits of BLAKE3 over the
  `SubjectPublicKeyInfo`.
- In v1 the private key is a file of mode `0600` in bifrost's state directory,
  and `bifrostd` refuses to load it with any other mode.
- The pinned peers live in the same directory, one record per peer: the full
  key, plus a name that is a label only.

## Session

1. The dialling side opens a QUIC connection with ALPN `bifrost/1`. Both
   verifiers accept only pinned keys. When the server refuses a key, the
   client sees a TLS alert 49 on first use, and reports "not trusted by the
   peer".
2. The dialling side opens the first bidirectional stream. That stream is the
   control stream, called stream 0, and it gets a raised priority.
3. Both sides send `Hello` at once.
4. A different `major` closes the connection with application error 1,
   "version mismatch".
5. Otherwise the session's capabilities are the intersection of the two sets.
   A message kind outside it is a protocol error.

```text
Hello { t: "hello", major: u16, caps: [text] }
```

Capability names are planned and may change, for example `clipboard.text`,
`clipboard.image`, `notify.mirror`, `notify.capture`, `file.v1`, and
`file.resume` later. A device leaves out what its system can't do: a Mac
doesn't advertise `notify.capture`.

If both peers dial at once, two connections exist. Proposed, not decided: the
connection opened by the side with the lower fingerprint survives, and the
other one is closed.

## Pairing

Device A runs `bifrost pair`, which opens the window: A prints a 6-digit code,
valid for 120 seconds, and accepts ALPN `bifrost-pair/1`. B connects with that
code.

1. The TLS handshake runs with raw public keys, and neither side checks the
   other's key yet.
2. B sends `PairStart { spake2 }` and A answers `PairReply { spake2 }`. Both
   derive the same secret if and only if the codes match.
3. Each side sends `PairConfirm { mac }`. The MAC is BLAKE3 keyed with a key
   derived from the PAKE secret, over the TLS exporter
   (`EXPORTER-bifrost-pair`, 32 bytes), then A's key, then B's key. A checks
   B's MAC first.
4. Planned, from the prototype: `PairDone`. Each side keeps the other's key as
   a pending pin and commits it only on `PairDone`, or on the first good
   session. That way a lost last message does not leave only one side paired.

The first connection that completes TLS on the pairing ALPN burns the code,
whatever happens next. A wrong code closes the window.

## Payloads

1. The sender puts an `Offer { id, name, size, blake3, mime }` on stream 0.
   The receiver uses only the base name of `name`.
2. The receiver answers `Accept { id }`, or declines.
3. The sender opens a unidirectional stream that carries the 8-byte `id`, then
   the bytes.
4. The receiver writes into a temp file next to the target, hashing as it
   goes, and refuses more bytes than `size`. At the end it checks the size and
   the hash, syncs the file, renames it into place, and sends
   `Done { id, ok, detail }`.
5. On any error, or when the connection is lost, the temp file is deleted.
   There is no resume in v1.

Clipboard content above a threshold that is not decided yet also travels as an
Offer.

## Unpairing

`Unpaired {}` on stream 0, sent if the peer is reachable. The receiver removes
the pin. A refused handshake from a former peer marks it "unpaired by remote".

## Transport settings

- **Flow control:** the stream and connection windows are raised well above
  quinn's defaults (the prototype used 32 and 64 MiB).
- **Congestion control:** cubic.
- **MTU:** quinn's defaults (initial 1200, discovery on), never higher.
- **Idle timeout and keep-alive:** set on purpose. The idle timeout is how a
  peer that died without closing is noticed.
- **Port:** a fixed UDP port, chosen in
  [#22](https://github.com/jaoppb/bifrost/issues/22). A restricted tailnet
  needs a grant that allows UDP to it.
