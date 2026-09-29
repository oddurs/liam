---
id: 78
uid: d4550d21-1fb6-4781-88b3-28fc14f0c950
title: Freeze CLI commands, flags and exit codes
type: chore
status: planned
milestone: v1.0
depends_on:
- 21
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: cli
effort: s
---

## Purpose

Scripts and CI pipelines call `liam`. Breaking them is breaking users.

## Approach

Review every command and flag, and remove or rename what is wrong now, before the freeze. Snapshot-test the `--help` output and the exit codes so a change shows up in review.

## Acceptance criteria

- [ ] `--help` snapshots cover every subcommand.
- [ ] Exit codes are listed in the `--help` footer and tested.
