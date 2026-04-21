ENTRY(_start)
SECTIONS
{
    . = 0xFFFFFFFF80000000;

    .text : {
	*(.text .text.*)
    }

    . = ALIGN(4096);

    .rodata : {
	*(.rodata .rodata.*)
    }

    . = ALIGN(4096);

    .data : {
	*(.data .data.*)
    }

    . = ALIGN(4096);

    .bss : {
	__bss_start = .;
	*(.bss .bss.*)
	__bss_end = .;
    }
}
