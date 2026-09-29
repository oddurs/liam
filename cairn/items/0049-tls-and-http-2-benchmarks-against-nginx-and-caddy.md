---
id: 49
uid: 5b8ee25b-3d2d-45fc-8e8e-5dc460015aa0
title: TLS and HTTP/2 benchmarks against nginx and Caddy
type: feature
status: planned
milestone: v0.2
depends_on:
- 37
- 39
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: bench
effort: m
---

## Problem

A speed pitch needs comparisons people can rerun, on the same hardware, with the other side configured fairly.

## Proposal

`bench/compare` runs oha and h2load against liam, nginx and Caddy on one host for four cases: a small file, a large file, many assets over HTTP/2, and new TLS handshakes per second. The configs for the other servers are committed and tuned from their own docs.

## Budget impact

None. Informational.

## Acceptance criteria

- [ ] Results are in `bench/results` with the raw output and host metadata.
- [ ] The nginx and Caddy configs are committed, with the source of each tuning setting.
