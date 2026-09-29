---
id: 38
uid: 0ffe118b-21cb-4551-a3f3-e0c33760e86c
title: Certificates from files, with SNI across domains
type: feature
status: planned
milestone: v0.2
depends_on:
- 20
- 37
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: tls
effort: m
---

## Problem

A site with several domains needs the right certificate for each, and a bad certificate should fail the build rather than the first visitor.

## Proposal

`[tls] cert` and `key` take PEM paths, or a table per domain. The certificate is chosen by SNI. At build time, liam checks that the key matches the certificate, that the chain is complete, and that the certificate covers its domains, and warns when it expires within 14 days.

## Budget impact

None.

## Acceptance criteria

- [ ] Two domains with separate certificates are each served the right one, in a test.
- [ ] A mismatched key, an incomplete chain and a domain the certificate doesn't cover each fail `liam build` with a clear error.
- [ ] An expiring certificate warns without failing.
