---
id: 93
uid: 46773de0-344f-4b3f-8197-583089534c97
title: XDP pre-filter for connection floods
type: feature
status: backlog
milestone: later
created: 2026-09-28
updated: 2026-09-28
priority: p3
area: security
effort: l
---

## Problem

A SYN or UDP flood reaches the kernel's TCP stack before liam can act.

## Proposal

An optional XDP program that drops traffic by rate per source before the stack sees it.

## Budget impact

`boot`: program load time.

## Acceptance criteria

- [ ] Split before it is scheduled.
