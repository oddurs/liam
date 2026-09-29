---
id: 5
uid: 84d97294-6f02-474c-88bb-01c8720ef154
key: v1.0
title: The promise
type: milestone
status: planned
depends_on:
- 4
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

The promise. No new surface: a stable config format and CLI, every budget measured on reference hardware and published, releases a stranger can verify, and docs a stranger can deploy from.

## Release gate

- [ ] liam.toml format 1 and the CLI are frozen, and the compatibility policy is published.
- [ ] Every budget is gated on every pull request, and the site shows measured numbers instead of budgets.
- [ ] Releases are reproducible and signed, with an SBOM.
- [ ] A 72-hour soak passes.

## Explicitly not in this milestone

- Any new protocol, target or handler type.
