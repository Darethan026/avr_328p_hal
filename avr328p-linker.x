/* Default linker script, for normal executables */
/* Copyright (C) 2014-2015 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */
/* Memory regions for ATmega328P */
MEMORY
{
  text (rx)   : ORIGIN = 0, LENGTH = 32K
  data (rw!x) : ORIGIN = 0x800100, LENGTH = 2K
}

SECTIONS
{
  /* 1. Standard AVR Code Section */  
  .text :
  {
    /* The vector table must be exactly at 0x0000 */
    KEEP(*(.vectors))
    
    /* Standard AVR-GCC initialization sections (init0 to init9) */
    /* gcrt1.S uses these to set the stack, clear BSS, and copy DATA */
    KEEP(*(.init0))
    KEEP(*(.init1))
    KEEP(*(.init2))
    KEEP(*(.init3))
    KEEP(*(.init4))
    KEEP(*(.init5))
    KEEP(*(.init6))
    KEEP(*(.init7))
    KEEP(*(.init8))
    KEEP(*(.init9))

    *(.text*)
    *(.rodata*)
    
    . = ALIGN(2);
    _etext = . ;
  } > text

  /* 2. Initialized Data (VMA in RAM, LMA in Flash) */

  .data : 
  {
    PROVIDE (__data_start = .) ;
    *(.data*)
    . = ALIGN(2);
    PROVIDE (__data_end = .) ;
  } > data AT > text

  /* 3. Uninitialized Data (Zeroed at boot) */
  .bss :
  {
    PROVIDE (__bss_start = .) ;
    *(.bss*)
    *(COMMON)
    . = ALIGN(2);
    PROVIDE (__bss_end = .) ;
  } > data

  /* Standard symbols expected by AVR-GCC for boot logic */
  __data_load_start = LOADADDR(.data);
}
