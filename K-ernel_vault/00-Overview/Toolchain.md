# Toolchain

## Tools
- `rustup` — toolchain manager
- `cargo` — build system
- `qemu-system-x86_64` — emulator
- `grub-mkrescue` or `bootimage` — bootable image creation
- `objdump` / `nm` — binary inspection
- `gdb` + QEMU stub — debugging

## Rust setup
- Target: `x86_64-unknown-none` (bare metal, no OS)
- Nightly required (for `#![no_std]` core features)
- `cargo-bootimage` crate or custom linker invocation

## NixOS notes
- All tools via nix — no pip/npm/cargo install -g without nix wrapping
- Use `nix develop` shell for project environment
- flake.nix to be created in kernel source dir (separate from vault)

## Key flags
- `-C link-arg=-Tlinker.ld` — custom linker script
- `--target x86_64-unknown-none`
