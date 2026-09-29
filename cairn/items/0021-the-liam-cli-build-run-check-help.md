---
id: 21
uid: ed564ea7-cf00-481d-900a-63b76f740eb3
title: 'The liam CLI: build, run, check, --help'
type: feature
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: cli
effort: s
---

## Problem

The CLI is the entire user interface in v0.1.

## Proposal

clap, with subcommands `build`, `run`, `check` and `version`. Exit codes: 0 for success, 1 for a user error (bad config, missing file), 2 for an internal error.

## Budget impact

None.

## Acceptance criteria

- [ ] `liam --help` lists every subcommand with one line of help each.
- [ ] `liam check` validates the config without building anything.
- [ ] A test covers each of the three exit codes.
