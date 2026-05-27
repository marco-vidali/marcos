use core::ptr;

use crate::PERIPHERALS_BASE_ADDR;

const GPIO_BASE_ADDR: usize = PERIPHERALS_BASE_ADDR + 0x200000;
const GPIO_PUP_PDN_BASE_ADDR: usize = GPIO_BASE_ADDR + 0xe4;

#[repr(usize)]
pub enum GPIOPinFunc {
    Input = 0,
    Output = 1,
    Alt0 = 4,
    Alt1 = 5,
    Alt2 = 6,
    Alt3 = 7,
    Alt4 = 3,
    Alt5 = 2,
}

pub fn enable_pin(pin_num: u8) {
    if pin_num > 57 {
        return;
    }

    let bit_start = (pin_num % 16) * 2;
    let reg = (pin_num / 16) as usize;
    let target_addr = GPIO_PUP_PDN_BASE_ADDR + (reg * 4);
    let gpio_pup_pdn_ptr = target_addr as *mut u32;

    unsafe {
        let mut reg_val = ptr::read_volatile(gpio_pup_pdn_ptr); // read entire GPIO_PUP_PDN address of the pin
        reg_val &= !(3 << bit_start); // set pin pup_pdn to 00
        ptr::write_volatile(gpio_pup_pdn_ptr, reg_val); // write new GPIO_PUP_PDN address value
    }
}

pub fn set_pin_function(pin_num: u8, func: GPIOPinFunc) {
    if pin_num > 57 {
        return;
    }

    let bit_start = (pin_num % 10) * 3;
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
