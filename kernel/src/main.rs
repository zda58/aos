#![no_std]
#![no_main]

#![feature(lang_items)]
use core::panic::PanicInfo;

mod arch;

#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    let UART_OUTPUT : *mut u8= 0x9000000 as *mut u8;
    unsafe {
        *UART_OUTPUT = b'a';
    }
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}