#![no_std]
#![no_main]

#![feature(lang_items)]
use core::panic::PanicInfo;

mod arch;
mod kio;

extern "C" {
    fn get_el_num() -> u64;
}

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    unsafe {
        kio::printk("fwefew");
    }
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}