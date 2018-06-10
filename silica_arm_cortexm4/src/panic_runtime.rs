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
#[panic_implementation]
pub fn panic_handler(_: &::core::panic::PanicInfo) -> ! {
    // use semihosting or failure cause buffer or stdout(a peripheral) or ITM
    unsafe { asm!("bkpt") }
    loop {}
}
