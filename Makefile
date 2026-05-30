ARMGNU ?= aarch64-none-elf
BOOTMNT ?= /Volumes/bootfs

.PHONY: default armstub clean

default:
	cargo clean
	cargo build --release
	$(ARMGNU)-objcopy -O binary target/aarch64-unknown-none/release/kernel sd/kernel8.img
	cp -r sd/* $(BOOTMNT)/
	sync

armstub/build/armstub_s.o: armstub/src/armstub.s
	mkdir -p $(@D)
	$(ARMGNU)-as $< -o $@

armstub: armstub/build/armstub_s.o
	$(ARMGNU)-ld --section-start=.text=0 -o armstub/build/armstub.elf armstub/build/armstub_s.o
	$(ARMGNU)-objcopy armstub/build/armstub.elf -O binary sd/armstub8-2711.bin.bin
	cp sd/armstub8-2711.bin.bin $(BOOTMNT)/
	sync

clean:
	cargo clean
	rm -rf armstub/build