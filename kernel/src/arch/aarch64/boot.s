.section .text
.global _start
_start:
    mov eax, 60          // syscall number for exit
    xor edi, edi         // exit code 0
    call _aarch64_kernel_entry