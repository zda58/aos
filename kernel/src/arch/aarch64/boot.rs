use core::arch::global_asm;

global_asm!(
    include_str!("boot.s")
);

pub fn _aarch64_kernel_entry() -> ! {
    crate::_kernel_start();
}