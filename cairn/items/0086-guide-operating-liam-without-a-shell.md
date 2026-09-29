---
id: 86
uid: 59d899f4-de11-4d5a-bc8c-47eb0e53f3e4
title: 'Guide: operating liam without a shell'
type: docs
status: planned
milestone: v1.0
depends_on:
- 33
- 59
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: docs
effort: s
---

## Reader and question

An operator used to `ssh` and `top` asks how to find out what is wrong with an instance.

## Change

A guide to what exists instead: logs, metrics, the admin port, boot phase timings, the watchdog, and rollback. It includes worked examples of four failures: a crash loop, a slow upstream, certificate expiry, and running out of memory.

## Acceptance criteria

- [ ] Each worked failure is reproduced with a real image, and its steps are copied from that run.
