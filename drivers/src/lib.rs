#![no_std]

pub mod gpio;
pub mod mini_uart;

const PERIPHERALS_BASE_ADDR: usize = 0xfe000000;
