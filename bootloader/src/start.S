
    .section .text._start
    .global _start
    .type _start, @function

_start:
    /* Set up stack pointer to top of stack defined in linker script */
    la sp, _stack_end

    /* Call rust entry */
    call rust_main

    /* If rust_main returns, loop */
1:  j 1b

    .size _start, . - _start
