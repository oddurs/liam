---
id: 28
uid: daeeec23-b26a-4fa6-b392-d8d74893ea23
title: Release workflow and one-command install
type: chore
status: planned
milestone: v0.1
depends_on:
- 13
- 27
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: infra
effort: m
---

## Purpose

A stranger has to be able to install liam in one command.

## Approach

Pushing a `v*` tag builds the CLI for x86_64 and aarch64 Linux and for aarch64 macOS (build works on macOS; run needs Linux), and attaches the binaries with checksums to a GitHub release. Check whether the `liam` name is free on crates.io and decide the install path: `cargo binstall`, a release install script, or both.

## Acceptance criteria

- [ ] Tagging `v0.1.0-rc.1` produces a release with binaries and checksums for all three targets.
- [ ] The install command works on a clean Ubuntu VM and a clean macOS machine.
- [ ] The crate name decision is recorded as a note on this item.
