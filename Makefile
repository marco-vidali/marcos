default:
	cargo clean
	cargo build --release
	aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/release/kernel sd/kernel8.img
	cp -r sd/* /Volumes/bootfs