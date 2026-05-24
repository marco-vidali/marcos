ENTRY(_start)

SECTIONS
{
    . = 0x8000;
    __start = .;
    
    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .eh_frame_hdr : { *(.eh_frame_hdr) }
    .eh_frame : { *(.eh_frame) }

    . = ALIGN(4096);
    .data :
    {
        *(.data .data.*)
    }

    . = ALIGN(4096);
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss .bss.*)
    }
    
    . = ALIGN(4096); 
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;

    /DISCARD/ :
    {
        *(.comment)
        *(.ARM.attributes)
    }
}