use core::fmt;
use core::convert::{Into, TryInto, TryFrom};
use core::num::TryFromIntError;
use silica::register::{RegisterRoCell, RegisterCell, Field};

#[derive(Debug, Copy, Clone)]
pub struct InvalidEndiannessError(());
#[derive(Debug, Copy, Clone)]
pub enum Endianness {
    Little,
    Big
}
impl TryFrom<u32> for Endianness {
    type Error = InvalidEndiannessError;
    fn try_from(v: u32) -> Result<Endianness, Self::Error> {
        match v {
            0 => Ok(Endianness::Little),
            1 => Ok(Endianness::Big),
            _ => Err(InvalidEndiannessError(()))
        }
    }
}

register! {
    @impl_debug;
    /// CPUID Base Register
    #[derive(Copy, Clone)]
    pub struct CPUIDRegister(u32) {
        u8: pub implementer_code, _: 31, 24;
        u8: pub variant, _: 23, 20;
        u8: pub constant, _: 19, 16;
        u16: pub part_number, _: 15, 4;
        u8: pub revision, _: 3, 0;
    }
}

register! {
    @impl_debug;
    /// Interrupt Control and State Register
    #[derive(Copy, Clone)]
    pub struct ICSRRegister(u32) {
        bool: pub is_nmi_pending, _: 31; // write manually implemented
        bool: pub is_pendsv_pending, _: 28; // write manually implemented
        bool: pub is_systick_pending, _: 26; // write manually implemented
        /// True if any ISR but NMI & FAULT is pending.
        bool: pub is_any_isr_pending, _: 22;
        /// Indicates the exception number of the highest priority pending enabled exception:
        /// 0 = no pending exceptions
        /// Nonzero = the exception number of the highest priority pending enabled exception.
        /// The value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register.
        u8: pub highest_vector_pending, _: 17, 12;
        /// Is `false` if the current context will return to an exception context.
        bool: pub return_to_base, _: 11;
        /// Contains the active exception number:
        /// 0 = Thread mode
        /// Nonzero = The exception number[^a] of the currently active exception.
        /// Note:
        /// Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers.
        ///
        /// [^a]: This is the same value as IPSR bits[8:0], see Interrupt Program Status Register.
        u16: pub active_vector, _: 8, 0;
    }
}

impl ICSRRegister {
    /// Pend an NMI interrupt
    pub fn set_nmi_pending(&mut self) {
        self.insert(&Self::FIELDS[0], 1)
    }

    /// Pend or unpend a PendSV interrupt
    pub fn set_pendsv_pending(&mut self, value: bool) {
        if value {
            self.insert(&Self::FIELDS[1], 1)
        } else {
            let f = Field::new(27, 27);
            self.insert(&f, 1)
        }
    }

    /// Pend or unpend a SysTick interrupt
    pub fn set_systick_pending(&mut self, value: bool) {
        if value {
            self.insert(&Self::FIELDS[2], 1)
        } else {
            let f = Field::new(25, 25);
            self.insert(&f, 1)
        }
    }
}

register! {
    @impl_debug;
    #[derive(Copy, Clone)]
    pub struct VectorTableOffsetRegister(u32) {}
}
impl VectorTableOffsetRegister {
    /// Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map.
    /// Note
    /// Bit[29] determines whether the vector table is in the code or SRAM memory region:
    ///     0 = code
    ///     1 = SRAM.
    /// In implementations bit[29] is sometimes called the TBLBASE bit.
    ///
    /// You must align the offset to the number of exception entries in the
    /// vector table. The minimum alignment is 32 words, enough for up to 16 interrupts. For more
    /// interrupts, adjust the alignment by rounding up to the next power of two. For example, if
    /// you require 21 interrupts, the alignment must be on a 64-word boundary because the required
    /// table size is 37 words, and the next power of two is 64. See your vendor documentation for
    /// the alignment details of your device.
    pub fn offset(&self) -> u32 {
        self.0 & !0x7F
    }

    pub fn set_offset(&mut self, value: u32) {
        #[cfg(target_arch = "arm")]
        {
            let zcount: u32;
            // reverse
            // clz
            unsafe {
                asm!("rbit $0, $1
                      clz $0, $0"
                    : "=r"(zcount) // outputs
                    : "r"(value) // inputs
                    : // clobbers
                    : // no options
                );
            }
            debug_assert!(zcount >= 7); // ensure that address is aligned to a power of 2
        }
        self.0 = value;
    }
}

