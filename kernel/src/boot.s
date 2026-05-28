.section ".text.boot"
.global _start

_start:
    // set stack pointer to 0x8000
    mov x0, #0x8000
    mov sp, x0

    bl kernel_main

halt:
    wfe
    b halt