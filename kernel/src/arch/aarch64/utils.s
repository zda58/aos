.globl get_el_num
get_el_num:
    mrs x0, CurrentEL
    lsr x0, x0, #2
    ret

// x0: dest
// x1: src
// x2: size
.globl memcpy
memcpy:
cbz x2, memcpy_done

memcpy_loop:
ldrb w3, [x1], #1
strb w3, [x0], #1
subs x2, x2, #1
b.ne memcpy_loop

memcpy_done:
ret

dump_registers:
sub sp, sp, #