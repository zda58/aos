__virt_memory_base = 0x40000000;

/* 4kb for setup */
__stack_top = __virt_memory_base + 0x1000;

SECTIONS
{
    . = __virt_memory_base;
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss) }
    .rodata : { *(.rodata) }
}