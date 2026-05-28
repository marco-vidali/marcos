use crate::{
    PERIPHERALS_BASE_ADDR,
    gpio::{self, GPIOPinFunc},
};

const AUX_BASE_ADDR: usize = PERIPHERALS_BASE_ADDR + 0x215000;
const AUX_ENABLES_ADDR: usize = AUX_BASE_ADDR + 0x04;
const AUX_MU_IO_ADDR: usize = AUX_BASE_ADDR + 0x40;
const AUX_MU_IER_ADDR: usize = AUX_BASE_ADDR + 0x44;
const AUX_MU_LCR_ADDR: usize = AUX_BASE_ADDR + 0x4c;
const AUX_MU_MCR_ADDR: usize = AUX_BASE_ADDR + 0x50;
const AUX_MU_LSR_ADDR: usize = AUX_BASE_ADDR + 0x54;
const AUX_MU_CNTL_ADDR: usize = AUX_BASE_ADDR + 0x60;
const AUX_MU_BAUD_ADDR: usize = AUX_BASE_ADDR + 0x68;

const TXD_PIN_NUM: u8 = 14;
const RXD_PIN_NUM: u8 = 15;

pub fn init() {
    gpio::set_pin_func(TXD_PIN_NUM, GPIOPinFunc::Alt5);
    gpio::set_pin_func(RXD_PIN_NUM, GPIOPinFunc::Alt5);

    gpio::enable_pin(TXD_PIN_NUM);
    gpio::enable_pin(RXD_PIN_NUM);

    unsafe {
        core::ptr::write_volatile(AUX_ENABLES_ADDR as *mut u32, 1);
        core::ptr::write_volatile(AUX_MU_CNTL_ADDR as *mut u32, 0);
        core::ptr::write_volatile(AUX_MU_IER_ADDR as *mut u32, 0);
        core::ptr::write_volatile(AUX_MU_LCR_ADDR as *mut u32, 3);
        core::ptr::write_volatile(AUX_MU_MCR_ADDR as *mut u32, 3);
        core::ptr::write_volatile(AUX_MU_BAUD_ADDR as *mut u32, 541);
        core::ptr::write_volatile(AUX_MU_CNTL_ADDR as *mut u32, 3);
    }
}

pub fn send_char(c: char) {
    unsafe {
        while (core::ptr::read_volatile(AUX_MU_LSR_ADDR as *const u32) & 0x20) == 0 {}
        core::ptr::write_volatile(AUX_MU_IO_ADDR as *mut u32, c as u32);
    }
}

pub fn send_str(s: &str) {
    for c in s.chars() {
        if c == '\n' {
            send_char('\r');
        }

        send_char(c);
    }
}

pub fn recv() -> char {
    unsafe {
        while (core::ptr::read_volatile(AUX_MU_LSR_ADDR as *const u32) & 1) == 0 {}
        (core::ptr::read_volatile(AUX_MU_IO_ADDR as *const u32) & 0xff) as u8 as char
    }
}
