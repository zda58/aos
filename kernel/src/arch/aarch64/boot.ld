__virt_memory_base = 0x40000000

SECTIONS
{
    . = __virt_memory_base;
    .text : { *(.text) }
}