---
id: 42
uid: 319e9390-b733-434b-927b-97f184e349ff
title: Range and conditional requests
type: feature
status: planned
milestone: v0.2
depends_on:
- 24
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: pack
effort: m
---

## Problem

Video, resumable downloads and some CDNs need byte ranges.

## Proposal

A single range returns 206 Partial Content. `If-Range` and `If-Modified-Since` are supported. A request for multiple ranges gets 200 with the full body, which the spec allows. Ranges are served from the identity variant only.

## Budget impact

None.

## Acceptance criteria

- [ ] A table test covers valid, suffix, unsatisfiable (416) and multi-range requests.
- [ ] A range read over kTLS still uses sendfile.
