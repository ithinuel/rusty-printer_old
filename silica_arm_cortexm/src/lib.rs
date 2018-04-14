#![feature(lang_items, asm, linkage, used, panic_runtime, naked_functions)]
#![no_std]
#![cfg_attr(target_arch = "arm", panic_runtime)]

extern crate silica;

#[cfg(target_arch = "arm")]
pub mod panic_runtime;

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

#[cfg(target_arch = "arm")]
pub mod exceptions_vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
