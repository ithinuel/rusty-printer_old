/* Memory layout of the ATSAM4E8E */
MEMORY
{
  FLASH : ORIGIN = 0x00400000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

panic_str_size = 512;
stack_size = 1024;

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM)); /* Initial Stack Pointer value */
    KEEP(*(.vector_table.exceptions_vector));
    KEEP(*(.vector_table.interrupts_vector));
  } > FLASH


  .text : {
    *(.text*)
  } > FLASH
  .rodata : {
    *(.rodata*)
    idata_from = .;
  } > FLASH

  .panic_fmt ORIGIN(RAM) : {
    . = ALIGN(. + panic_str_size, 4);
  } > RAM

  .data : AT(idata_from) {
    idata_to = .;
    *(.data*)
  } > RAM
  idata_size = SIZEOF(.data);

  .bss : {
    bss_start = .;
    *(.bss*)
  } > RAM
  bss_size = SIZEOF(.bss);

  heap_size = LENGTH(RAM) - idata_size - panic_str_size - stack_size - bss_size;
  .heap : {
    heap_start = .;
    . = . + heap_size;
  } > RAM = 0xAAAAAAAA

  stack_start = ALIGN(stack_end - (stack_size + 7), 8);
  stack_end = ORIGIN(RAM) + LENGTH(RAM);
  .stack stack_start : {
    . = . + stack_size;
  } > RAM = 0x55555555

  /DISCARD/ :
  {
    *(.note.gnu.build-id*)
    *(.ARM.attributes*)
    *(.ARM.extab*)
    *(.ARM.exidx*)
  }
}
