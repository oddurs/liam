---
id: 12
uid: 10481a9f-b883-4165-bd4f-a3236d394e11
title: Scaffold the Cargo workspace and the task seam
type: chore
status: planned
milestone: v0.1
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

- [ ] `scripts/task check` passes on the empty workspace.
- [ ] A deliberate clippy warning makes `scripts/task lint` exit non-zero.
- [ ] `file` reports the release `liamd` and `liam-init` binaries as statically linked.
- [ ] Both license files are present, and every crate declares `MIT OR Apache-2.0`.
