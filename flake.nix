{
  description = "K-ernel — bare metal x86_64 Rust kernel";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      # Pinned nightly — bump this when you need newer features
      rustToolchain = pkgs.rust-bin.nightly."2025-04-01".default.override {
        extensions = [ "rust-src" "llvm-tools-preview" ];
        targets = [ "x86_64-unknown-none" ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "k-ernel";

        packages = [
          # Rust
          rustToolchain

          # Build tools
          pkgs.limine                 # Limine bootloader + limine-deploy
          pkgs.xorriso                # ISO creation (used by limine)
          pkgs.nasm                   # assembler (may still be needed for stubs)

          # Emulation & debugging
          pkgs.qemu                   # qemu-system-x86_64
          pkgs.gdb                    # debugger (connect to QEMU gdbstub)

          # Binary inspection
          pkgs.binutils               # objdump, nm, readelf
        ];

        # Prevent cargo from trying to link against host libc
        CARGO_BUILD_TARGET = "x86_64-unknown-none";

        shellHook = ''
          echo "K-ernel dev shell"
          echo "rust: $(rustc --version)"
          echo "qemu: $(qemu-system-x86_64 --version | head -1)"
        '';
      };
    };
}
