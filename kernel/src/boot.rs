use core::arch::{asm, global_asm};

// SPSR_EL3
const SPSR_MASK_ALL: u64 = 0b1111 << 6;
const SPSR_EL1H: u64 = 0b0100 << 0;
const SPSR_VALUE: u64 = SPSR_MASK_ALL | SPSR_EL1H;

// HCR_EL2
const HCR_RW: u64 = 1 << 31;
const HCR_VALUE: u64 = HCR_RW;

const SCR_RESERVED: u64 = 3 << 4;
const SCR_RW: u64 = 1 << 10;
const SCR_NS: u64 = 1 << 0;
const SCR_VALUE: u64 = SCR_RESERVED | SCR_RW | SCR_NS;

global_asm!(
    r#"
    .section ".text.boot"

    .global _start
    _start:
        // halt all the cores except core 0
        mrs x0, mpidr_el1
        and x0, x0, #0xFF
        cbz x0, master
        b halt

    master:
        // configure exception level change (EL3 -> EL1)
        ldr x0, ={HCR_VALUE}
        msr hcr_el2, x0

        ldr x0, ={SCR_VALUE}
        msr scr_el3, x0

        ldr x0, ={SPSR_VALUE}
        msr spsr_el3, x0

        adr x0, el1_entry
        msr elr_el3, x0

        eret // change level and branch to el1_entry

    el1_entry:
        // set stack pointer to 0x8000000
        mov x0, #0x8000000
        mov sp, x0

        bl kernel_main

    halt:
        wfe
        b halt
    "#,
    HCR_VALUE  = const HCR_VALUE,
    SCR_VALUE  = const SCR_VALUE,
    SPSR_VALUE = const SPSR_VALUE,
);

pub fn get_current_el() -> u8 {
    let mut cur_el: u64;

    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) cur_el); // read CurrentEL reg and save it to cur_el
    }

    ((cur_el >> 2) & 0x3) as u8
}
