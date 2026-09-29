---
id: 54
uid: ea81c6c5-9e4b-4281-bfa2-b2462554acd2
title: Reload without a restart
type: feature
status: planned
milestone: v0.3
depends_on:
- 40
- 53
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: config
effort: m
budget:
- zero-drop
---

## Problem

Rotating a certificate or secret should not cost a restart.

## Proposal

A reload is triggered by SIGHUP or the admin socket, for the sources the reload spike accepted. The new state is compiled and validated, then swapped in atomically per core (arc-swap). In-flight requests finish on the old state. A reload that fails validation keeps the old state and logs the error.

## Budget impact

`zero-drop` for reload.

## Acceptance criteria

- [ ] Reloading a rotated certificate under load fails no requests.
- [ ] An invalid new config is rejected, the old one keeps serving, and the error is logged in the build error format.
- [ ] The swap adds no allocation to the hot path.
