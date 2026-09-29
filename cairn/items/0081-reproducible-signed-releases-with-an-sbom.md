---
id: 81
uid: 5bd69436-46fc-4fa0-90e3-b0a5bf30b9da
title: Reproducible, signed releases with an SBOM
type: chore
status: planned
milestone: v1.0
depends_on:
- 27
- 28
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: security
effort: m
---

## Purpose

People running liam as their whole OS need to verify what they are running.

## Approach

Reproducible builds for the CLI, kernel, initramfs and OCI images. Keyless cosign signatures, SLSA provenance, and a CycloneDX SBOM covering the Rust crates and the kernel config.

## Acceptance criteria

- [ ] An independent rebuild from the tag matches the release checksums.
- [ ] `cosign verify` succeeds on every artifact, with the command documented.
- [ ] An SBOM is attached to each release.
