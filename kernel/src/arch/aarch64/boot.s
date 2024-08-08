.section .text
.globl _start
_start:
    ldr x30, =__stack_top;
    mov sp, x30
    b aarch64_kernel_entry