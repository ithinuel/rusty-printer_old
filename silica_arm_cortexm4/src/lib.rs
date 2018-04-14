#![feature(lang_items, asm, linkage, used, panic_runtime)]
#![no_std]
#![cfg_attr(target_arch = "arm", panic_runtime)]

extern crate silica_arm_cortexm;


// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[no_mangle]
#[cfg(target_arch = "arm")]
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
#[cfg(target_arch = "arm")]
#[cfg_attr(not(test), lang = "panic_fmt")]
pub fn panic_handler(_msg: core::fmt::Arguments,
                     _file: &'static str,
                     _line: u32,
                     _column: u32) -> ! {
    // use semihosting or failure cause buffer or stdout(a peripheral) or ITM
    unsafe { asm!("bkpt") }
    loop {}
}

pub type Handler = unsafe extern "C" fn ();

#[repr(C)]
pub struct Exceptions {
    pub reset: unsafe extern "C" fn () -> !,
    pub nmi: Handler,
    pub hard_fault: Handler,
    pub mem_manage: Handler,
    pub bus_fault: Handler,
    pub usage_fault: Handler,
    pub reserved1: [u32; 4],
    pub sv_call: Handler,
    pub debug_monitor: Handler,
    pub reserved2: u32,
    pub pendsv: Handler,
    pub systick:  Handler
}

pub unsafe extern "C" fn start() -> ! {
    extern {
        fn main();
    }

    extern "C" {
        static idata_from: usize;
        static idata_to: usize;
        static idata_size: usize;
        static bss_start: usize;
        static bss_size: usize;
    }

    // initialize bss
    let _bss_start = &bss_start as *const usize as *mut u32;
    let _bss_size = &bss_size as *const usize as usize;
    core::intrinsics::write_bytes(_bss_start, 0, _bss_size);

    // initialize idata
    let _idata_from = &idata_from as *const usize as *const u32;
    let _idata_to = &idata_to as *const usize as *mut u32;
    let _idata_size = &idata_size as *const usize as usize;
    core::intrinsics::copy(_idata_from, _idata_to, _idata_size);

    // system init
    main();
    //silica_cortexm3::ppb::scb::system_reset();
    loop {};
}

pub unsafe extern "C" fn default_handler()  {
}

pub unsafe extern "C" fn hf_handler() {
}
pub unsafe extern "C" fn pendsv() {
}
pub unsafe extern "C" fn systick() {
}

#[used]
#[link_section = ".vector_table.exceptions_vector"]
static EXCEPTIONS: Exceptions = Exceptions {
    reset: start,  // RESET
    nmi: default_handler,   // NMI
    hard_fault: hf_handler,   // Hardfault
    mem_manage: default_handler,   // MemManage
    bus_fault: default_handler,   // BusFault
    usage_fault: default_handler,   // UsageFault
    reserved1: [0; 4],
    sv_call: default_handler,   // SVCall
    debug_monitor: default_handler,   // Debug Monitor
    reserved2: 0,
    pendsv: pendsv,   // PendSV
    systick: systick,   // Systick
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
