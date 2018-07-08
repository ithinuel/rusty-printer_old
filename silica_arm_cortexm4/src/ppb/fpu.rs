use core::convert::{Into, TryFrom, TryInto};
use core::fmt;
use silica::register::{Field, RegisterCell};

#[derive(Debug)]
pub struct TryIntoCoProcessorAccessError(());

#[derive(Clone, Copy, Debug)]
pub enum CoProcessorAccess {
    /// Any attempted access generates a NOCP UsageFault.
    AccessDenied,
    /// An unprivileged access generates a NOCP fault.
    PrivilegedOnly,
    FullAccess,
}
impl TryFrom<u32> for CoProcessorAccess {
    type Error = TryIntoCoProcessorAccessError;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(CoProcessorAccess::AccessDenied),
            1 => Ok(CoProcessorAccess::PrivilegedOnly),
            3 => Ok(CoProcessorAccess::FullAccess),
            _ => Err(TryIntoCoProcessorAccessError(())),
        }
    }
}
impl From<CoProcessorAccess> for u32 {
    fn from(v: CoProcessorAccess) -> u32 {
        match v {
            CoProcessorAccess::AccessDenied => 0,
            CoProcessorAccess::PrivilegedOnly => 1,
            CoProcessorAccess::FullAccess => 3,
        }
    }
}

register! {
    @impl_debug;
    /// CoProcessor Access Control Register
    #[derive(Copy, Clone)]
    pub struct CPACRegister(u32) {
        CoProcessorAccess: pub get_cp10_access, pub set_cp10_access: 21, 20;
        CoProcessorAccess: pub get_cp11_access, pub set_cp11_access: 23, 22;
    }
}

register! {
    /// Floating-point Context Control Register
    #[derive(Copy, Clone)]
    pub struct FPCCRegister(u32) {
    }
}

register! {
    /// Floating-point Context Address Register
    #[derive(Copy, Clone)]
    pub struct FPCARegister(u32) {
    }
}

register! {
    /// Floating-point Status Control Register
    /// This register is not memory mapped and require a specific instruction to be read/written.
    #[derive(Copy, Clone)]
    pub struct FPSCRegister(u32) {
    }
}

register! {
    /// Floating-point Default Status Control Register
    #[derive(Copy, Clone)]
    pub struct FPDSCRegister(u32) {
    }
}

#[repr(C)]
pub struct FloatingPointUnit {
    pub ccr: RegisterCell<FPCCRegister>,
    pub car: RegisterCell<FPCARegister>,
    pub scr: RegisterCell<FPSCRegister>,
    pub dscr: RegisterCell<FPDSCRegister>,
}
