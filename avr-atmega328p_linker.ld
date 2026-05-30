/* Custom AVR-ATmega328p linker script. */
/* Github username: https://github.com/Darethan026 */

OUTPUT_FORMAT("elf32-avr")
OUTPUT_ARCH(avr:5)

MEMORY
{
	text (rx) : ORIGIN = 0, LENGTH = 32K
	data : ORIGIN = 0x800100, LENGTH = 2K
}

SECTIONS
{
	.text : 
	{
		*(.vectors)
		KEEP(*(.vectors))

		/* Initialise vectors from 0 to 9 */
		*(.init0) /* After reset, start here */
		KEEP(*(.init0))

		*(.init1)
		KEEP(*(.init1))

		*(.init2)
		KEEP(*(.init2))

		*(.init3)
		KEEP(*(.init3))

		*(.init4)
		KEEP(*(.init4))

		*(.init5)
		KEEP(*(.init5))

		*(.init6)
		KEEP(*(.init6))

		*(.init7)
		KEEP(*(.init7))

		*(.init8)
		KEEP(*(.init8))

		*(.init9)
		KEEP(*(.init9))

		*(.text)
		*(.text*)

		. = ALIGN(2);
		_etext = . ;
	} > text

	.data :
	{
		__data_start = . ;
		*(.data)
		*(.data*)

		*(.rodata)
		*(.rodata*)

		. = ALIGN(2);

		__data_end = . ;
	
	} > data AT > text

	.bss :
	{
		__bss_start = . ;

		*(.bss)
		*(.bss*)
		*(COMMON)

		. = ALIGN(2);

		__bss_end = . ;
	} > data

	__data_load_start = LOADADDR(.data);
}

__stack_top = ORIGIN(data) + LENGTH(data);