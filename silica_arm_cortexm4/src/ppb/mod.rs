//! CM4's Private Peripheral Bus
use core::fmt;
use silica::register::{Field, RegisterCell};

pub mod scb;
pub mod systick;

register! {
    @impl_debug;
    /// Auxiliary Control Register
    #[derive(Copy, Clone)]
    pub struct ACRegister(u32) {
        bool: pub multiple_cycle_instr_interruptable, pub allow_interruption_of_multicycle_instr: 0;
        bool: pub write_buffer_disabled, pub disable_write_buffer: 1;
        bool: pub instr_folding_disabled, pub disable_instr_folding: 2;
        bool: pub fpca_autoupdate_disabled, pub disable_fpca_auto_update: 8;
        bool: pub out_of_order_fp_disabled, pub disable_out_of_order_fp: 9;
    }
}

extern "C" {
    pub static mut ACTLR: RegisterCell<ACRegister>;
    pub static mut SCB: scb::SystemControlBlock;
    pub static mut SYSTICK: systick::SystickBlock;
}
