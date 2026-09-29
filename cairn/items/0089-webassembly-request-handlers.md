---
id: 89
uid: e3d14212-3097-412b-a5c0-0ec66bf1c309
title: WebAssembly request handlers
type: feature
status: backlog
milestone: later
depends_on:
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: proxy
effort: xl
---

## Problem

Small pieces of dynamic logic (auth checks, rewrites, form handlers) don't justify an app process.

## Proposal

Handlers compiled ahead of time with wasmtime at `liam build`, running on the same core as the request, with a WASI HTTP interface and fuel and memory limits.

## Budget impact

`boot`, `size`: wasmtime is large. Measure it before committing.

## Acceptance criteria

- [ ] Split into a spike and features before it is scheduled.
