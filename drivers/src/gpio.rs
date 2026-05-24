use core::ptr;

use crate::PERIPHERALS_BASE_ADDR;

const GPIO_BASE_ADDR: usize = PERIPHERALS_BASE_ADDR + 0x200000;

#[repr(usize)]
pub enum GPIOFunc {
    Input = 0,
    Output = 1,
    Alt0 = 4,
    Alt1 = 5,
    Alt2 = 6,
    Alt3 = 7,
    Alt4 = 3,
    Alt5 = 2,
}

pub fn set_function(pin_num: u8, func: GPIOFunc) {
    if pin_num > 57 {
        return;
    }

    let bit_start = (pin_num * 3) % 30;
    let reg = (pin_num / 10) as usize;
    let target_addr = GPIO_BASE_ADDR + (reg * 4);
    let gpfsel_ptr = target_addr as *mut u32;

    unsafe {
        let mut reg_val = ptr::read_volatile(gpfsel_ptr); // read entire GPFSEL address of the pin

        reg_val &= !(7 << bit_start); // set pin function bits to 000
        reg_val |= (func as u32) << bit_start; // set function bits to desired function

        ptr::write_volatile(gpfsel_ptr, reg_val); // write new GPFSEL address value
    }
}
