/* With multiple codegen units the rlib produced for this crate has several object files in it. */
/* Because the linker is Smart it may not look into all the object files and not pick up the */
/* .vector_table.exceptions section. But we want it to! To workaround the problem we create an */
/* undefined reference to the EXCEPTIONS symbol (located in .vector_table.exceptions); this way the */
/* linker will look at all the object of the rlib and pick up our EXCEPTIONS symbol */
/* thanks to https://github.com/japaric/cortex-m-rt/blob/master/link.x for that Hint */
/* embedonomicon might need to be updated though */
EXTERN(EXCEPTIONS);

/* Create an undefined reference to the INTERRUPTS symbol. This is required to
   force the linker to *not* drop the INTERRUPTS symbol if it comes from an
   object file that's passed to the linker *before* this crate */
/* EXTERN(INTERRUPTS); */

PROVIDE(SCB = 0xE000ED00);

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
    } > FLASH

    .panic_msg ORIGIN(RAM) : {
        . = ALIGN(. + panic_msg_size, 4);
    } > RAM
    .data : {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _sdata = .;
        *(.data*)
        _edata = .;
    } > RAM AT > FLASH
    data_size = SIZEOF(.data);

    .bss : {
        _sbss = .;
        *(.bss*)
        _ebss = .;
    } > RAM
    PROVIDE(bss_size = SIZEOF(.bss));

    heap_size = LENGTH(RAM) - data_size - panic_msg_size - stack_size - bss_size;
    .heap : {
        _sheap = .;
        . = . + heap_size;
        _eheap = .;
    } > RAM = 0xAAAAAAAA

    _estack = ORIGIN(RAM) + LENGTH(RAM);
    _sstack = ALIGN(_estack - (stack_size + 7), 8);
    
    /* this section is to ensure we save enough room for the main stack. */
    .stack _sstack : {
        . = . + stack_size;
    } > RAM = 0x55555555

    /DISCARD/ :
    {
        *(.note.gnu.build-id*)
        *(.ARM.attributes*)
        *(.ARM.extab*)
        *(.ARM.exidx*)
        *(.init*)
        *(.fini*)
    }

}
ASSERT(EXCEPTIONS != 0, "missing start symbol");

