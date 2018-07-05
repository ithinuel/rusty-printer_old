//! CM4's Private Peripheral Bus
use core::fmt;
use silica::register::{Field, RegisterCell};

pub mod fpu;
pub mod mpu;
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
    pub static mut CPACR: RegisterCell<fpu::CPACRegister>;
    pub static mut SYSTICK: systick::SystickBlock;
    pub static FR: scb::FeatureRegisters;
    pub static mut MPU: mpu::MemoryProtectionUnit;
    pub static mut FPU: fpu::FloatingPointUnit;
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_allow_interruption() {
        let mut actlr = super::ACRegister(0);
        assert!(!actlr.multiple_cycle_instr_interruptable());
        actlr.allow_interruption_of_multicycle_instr(true);
        assert_eq!(1, actlr.0);
    }
}
