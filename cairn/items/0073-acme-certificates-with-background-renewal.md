---
id: 73
uid: fcacb2a9-a1be-46a2-9a74-723ffef384f6
title: ACME certificates with background renewal
type: feature
status: planned
milestone: v0.4
depends_on:
- 38
- 72
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: tls
effort: l
---

## Problem

Managing certificate files by hand is the thing people most want a server to do for them.

## Proposal

`[tls] acme = "letsencrypt"` issues through the path the ACME spike chose. Renewal starts 30 days before expiry, in the background, and the new certificate is swapped in through reload. A boot always uses the cached certificate.

## Budget impact

`boot`: no network wait.

## Acceptance criteria

- [ ] First issuance and renewal work against Let's Encrypt staging.
- [ ] Boot with a cached certificate makes no ACME call before serving.
- [ ] A failed renewal is retried with backoff and shows in metrics.
