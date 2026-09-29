---
id: 19
uid: 6182756f-a13a-4fa6-b61d-fe8fdfc16cc1
title: Site pack format and builder
type: feature
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: pack
effort: l
budget:
- listen
---

## Problem

Serving from a directory means stat calls, open calls and runtime compression on every request.

## Proposal

`liam-pack` writes one file: a header with a format version byte, an index, and the data.

- The index maps path → entry. Each entry holds offsets for its identity, Brotli and gzip variants, plus content-type, ETag and size.
- Data is aligned to 4 KiB so sendfile and mmap work on it.
- Compression happens at build time: Brotli at quality 11 and gzip at level 9. A variant is skipped when it saves less than 5%.
- The ETag is a truncated BLAKE3 of the identity bytes.
- Content-type comes from an extension table.
- The compiled config is stored in the pack, so an image carries a single site artifact.

## Budget impact

`listen`: liamd maps the pack instead of reading it.

## Acceptance criteria

- [ ] Building the same directory twice yields byte-identical packs.
- [ ] Every file in the fixture site round-trips in every variant.
- [ ] A truncated or corrupt pack is rejected at load with an error, never a panic, with a test per corruption case.
- [ ] The format is described in the crate's module docs, with its version byte.
