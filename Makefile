default:
	cargo clean
	cargo build
	aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/debug/marcos sd/kernel8.img
	cp -r sd/* /Volumes/bootfs