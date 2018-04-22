use core::mem::size_of;
use core::ptr::Unique;

use bitfield::BitRange;

use registers::VolatileCell;

bitfield! {
    /// CPUID Base Register
    pub struct CPUIDRegister(VolatileCell<u32>);
    impl Debug;
    pub u8, implementer_code, _: 31, 24;
    pub u8, variant, _: 23, 20;
    pub u8, constant, _: 19, 16;
    pub u16, part_number, _: 15, 4;
    pub u8, revision, _: 3, 0;
}

bitfield! {
    /// Interrupt Control and State Register
    pub struct ICSRRegister(VolatileCell<u32>);
    impl Debug;
    u32;
    pub is_nmi_pending, _: 31; // write manually implemented
    pub is_pendsv_pending, _: 28; // write manually implemented
    pub is_systick_pending, _: 26; // write manually implemented
    /// True if any ISR but NMI & FAULT is pending.
    pub is_any_isr_pending, _: 22;
    /// Indicates the exception number of the highest priority pending enabled exception:
    /// 0 = no pending exceptions
    /// Nonzero = the exception number of the highest priority pending enabled exception.
    /// The value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register.
    pub u8, highest_vector_pending, _: 17, 12;
    /// Is `false` if the current context will return to an exception context.
    pub return_to_base, _: 11;
	/// Contains the active exception number:
    /// 0 = Thread mode
    /// Nonzero = The exception number[^a] of the currently active exception.
    /// Note:
    /// Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers.
    /// 
    /// [^a]: This is the same value as IPSR bits[8:0], see Interrupt Program Status Register.
    pub u16, active_vector, _: 8, 0;
}
impl ICSRRegister {
    /// Pend an NMI interrupt
    pub fn set_nmi_pending(&mut self) {
        self.set_bit_range(31, 31, 1u8)
    }
    
    /// Pend or unpend a PendSV interrupt
    pub fn set_pendsv_pending(&mut self, value: bool) {
        if value {
            self.set_bit_range(28, 28, 1u8)
        } else {
            self.set_bit_range(27, 27, 1u8)
        }
    }
    
    /// Pend or unpend a SysTick interrupt
    pub fn set_systick_pending(&mut self, value: bool) {
        if value {
            self.set_bit_range(26, 26, 1u8)
        } else {
            self.set_bit_range(25, 25, 1u8)
        }
    }
}

bitfield! {
    pub struct VectorTableOffsetRegister(VolatileCell<u32>);
    impl Debug;
    u32;
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
        <Self as BitRange<u32>>::bit_range(self, 31, 7) << 7
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
        <Self as BitRange<u32>>::set_bit_range(self, 31, 7, value >> 7)
    }
}

bitfield! {
    /// Application Interrupt and Reset Control Register
    pub struct AIRCRegister(VolatileCell<u32>);
    no default BitRange;
    impl Debug;
    u8;
    pub _, sys_reset_req: 2;
    pub prigroup, set_prigroup: 10, 8;
    pub endianness, _: 15;
}
impl BitRange<u8> for AIRCRegister {
    fn bit_range(&self, msb: usize, lsb: usize) -> u8 {
        self.0.bit_range(msb, lsb)
    }

    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: u8) {
        debug_assert!(msb < size_of::<Self>()*8, "The msb must be smaller than the cell size.");
        let width = msb - lsb + 1;
        let mask = (1 << width) - 1;
        
        self.0.write((0x05FA << 16) | (((value as u32) & mask) << lsb));
    }
}

/// System Control Block
/// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/CIHFDJCA.html
#[repr(C)]
pub struct SystemControlBlock {
    pub cpuid: CPUIDRegister,
    pub icsr: ICSRRegister,
    pub vtor: VectorTableOffsetRegister,
    /// Application Interrupt and Reset Control Register
    pub aircr: AIRCRegister,
    pub scr: VolatileCell<u32>,
    pub ccr: VolatileCell<u32>,
    pub shp: [VolatileCell<u8>; 12],
    pub shcsr: VolatileCell<u32>,
    pub cfsr: VolatileCell<u32>,
    pub hfsr: VolatileCell<u32>,
    pub dfsr: VolatileCell<u32>,
    pub mmfar: VolatileCell<u32>,
    pub bfar: VolatileCell<u32>,
    pub afsr: VolatileCell<u32>,
    pub pfr: [VolatileCell<u32>; 2],
    pub dfr: VolatileCell<u32>,
    pub adr: VolatileCell<u32>,
    pub mmfr: [VolatileCell<u32>; 4],
    pub isar: [VolatileCell<u32>; 5],
    reserved: [u32; 5],
    pub cpacr: VolatileCell<u32>
}

bitfield! {
    pub struct AuxiliaryControlRegister(VolatileCell<u32>);
    impl Debug;
    u32;
    pub is_load_store_multiple_interruption_disabled, disable_load_store_multiple_interruption, _: 0;
    pub is_default_write_buffer_disabled, disable_default_write_buffer, _: 1;
    pub is_folding_disabled, disable_folding, _: 2;
    pub is_ctrl_fpca_auto_update_disabled, disable_ctrl_fpca_auto_update, _: 8;
    pub is_out_of_order_fp_disabled, disable_out_of_order_fp, _: 9;
}

pub const ACTLR: Unique<AuxiliaryControlRegister> = unsafe { Unique::new_unchecked(0xE000E008 as *mut _) };

pub const SCB: Unique<SystemControlBlock> = unsafe { Unique::new_unchecked(0xE000ED00 as *mut _) };
