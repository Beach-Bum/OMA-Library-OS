# ΦΜΛ — Build targets
#
# Usage:
#   make build          Build release binary
#   make install        Install to ~/.local/bin/
#   make cross-pi       Cross-compile for Raspberry Pi (aarch64)
#   make image          Build bootable SD card image (requires Buildroot)
#   make site           Rebuild the documentation site
#   make clean          Clean build artifacts

BINARY = target/release/oma
PI_BINARY = target/aarch64-unknown-linux-musl/release/oma
INSTALL_DIR = $(HOME)/.local/bin

.PHONY: build install cross-pi image site clean

build:
	cargo build --release
	strip $(BINARY)
	@echo "Built: $(BINARY) ($$(du -h $(BINARY) | cut -f1))"

install: build
	mkdir -p $(INSTALL_DIR)
	cp $(BINARY) $(INSTALL_DIR)/oma
	@echo "Installed to $(INSTALL_DIR)/oma"

cross-pi:
	rustup target add aarch64-unknown-linux-musl 2>/dev/null || true
	cargo build --release --target aarch64-unknown-linux-musl
	@echo "Built: $(PI_BINARY)"

image: cross-pi
	@echo "Building bootable image..."
	@mkdir -p build
	@# Create a minimal initramfs with just the oma binary
	@mkdir -p build/initramfs/{bin,dev,proc,sys,tmp,root}
	@cp $(PI_BINARY) build/initramfs/bin/oma
	@# Init script: mount filesystems, set up environment, exec oma
	@cat > build/initramfs/init << 'INITEOF'
#!/bin/oma
# This init script is never actually read by oma as a document.
# The kernel execs /init which is a symlink to /bin/oma.
# But if you could read it, it would say:
# The library is the first thing that exists after the kernel.
INITEOF
	@# Actually make init a shell script that launches oma
	@cat > build/initramfs/init << 'INITEOF'
#!/bin/sh
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
export HOME=/root
export OMA_ROOT=/root/oma-library
export TERM=linux
exec /bin/oma
INITEOF
	@chmod +x build/initramfs/init
	@# Create initramfs cpio
	@cd build/initramfs && find . | cpio -o -H newc 2>/dev/null | gzip > ../initramfs.cpio.gz
	@echo ""
	@echo "Initramfs created: build/initramfs.cpio.gz ($$(du -h build/initramfs.cpio.gz | cut -f1))"
	@echo ""
	@echo "To boot on a Raspberry Pi:"
	@echo "  1. Install a minimal aarch64 Linux kernel on an SD card"
	@echo "  2. Set initramfs=initramfs.cpio.gz in config.txt"
	@echo "  3. Copy build/initramfs.cpio.gz to the boot partition"
	@echo ""
	@echo "To test with QEMU:"
	@echo "  qemu-system-aarch64 -M virt -cpu cortex-a72 \\"
	@echo "    -kernel <your-kernel> -initrd build/initramfs.cpio.gz \\"
	@echo "    -append 'console=ttyAMA0' -nographic"

site:
	python3 build.py
	@echo "Site built in site/"

clean:
	cargo clean
	rm -rf build/
