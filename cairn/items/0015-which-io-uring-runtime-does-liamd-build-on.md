---
id: 15
uid: 5b1b67ff-aa53-4713-ba8e-7e68c2ffc77c
title: Which io_uring runtime does liamd build on?
type: spike
status: planned
milestone: v0.1
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: io
effort: m
budget:
- listen
- no-alloc
---

## Question

Should liamd use a thin runtime of our own on the `io-uring` crate, or adopt monoio, compio or glommio?

## Why it blocks

Every liamd crate sits on this choice. HTTP/2 (`h2`) and later QUIC (`quinn`) expect tokio-style I/O traits, so the choice also decides how those integrate. A runtime that cannot host them costs a rewrite in v0.2.

## Timebox

Three days.

## Approach

1. On each candidate, write a thread-per-core HTTP/1.1 keep-alive hello-world with `SO_REUSEPORT` and multishot accept.
2. Measure requests per second and p99 on 1 and 4 cores with the same load generator and host.
3. Check each candidate for: sendfile or splice support, registered buffers, kTLS compatibility, maintenance activity, size of its unsafe surface, and whether `h2` runs on it through a compat layer.
4. File a decision item with the choice.

## Acceptance criteria

- [ ] Benchmark numbers for each candidate, with the exact commands.
- [ ] `h2` compatibility demonstrated or ruled out for the chosen runtime.
- [ ] A decision item records the choice.
- [ ] Closed with `--result` naming the runtime and the reason.
