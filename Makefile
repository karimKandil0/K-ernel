build:
	cd kernel && cargo build
iso: build
	mkdir -p iso_root/EFI/BOOT
	cp kernel/target/x86_64-unknown-none/debug/kernel iso_root/kernel
	cp kernel/limine.conf iso_root/
	cp $(dirname $(dirname $(which limine)))/share/limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -o kernel.iso --efi-boot EFI/BOOT/BOOTX64.EFI -efi-boot-part --efi-boot-image iso_root/

run: iso
	qemu-system-x86_64 -M q35 -m 256M -bios /nix/store/z5yzn2d9s4k12vwr34h8bvgfi1p015ql-OVMF-202508.01-fd/FV/OVMF.fd -cdrom kernel.iso -display gtk
