---
id: 12
uid: 10481a9f-b883-4165-bd4f-a3236d394e11
title: Scaffold the Cargo workspace and the task seam
type: chore
status: doing
milestone: v0.1
assignee: Oddur Sigurdsson
claimed: 2026-09-28
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: infra
effort: s
---

## Purpose

Nothing can be built, tested or gated until the repository has its shape. Every later item assumes these crates and verbs exist.

## Approach

- A Cargo workspace on edition 2024 with the toolchain pinned in `rust-toolchain.toml`. Crates: `liam-io`, `liam-http`, `liam-pack`, `liam-config`, `liam-init` (bin), `liamd` (bin), `liam` (the CLI bin).
- Release profile: `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `opt-level = 3`. mimalloc as the global allocator in the binaries, because musl's allocator is slow.
- Build `x86_64-unknown-linux-musl` static binaries.
- `scripts/task` with `fmt fmt:check lint test build check`. `lint` is clippy with `-D warnings`.
- `LICENSE-MIT` and `LICENSE-APACHE`.
- `site/` is left as it is. The microsite already lives there.

## Acceptance criteria

- [x] `scripts/task check` passes on the empty workspace.
- [x] A deliberate clippy warning makes `scripts/task lint` exit non-zero.
- [ ] `file` reports the release `liamd` and `liam-init` binaries as statically linked.
- [x] Both license files are present, and every crate declares `MIT OR Apache-2.0`.

## 2026-09-28

Library crates (liam-io, liam-http, liam-pack, liam-config) are not created here: empty crates are dead scaffolding, and each has an item (0017, 0018, 0019, 0020) that creates it with real code. This item builds the two image binaries the static pipeline needs.

## 2026-09-28

mimalloc is the global allocator in liamd only. liam-init barely allocates, and its size budget is 300 KB; the CLI runs on developer machines. The release profile also sets strip = true for the size budget.

## 2026-09-28

Static builds are a separate verb, scripts/task build:static, run by a 'static' CI job on Linux (musl-tools provides musl-gcc for mimalloc's C). It fails unless file(1) reports each binary as statically linked or static-pie linked, and prints each binary's size. It refuses on macOS rather than half-working.

## 2026-09-28

Evidence. 1: scripts/task check green on macOS. 2: a planted clippy::len_zero made scripts/task lint exit 101; removing it, 0. 4: LICENSE-MIT and LICENSE-APACHE present; cargo metadata shows liam, liam-init and liamd all MIT OR Apache-2.0.
