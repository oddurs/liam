---
id: 2
uid: d5522663-d7ff-40f1-9d5d-4219582a3c76
key: v0.2
title: HTTPS and containers
type: milestone
status: planned
depends_on:
- 1
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

Serve a static site directly to the internet over HTTPS with HTTP/2, as an OCI container on Docker or Kubernetes, or as the microVM from v0.1.

## Release gate

- [ ] `docker run` of a built image serves a multi-domain site over TLS 1.3 and HTTP/2, with kTLS active.
- [ ] Parsers are fuzzed nightly, and every limit has a test.
- [ ] The listen and size gates pass on every pull request.

## Explicitly not in this milestone

- Proxying, app supervision, config reload (v0.3).
- ACME certificates (v0.4); certificates come from files.
