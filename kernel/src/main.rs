#![no_std]
#![no_main]

use core::panic::PanicInfo;

use drivers::mini_uart;

mod boot;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    mini_uart::init();

    mini_uart::send_str("Welcome to marcos!\n");
    mini_uart::send_str("Exception Level: ");

    let el = (b'0' + boot::get_current_el() as u8) as char;

    mini_uart::send_char(el);
    mini_uart::send_char('\n');

    loop {
        mini_uart::send_char(mini_uart::recv());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
