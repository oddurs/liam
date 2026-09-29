---
id: 76
uid: 4226d056-3038-4e5b-b81b-1e9352efd63d
title: Cloud Hypervisor target
type: feature
status: planned
milestone: v0.4
depends_on:
- 25
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: image
effort: s
---

## Problem

Cloud Hypervisor is the other common microVM monitor.

## Proposal

Emit a Cloud Hypervisor launch config next to the Firecracker one. The kernel boots the same way through PVH.

## Budget impact

`boot`: measured by the same harness.

## Acceptance criteria

- [ ] `cloud-hypervisor` boots the `.fc` artifacts and serves the site.
- [ ] Boot numbers are recorded.
