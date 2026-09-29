---
id: 24
uid: 77c2a41f-922c-444f-be93-c7c083ae2c78
title: Serve static files from the pack
type: feature
status: planned
milestone: v0.1
depends_on:
- 18
- 19
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: pack
effort: m
budget:
- no-alloc
---

## Problem

The pack exists so static responses are as cheap as the kernel can make them.

## Proposal

Map the pack read-only.

- Resolve path → entry, with `index.html` for directories.
- Negotiate `Accept-Encoding` (br, then gzip, then identity), honouring q-values including `q=0`.
- Answer `If-None-Match` with 304.
- Serve a custom 404 when configured.
- `spa_fallback` serves the index for paths without an extension and never for missing assets.
- Send bodies with sendfile from the pack fd at the entry's offset.

## Budget impact

`no-alloc`: this is the hot path.

## Acceptance criteria

- [ ] A table test covers encoding negotiation, including `q=0` and a missing header.
- [ ] A matching ETag returns 304 with no body.
- [ ] The trailing-slash rule is decided, written down, and tested.
- [ ] A test shows `spa_fallback` never answers `/missing.js`.
- [ ] Response bodies never pass through userspace, shown by an io_uring op counter or strace.
