use core::arch::global_asm;

global_asm!(
    include_str!("utils.s")
);

extern "C" {
    pub fn memcpy(src: *const u8, dest: *const u8, size: usize);
}