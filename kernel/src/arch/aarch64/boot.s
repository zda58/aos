.section .text
.global _start
_start:
    ldr x30, =0x40001000;
    mov sp, x30
    b _aarch64_kernel_entry