#![no_std]
#![no_main]

#![feature(lang_items)]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART_OUTPUT : *const u64 = 0x9000000;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}