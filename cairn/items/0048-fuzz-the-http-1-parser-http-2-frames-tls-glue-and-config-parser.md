---
id: 48
uid: e71c831d-b460-4ede-a0be-e78adf76be07
title: Fuzz the HTTP/1 parser, HTTP/2 frames, TLS glue and config parser
type: feature
status: planned
milestone: v0.2
depends_on:
- 18
- 20
- 39
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: test
effort: m
---

## Problem

Parsers facing the internet get attacked. Unit tests only cover the inputs someone thought of.

## Proposal

cargo-fuzz targets for HTTP/1 requests, HTTP/2 frames, the TLS-to-kTLS handoff, `liam.toml`, and pack loading. Each target has a committed seed corpus. A nightly CI job runs each target for 30 minutes. Each crash becomes a bug item with a regression test.

## Budget impact

None.

## Acceptance criteria

- [ ] Five targets exist, each with a seed corpus.
- [ ] The nightly job runs them and keeps the corpus it grows.
- [ ] A planted panic in the HTTP/1 parser is found within the nightly run.
