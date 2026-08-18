# K-ernel

A bare-metal x86_64 kernel written in Rust. Boots via Limine on UEFI firmware, runs in QEMU — with its own shell, scheduler, memory management, storage layer, and drivers.

Built from scratch. Not a Linux build: the boot process, memory management, scheduling, and device handling are written directly against the hardware.

## Status

Early-stage hobbyist OS project — alive and booting. The kernel is organized into modules:

| Module | What it does |
|---|---|
| `arch/` | x86_64 architecture code |
| `memory/` | Memory management |
| `scheduler/` | Process scheduling |
| `storage/` | Storage / block device layer |
| `drivers/` | Device drivers |
| `sync/` | Synchronization primitives |
| `shell/` | In-kernel shell |

## Building & Running

The Nix flake provides the full toolchain (pinned Rust nightly, Limine, xorriso, mtools, QEMU, gdb, binutils):

```bash
nix develop          # enter the dev shell
make iso             # build the kernel + produce a bootable UEFI ISO
make run             # boot it in QEMU (q35, 256M RAM, OVMF)
```

Manual build:

```bash
cd kernel && cargo build --target x86_64-unknown-none
```

## Layout

```
Makefile              # build / ISO / QEMU targets
flake.nix             # reproducible dev environment (nightly 2025-04-01, x86_64-unknown-none)
kernel/
  Cargo.toml          # kernel crate
  limine.conf         # Limine bootloader config
  linker.ld           # kernel linker script
  rust-toolchain.toml # pinned nightly
  src/
    main.rs
    arch/ memory/ scheduler/ storage/ drivers/ sync/ shell/
```

## Debugging

QEMU exposes a gdbstub (`qemu-system-x86_64 -s -S`); the dev shell includes `gdb` and `binutils` (`objdump`, `nm`, `readelf`) for inspecting the kernel ELF.
