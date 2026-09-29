---
id: 37
uid: 438ac892-b9d2-4802-8c0c-74245737abb3
title: TLS 1.3 with rustls, record layer in the kernel
type: feature
status: planned
milestone: v0.2
depends_on:
- 18
- 36
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: tls
effort: l
budget:
- no-alloc
---

## Problem

v0.2 faces the internet directly, so it needs TLS, and TLS must not cost the static path its zero-copy.

## Proposal

- rustls does the handshake: TLS 1.3, plus TLS 1.2 with ECDHE and AEAD suites only.
- ALPN offers `h2` and `http/1.1`.
- After the handshake, the keys go to the kernel and responses use sendfile.
- When kTLS is unavailable, fall back to userspace rustls and log it once.
- Session tickets rotate on a schedule.

## Budget impact

`no-alloc` on the static path after the handshake.

## Acceptance criteria

- [ ] `/proc/net/tls_stat` shows TX and RX offload on a served connection.
- [ ] The fallback is tested by disabling kTLS in the test kernel.
- [ ] testssl.sh reports no weak protocols or ciphers.
- [ ] The handshake path is covered by the fuzzing item's targets.
