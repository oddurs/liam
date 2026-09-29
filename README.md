# liam

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

liam is an open-source OS image for serving websites. It is one Rust binary
running as PID 1 on a stripped Linux kernel, built from a single `liam.toml`
into a container, a Firecracker microVM, or a UEFI disk for cloud VMs and bare
metal.

Speed is the point, and it is held to numbers. Each of these budgets gets a CI
gate that fails any pull request that misses it:

| Budget | Promise |
|---|---|
| `boot` | Cold boot to first byte under 25 ms in a Firecracker microVM |
| `listen` | Exec to listening under 5 ms as a container |
| `size` | Base container image under 6 MB |
| `zero-drop` | No failed requests across reload and SIGTERM drain; no refused connections across a liamd crash |
| `no-alloc` | Zero heap allocations per request on the static hot path |

## Status

Pre-alpha. The CLI prints its version and does nothing else yet, and no budget
has been measured. The plan to 1.0 lives in cairn items under `cairn/items/`.

## Install

There are no releases yet. To build from source you need Rust (the toolchain is
pinned in `rust-toolchain.toml` and installed by rustup on first use):

```sh
git clone https://github.com/oddurs/liam.git
cd liam
cargo run -p liam -- --version
```

## Development

```sh
scripts/setup          # once per clone: hooks, toolchain, cairn merge driver
scripts/agent doctor   # verify the checkout
scripts/task check     # format, lint, test, build: exactly what CI runs
```

Every change goes through its own worktree, branch and pull request. See
[CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option. Unless you state otherwise, any
contribution you submit for inclusion is dual-licensed the same way, without
additional terms.
