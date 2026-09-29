---
id: 36
uid: 1564d00f-a078-4196-9c09-ee9a94dbb7c1
title: 'kTLS with io_uring and sendfile: what works on the LTS kernel?'
type: spike
status: planned
milestone: v0.2
depends_on:
- 15
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: tls
effort: m
budget:
- no-alloc
---

## Question

Can rustls hand both TX and RX keys to kernel TLS for TLS 1.3 with AES-GCM and ChaCha20-Poly1305 on our kernel? Does sendfile of a pack range over a kTLS socket work through io_uring? How are key updates, `close_notify` and alerts handled once the kernel owns the record layer?

## Why it blocks

kTLS is how HTTPS static responses avoid copying through userspace. If a piece is missing, the TLS design changes.

## Timebox

Three days.

## Approach

Prototype with rustls secret extraction and the `ktls` crate on the chosen runtime. Measure throughput and CPU against plain userspace rustls. Exercise a key update, `close_notify`, and a fatal alert.

## Acceptance criteria

- [ ] A table of what works per cipher suite and direction, with the kernel options it needs added to the kernel config fragment.
- [ ] Throughput and CPU for kTLS against userspace rustls.
- [ ] Closed with `--result`.
