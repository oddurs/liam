---
id: 97
uid: fcf55ec2-86a4-4d81-9d93-8eb637cc9992
title: Linux lab for development on macOS
type: chore
status: done
milestone: v0.1
created: 2026-10-02
updated: 2026-10-02
closed_at: 2026-10-02
priority: p0
effort: s
area: infra
---

## Purpose

The maintainer works on macOS (Apple silicon). Everything liam ships is Linux: static musl binaries, io_uring, and a kernel image. Without a Linux machine at hand, every Linux-only check (`scripts/task build:static`, io_uring tests, kernel builds) waits on a CI round trip. A local lab makes those a one-line command, and gives parallel agents a shared place to run them without sharing a target directory.

## Approach

- An OrbStack machine named `liam-lab` (Ubuntu, aarch64). OrbStack mounts `/Users` at the same path, so the worktree is visible inside the VM unchanged.
- `scripts/lab <cmd...>` runs a command in the VM at the caller's directory. `CARGO_TARGET_DIR` is set under the VM user's home, derived from the worktree path, so macOS builds, Linux builds and parallel worktrees never share a target directory.
- `scripts/lab setup` provisions idempotently: build-essential, musl-tools, the x86_64 cross gcc and binutils, flex, bison, bc, libelf-dev, libssl-dev, cpio, file, curl, qemu-system-x86; rustup with the toolchain from `rust-toolchain.toml` plus the x86_64 and aarch64 musl targets. Cargo is configured so the x86_64 musl target links with the cross toolchain.
- The VM has no KVM, so nothing here boots Firecracker. That stays on CI.

## Acceptance criteria

- [x] `scripts/lab --help` describes the commands, and a missing `orb` or a missing `liam-lab` machine fails with an error naming the fix.
- [x] `scripts/lab setup` provisions a fresh machine, and a second run succeeds without reinstalling anything.
- [x] `scripts/lab cargo test --workspace` passes in the lab.
- [x] `scripts/lab scripts/task build:static` produces static x86_64 `liamd` and `liam-init`.
- [x] io_uring works in the lab: a program calls `io_uring_setup` and completes a request.
- [x] Two worktrees get different target directories, neither of them the macOS `target/`.
- [x] CONTRIBUTING.md tells macOS contributors how to use the lab.

## 2026-10-02

Cross-compiling C for x86_64 musl on an aarch64 machine: x86_64-linux-gnu-gcc alone fails on mimalloc (it finds the aarch64 glibc headers: bits/libc-header-start.h missing). Ubuntu ships musl for the native architecture only, so setup builds musl 1.2.5 (the version in Rust's bundled x86_64 musl libc.a) from upstream source for x86_64 with --enable-wrapper=gcc, into ~/.local/x86_64-linux-musl. The tarball's sha256 a9a118bb...c75e4 matches Ubuntu's musl_1.2.5.orig.tar.gz on both archive.ubuntu.com and ports.ubuntu.com. Cargo config: linker x86_64-linux-gnu-gcc, CC_x86_64_unknown_linux_musl the musl-gcc wrapper. scripts/task build:static now honours CARGO_TARGET_DIR, which scripts/lab sets.

## 2026-10-02

Evidence 1: scripts/lab --help prints usage, exit 0; no args exits 2. With PATH=/usr/bin:/bin: 'scripts/lab: orb not found. Install OrbStack (brew install orbstack), then: orb create ubuntu liam-lab', exit 1. With a stub orb listing another machine: 'scripts/lab: no OrbStack machine named liam-lab. Create it: orb create ubuntu liam-lab && scripts/lab setup', exit 1. A command's exit status passes through (sh -c 'exit 7' gave 7).

## 2026-10-02

Evidence 2: a throwaway machine (orb create ubuntu:resolute liam-lab-fresh) provisioned from nothing by this script with only the machine name changed: exit 0 in 1m33s, installing all packages, rust 1.98.1 with both musl targets and building musl; build:static then passed there. The second setup on it: exit 0 in 0.74s, 'all installed', toolchain 'unchanged', musl not rebuilt. liam-lab itself: setup re-run several times, each exit 0 with nothing reinstalled. The throwaway machine was deleted.

## 2026-10-02

Evidence 3: scripts/lab cargo test --workspace in liam-lab, exit 0: six test binaries, 'test result: ok' for each (2 + 2 + 2 passed, three with 0 tests).

## 2026-10-02

Evidence 4: scripts/lab scripts/task build:static, exit 0: liamd 543392 bytes and liam-init 397880 bytes, both 'ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, stripped'. CI's static job measured 543,400 and 397,880 for the same code. Both run in the lab under OrbStack's Rosetta binfmt: 'liamd 0.0.0', 'liam-init 0.0.0', which exercises mimalloc's C built by the cross musl-gcc. The aarch64-unknown-linux-musl target also builds both binaries statically and they run.

## 2026-10-02

Evidence 5: kernel 7.0.14-orbstack, /proc/sys/kernel/io_uring_disabled = 0, 149 io_uring symbols in /proc/kallsyms including __arm64_sys_io_uring_setup. A 50-line C program (raw syscalls, no liburing) set up a 4-entry ring, submitted IORING_OP_READ of /etc/hostname and reaped it: 'io_uring: features=0x3ffff user_data=42 res=9 data=liam-lab', exit 0.

## 2026-10-02

Evidence 6: CARGO_TARGET_DIR in the lab is /home/oddurs/.cache/liam-lab/target/Users/oddurs/Code/.worktrees/liam/chore/linux-lab for this worktree (and for its crates/liamd subdirectory), and /home/oddurs/.cache/liam-lab/target/Users/oddurs/Code/liam for the primary checkout. After the lab builds the worktree has no target/ directory at all.

## 2026-10-02

Evidence 7: CONTRIBUTING.md, under Checks, says in three lines that on macOS Linux-only checks run in an OrbStack machine via scripts/lab setup and a prefixed command, and that Firecracker boots stay in CI.

## 2026-10-02

Review follow-up: the musl prefix is now versioned (~/.local/x86_64-linux-musl-1.2.5), so bumping musl_version rebuilds rather than keeping old headers; rustup-init is downloaded to a file before it runs, so a failed download fails setup with curl's error. Re-verified: setup on liam-lab built the versioned prefix, the old one was removed, and a clean build:static gave liamd 543392 and liam-init 397880 bytes, static-pie x86-64. A second throwaway machine was provisioned from nothing by the revised script (exit 0), build:static passed on it, and it was deleted.

## Result

scripts/lab runs any command in the liam-lab OrbStack machine (Ubuntu 26.04 aarch64, kernel 7.0 with io_uring enabled) at the caller's directory, with CARGO_TARGET_DIR under ~/.cache/liam-lab/target/<worktree path> so worktrees and macOS never share a target. scripts/lab setup provisions it idempotently in about 1.5 minutes from nothing. x86_64 static builds work: linker x86_64-linux-gnu-gcc, and C dependencies compile against an x86_64 musl 1.2.5 built from source, because Ubuntu has no cross musl. scripts/lab scripts/task build:static gives the same sizes as CI to within 8 bytes. x86_64 binaries run in the lab under Rosetta. No KVM: Firecracker boots stay in CI. Run scripts/task check on macOS, because site/node_modules belongs to macOS.
