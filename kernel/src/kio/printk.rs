use crate::arch::utils::memcpy;

const UART_OUTPUT: *mut u8 = 0x9000000 as *mut u8;

#[no_mangle]
pub extern "C" fn printk(str: &[u8]) {
    unsafe {
        for c in str {
            *UART_OUTPUT = *c;
        }
    }
}