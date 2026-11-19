# Architecture decision records

An ADR records one decision that shaped how bifrost is built: the context, the
decision, its consequences and the options that were rejected. The
[product specification](../SPEC.md) says what bifrost does, and an ADR says why
it is built the way it is.

- A record is a file named `NNNN-short-title.md`, numbered in order and never
  renumbered.
- Copy [template.md](template.md) and fill every section.
- A record is not edited after it is accepted, except for its status. A later
  record supersedes it, and both link to each other.
- Status is `proposed`, `accepted` or `superseded by NNNN`.
- A record is added with the first decision that needs one, usually from a
  `type:spike` issue. None of these rules has a check: all are `review-only`.

## Index

| Record | Decision | Status |
|--------|----------|--------|
| [0001](0001-process-model.md) | Process model, local IPC, UI toolkit and first Linux release | accepted |
| [0002](0002-trust-and-pairing.md) | Trust model, pairing and device keys | proposed |
| [0003](0003-protocol-and-transport.md) | Wire protocol, transport, encoding and payloads | proposed |
