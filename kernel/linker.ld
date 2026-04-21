ENTRY(_start)
SECTIONS
{
    . = 0xFFFFFFFF80000000;

    .requests : {
	. = ALIGN(4096);
	KEEP(*(.requests))
    }

    . = ALIGN(4096);

    .text : {
        . = ALIGN(4096);
	*(.text .text.*)
    }

    . = ALIGN(4096);

    .rodata : {
        . = ALIGN(4096);
	*(.rodata .rodata.*)
    }

    . = ALIGN(4096);

    .got : {
	*(.got .got.*)
    }

    . = ALIGN(4096);

    .data : {
        . = ALIGN(4096);
	*(.data .data.*)
    }

    . = ALIGN(4096);

    .bss : {
        . = ALIGN(4096);
	__bss_start = .;
	*(.bss .bss.*)
	__bss_end = .;
    }
}
