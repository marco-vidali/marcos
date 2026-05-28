#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

use drivers::mini_uart;

global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    mini_uart::init();

    mini_uart::send_str("Welcome to marcos!\n");

    loop {
        mini_uart::send_char(mini_uart::recv());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
