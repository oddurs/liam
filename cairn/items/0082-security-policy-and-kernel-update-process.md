---
id: 82
uid: 3e41ebeb-611a-4f52-ac1d-fb2bc29d3b31
title: Security policy and kernel update process
type: chore
status: planned
milestone: v1.0
depends_on:
- 22
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: security
effort: s
---

## Purpose

liam ships a kernel, so kernel CVEs are liam's to answer.

## Approach

A SECURITY.md with private reporting and a response target. An automated pull request bumps the kernel to each new LTS point release, and the budget gates run on it. A patch release goes out within 7 days of a kernel fix that is relevant to the config.

## Acceptance criteria

- [ ] SECURITY.md is published, and private reporting is enabled on the repository.
- [ ] The kernel bump bot opened at least one pull request that merged through the gates.
