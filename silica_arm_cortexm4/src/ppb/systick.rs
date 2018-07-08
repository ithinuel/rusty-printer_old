use core::convert::{Into, TryInto};
use core::fmt;
use silica::register::{Field, RegisterCell, RoRegisterCell};

register! {
    @impl_debug;
    /// SysTick Control and Status Register
    #[derive(Copy, Clone)]
    pub struct STCSRegister(u32) {
        /// Indicates whether the counter has counted to 0 since the last read of this register.
        /// This bit is cleared by a read of this register or any write to the Current Value
        /// register.
        bool: pub count_flag, _: 16;
        /// Indicates the SysTick clock source.
        /// If no external clock is provided, this bit reads as true and ignores writes.
        bool: pub using_processor_clock, pub use_processor_clock: 2;
        /// Indicates whether counting to 0 causes the status of the SysTick exception to change to
        /// pending.
        /// Changing the value of the counter to 0 by writing zero to the SysTick Current Value
        /// register to 0 never changes the staus of the SysTick exception.
        bool: pub tick_int_enabled, pub enable_tick_interrupt: 1;
        /// Indicates the enabled status of the SysTick counter
        bool: pub systick_enabled, pub enable_systick: 0;
    }
}
register! {
    @impl_debug;
    /// SysTick Reload Value
    #[derive(Copy, Clone)]
    pub struct STRVRegister(u32) {
        u32: pub reload, pub set_reload: 23, 0;
    }
}
register! {
    @impl_debug;
    /// SysTick Calibration Value
    #[derive(Copy, Clone)]
    pub struct STCRegister(u32) {
        /// Indicates whether the reference clock is implemented.
        bool: pub has_reference_clock, _: 31;
        /// Indicates wheter the 10ms calibration value is inexact.
        bool: pub is_calibration_inexact, _: 30;
    }
}
impl STCRegister {
    pub fn ten_millisecond(self) -> Option<u32> {
        let f = Field::new(23, 0);
        match self.extract(&f) {
            0 => None,
            v => Some(v),
        }
    }
}

/// SysTick Block
/// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0460c/BGBEDEIF.html
#[repr(C)]
pub struct SystickBlock {
    pub control_and_status: RegisterCell<STCSRegister>,
    pub reload_value: RegisterCell<STRVRegister>,
    /// Systick Current value.
    /// Any write to the register clears it to zero.
    pub current_value: RegisterCell<u32>,
    pub calibration: RoRegisterCell<STCRegister>,
}
