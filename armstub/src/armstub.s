.global _start
_start:
    // branch to kernel entry
    ldr w0, kernel_entry32
    br x0

.ltorg

// stub configuration
.org 0xf0
.globl stub_magic
stup_magic:
    .word 0x5afe570b

.org 0xf4
.globl stub_version
stub_version:
    .word 0

.org 0xfc
.globl kernel_entry32
kernel_entry32:
    .word 0x0