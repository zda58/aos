use core::arch::global_asm;

global_asm!(
    include_str!("boot.s")
);

#[no_mangle]
pub extern "C" fn aarch64_kernel_entry() -> ! {
    crate::kernel_start();
}