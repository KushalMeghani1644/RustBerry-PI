OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    ram (rxw) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    . = ORIGIN(ram);

    .text : {
        KEEP(*(.text._start))
        *(.text*)
        *(.rodata*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram

    .data : {
        *(.data*)
    } > ram

    .bss (NOLOAD) : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    } > ram

    . = ALIGN(8);
    _stack_start = .;
    . += 16K;
    _stack_end = .;
}