register! {
    @impl_debug;
    @optout_extract_insert;
    /// Application Interrupt and Reset Control Register
    #[derive(Copy, Clone)]
    pub struct AIRCRegister(u32) {
        Endianness: pub endianness, _: 15;
        u8: pub prigroup, pub set_prigroup: 10, 8;
    }
}
impl AIRCRegister {
    #[inline]
    fn extract(&self, f: &Field) -> u32 {
        (self.0 >> f.lsb()) & f.mask::<u32>()
    }
    #[inline]
    fn insert(&mut self, f: &Field, v: u32) {
        let mask = 0x0000_FFFF & !(f.mask::<u32>() << f.lsb());
        let value = 0x05FA_0000 | ((v & f.mask::<u32>()) << f.lsb());
        self.0 = (self.0 & mask) | value;
    }
    
    #[inline]
    pub fn sys_reset_request(&mut self) {
        let f = Field::new(2, 2);
        self.insert(&f, 1);
    }
}

#[derive(Debug)]
pub struct TryIntoSleepModeError(());

#[derive(Clone, Copy, Debug)]
pub enum SleepMode {
    Sleep,
    DeepSleep
}
impl TryFrom<u32> for SleepMode {
    type Error = TryIntoSleepModeError;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(SleepMode::Sleep),
            1 => Ok(SleepMode::DeepSleep),
            _ => Err(TryIntoSleepModeError(()))
        }
    }
}
impl From<SleepMode> for u32 {
    fn from(v: SleepMode) -> u32 {
        match v {
            SleepMode::Sleep => 0,
            SleepMode::DeepSleep => 1
        }
    }
}

register! {
    @impl_debug;
    /// System Control Register
    pub struct SCRegister(u32) {
        bool: pub sleep_on_exit, _: 1;
        SleepMode: pub sleep_mode,
                   pub set_sleep_mode: 2;
        bool: pub event_sent_on_pending_bit,
              pub send_event_on_pending_bit: 4;
    }
}

register! {
    @impl_debug;
    /// Configuration and Control Register
    pub struct CCRegister(u32) {
        
    }
}

register! {
    @impl_debug;
    /// MemManage Fault Status Register
    pub struct MMSRegister(u8) {
        bool: pub is_valid, _: 7;
        bool: pub fault_on_stacking_for_exception_entry, _: 4;
        bool: pub fault_on_unstacking_for_a_return_from_exception, _: 3;
        bool: pub data_access_violation, _: 1;
        bool: pub instruction_access_violation, _: 0;
    }
}
impl TryFrom<u32> for MMSRegister {
    type Error = TryFromIntError;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        let w = v.try_into()?;
        Ok(MMSRegister(w))
    }
}

register! {
    @impl_debug;
    /// Configurable Fault Status Register
    pub struct CFSRegister(u32) {
        MMSRegister: pub get_mmsr, _: 7, 0;
        // BFSRegister: pub get_bfsr, _: 15, 8;
        // UFSRegister: pub get_ufsr, _: 31, 16;
    }
}

/// System Control Block
/// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/CIHFDJCA.html
#[repr(C)]
pub struct SystemControlBlock {
    pub cpuid: RegisterCell<CPUIDRegister>,
    pub icsr: RegisterCell<ICSRRegister>,
    pub vtor: RegisterCell<VectorTableOffsetRegister>,
    /// Application Interrupt and Reset Control Register
    pub aircr: RegisterCell<AIRCRegister>,
    pub scr: RegisterCell<SCRegister>,
    pub ccr: RegisterCell<u32>,
    pub shp: [RegisterCell<u8>; 12],
    pub shcsr: RegisterCell<u32>,
    pub cfsr: RegisterRoCell<CFSRegister>,
    pub hfsr: RegisterCell<u32>,
    pub dfsr: RegisterCell<u32>,
    pub mmfar: RegisterCell<u32>,
    pub bfar: RegisterCell<u32>,
    pub afsr: RegisterCell<u32>,
    pub pfr: [RegisterCell<u32>; 2],
    pub dfr: RegisterCell<u32>,
    pub adr: RegisterCell<u32>,
    pub mmfr: [RegisterCell<u32>; 4],
    pub isar: [RegisterCell<u32>; 5],
    reserved: [u32; 5],
    pub cpacr: RegisterCell<u32>,
}

extern {
    pub static mut SCB: SystemControlBlock;
}

