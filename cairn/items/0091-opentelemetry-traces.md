---
id: 91
uid: 903160e5-1baa-43d5-921b-37623d8f2dcc
title: OpenTelemetry traces
type: feature
status: backlog
milestone: later
depends_on:
- 59
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: ops
effort: m
---

## Problem

Proxied requests cross services. Traces show where the time goes.

## Proposal

OTLP export with W3C trace-context propagation to upstreams, sampled, and off by default.

## Budget impact

`no-alloc` must hold when tracing is off.

## Acceptance criteria

- [ ] Split before it is scheduled.
