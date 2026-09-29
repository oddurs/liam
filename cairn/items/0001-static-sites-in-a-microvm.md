---
id: 1
uid: b4e943e6-4513-4d35-a2f8-2686e6950fc4
key: v0.1
title: Static sites in a microVM
type: milestone
status: doing
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

Serve a static site from a Firecracker microVM over HTTP/1.1, from cold boot to first byte inside the boot budget, measured in CI. For sites behind a load balancer or CDN that already terminates TLS.

## Release gate

- [ ] A stranger installs liam with one command, runs `liam build` and `liam run` on a Linux host with KVM, and their site is served.
- [ ] The boot gate runs on every pull request, and the latest p99 is under 25 ms.
- [ ] All p0 items in this milestone are done.

## Explicitly not in this milestone

- TLS, HTTP/2, host-based routing (v0.2).
- Container images and macOS `liam run` (v0.2).
- Anything dynamic: proxying, apps, reload (v0.3).
