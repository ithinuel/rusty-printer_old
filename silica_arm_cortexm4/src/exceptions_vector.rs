use core;
use Exceptions;

extern "C" {
    fn main();
}

extern "C" {
    static mut _sbss: u32;
    static mut _ebss: u32;
    static bss_size: usize;
    static mut _sdata: u32;
    static mut _edata: u32;
    static _sidata: u32;
    static data_size: usize;
}

#[naked]
unsafe extern "C" fn start() -> ! {
    let psbss = &mut _sbss as *mut u32;
    let lbss_size = &bss_size as *const usize as usize;
    // initialize bss
    core::intrinsics::write_bytes(psbss, 0, lbss_size);

    // initialize idata
    let psidata = &_sidata as *const u32;
    let psdata = &mut _sdata as *mut u32;
    let ldata_size = &data_size as *const usize as usize;
    core::intrinsics::copy_nonoverlapping(psidata, psdata, ldata_size);

    // system init
    main();
    ::ppb::SCB.aircr.get_mut().sys_reset_request();
    debug_assert!(false, "should not be reached");
    unreachable!();
}

unsafe extern "C" fn default_handler() {}
unsafe extern "C" fn hf_handler() {}
unsafe extern "C" fn pendsv() {}
unsafe extern "C" fn systick() {}

#[cfg(target_arch = "arm")]
#[link_section = ".vector_table.exceptions_vector"]
#[no_mangle]
#[used]
pub static EXCEPTIONS: Exceptions = Exceptions {
    reset: start,                 // RESET
    nmi: default_handler,         // NMI
    hard_fault: hf_handler,       // Hardfault
    mem_manage: default_handler,  // MemManage
    bus_fault: default_handler,   // BusFault
    usage_fault: default_handler, // UsageFault
    reserved1: [0; 4],
    sv_call: default_handler,       // SVCall
    debug_monitor: default_handler, // Debug Monitor
    reserved2: 0,
    pendsv: pendsv,   // PendSV
    systick: systick, // Systick
};
