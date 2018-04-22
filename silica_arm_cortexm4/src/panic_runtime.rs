use core;

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[no_mangle]
#[lang = "eh_personality"]
pub fn rust_eh_personality() {
    unsafe { asm!("bkpt") }
}

/*
// This function may be needed based on the compilation target.
#[no_mangle]
#[lang = "eh_unwind_resume"]
pub fn rust_eh_unwind_resume() {
    unsafe { asm!("bkpt") }
}*/

#[no_mangle]
#[lang = "panic_fmt"]
pub fn panic_handler(_msg: core::fmt::Arguments,
                     _file: &'static str,
                     _line: u32,
                     _column: u32) -> ! {
    // use semihosting or failure cause buffer or stdout(a peripheral) or ITM
    unsafe { asm!("bkpt") }
    loop {}
}
