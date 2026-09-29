**liam 1.0** builds a `liam.toml` and a site directory into a container, microVM, or UEFI image that serves static and proxied traffic over HTTPS within published, measured budgets.

**For** engineers who deploy websites and small services onto Firecracker hosts, Kubernetes, cloud VMs, or their own servers, and care how fast it boots and serves.

**Not in 1.0:**

- A general-purpose Linux. There is no shell, no SSH, and no package installs at runtime. You debug with logs, metrics, and a separate debug build.
- A language runtime manager or build system for your app. liam supervises one app process that you bring as a self-contained executable.
- A control plane. There is no fleet management, autoscaling, or dashboard. `liam deploy` puts an image on a target; scheduling belongs to something else.
- HTTP/3 and WebAssembly handlers. Both come after 1.0; see `later`.

**Budgets**, each enforced by a CI gate (`cairn list -t budget`):

| Budget | Promise |
|---|---|
| `boot` | Cold boot to first byte under 25 ms in a Firecracker microVM |
| `listen` | Exec to listening under 5 ms as a container |
| `size` | Base container image under 6 MB |
| `zero-drop` | No failed requests across reload and SIGTERM drain; no refused connections across a liamd crash |
| `no-alloc` | Zero heap allocations per request on the static hot path |

Releases ship when their gate passes, not on a date.
