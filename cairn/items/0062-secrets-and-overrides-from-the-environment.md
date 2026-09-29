---
id: 62
uid: 70156628-15f6-4de0-9bcd-6ccc9973ef01
title: Secrets and overrides from the environment
type: feature
status: planned
milestone: v0.3
depends_on:
- 20
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: config
effort: m
---

## Problem

One image should run in staging and production, and secrets do not belong in images.

## Proposal

Any key can be overridden with `LIAM_<SECTION>_<KEY>`. Values can be `file:/path` references, read at start. Overrides are validated at start with the same error format as the build.

## Budget impact

`boot`: overrides are read once at start. Keep it cheap.

## Acceptance criteria

- [ ] An override and a `file:` reference each have a test.
- [ ] An invalid override fails start with the build error format, naming the variable.
- [ ] Secret values never appear in logs, including in errors.
