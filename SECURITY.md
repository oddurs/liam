# Security

## Supported versions

liam is pre-alpha and has no releases yet. Once it does, only the latest release
receives fixes until 1.0.

## Reporting a vulnerability

Report privately through GitHub Security Advisories:

<https://github.com/oddurs/liam/security/advisories/new>

Please do not open a public issue. Include the version or commit, the target
(container, Firecracker, cloud VM, bare metal), and the smallest reproduction you
have.

Expect an acknowledgement within a week and an assessment within two. Fixes ship
in the next release, and the advisory is published with credit unless you would
rather it was not.

## What counts

liam is meant to face the internet directly, so these are vulnerabilities:

- Anything a remote client can send that crashes liamd, exhausts its memory or
  file descriptors, or holds a core past the configured limits.
- Anything that makes liam serve a file outside the site pack, or a response for
  a host it was not configured for.
- TLS weaknesses: downgrade, a certificate served for the wrong name, key
  material exposed in logs or metrics.
- A way to run code in the image, or to change what it serves, without building
  and deploying a new one.

liam ships a Linux kernel, so a kernel CVE that is reachable through the enabled
configuration is also liam's to fix.
