clean:
	rm -rf iso_root/ efi.img

build: clean
	cd kernel && cargo build
iso: build
	mkdir -p iso_root/EFI/BOOT
	cp kernel/target/x86_64-unknown-none/debug/kernel iso_root/kernel
	cp kernel/limine.conf iso_root/
	cp $(shell dirname $(shell dirname $(shell which limine)))/share/limine/BOOTX64.EFI iso_root/EFI/BOOT/
	dd if=/dev/zero of=efi.img bs=1M count=4
	mkfs.fat -F 12 efi.img
	mmd -i efi.img ::/EFI
	mmd -i efi.img ::/EFI/BOOT
	mcopy -i efi.img iso_root/EFI/BOOT/BOOTX64.EFI ::/EFI/BOOT/
	mcopy -i efi.img kernel/limine.conf ::/
	mcopy -i efi.img iso_root/kernel ::/kernel
	cp efi.img iso_root/EFI/BOOT/efi.img
	xorriso -as mkisofs -isohybrid-gpt-basdat -o kernel.iso -e EFI/BOOT/efi.img -no-emul-boot -efi-boot-part --efi-boot-image iso_root/EFI/BOOT/efi.img iso_root/

disk.img:
	dd if=/dev/zero of=disk.img bs=1M count=64

run: iso disk.img
	qemu-system-x86_64 -M q35 -m 256M -vga std \
		-bios /nix/store/z5yzn2d9s4k12vwr34h8bvgfi1p015ql-OVMF-202508.01-fd/FV/OVMF.fd \
		-drive format=raw,file=kernel.iso \
		-drive if=none,format=raw,file=disk.img,id=disk0 \
		-device ich9-ahci,id=ahci \
		-device ide-hd,drive=disk0,bus=ahci.0 \
		-display gtk
