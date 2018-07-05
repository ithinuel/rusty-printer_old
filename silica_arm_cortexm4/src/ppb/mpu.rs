use core::convert::{Into, TryInto};
use silica::register::{Field, RegisterCell};

register! {
    pub struct TypeRegister(u32) {
        bool: pub separate, _: 0;
        u8: pub data_regions, _: 15, 8;
        u8: pub inst_regions, _: 23, 16;
    }
}

register! {
    pub struct ControlRegister(u32) {
        bool: pub enabled, pub enable: 0;
        bool: pub hf_nmi_enabled, pub enable_in_hf_and_nmi: 1;
        bool: pub default_map_in_privileged_enabled, pub enable_default_map_in_privileged: 2;
    }
}

register! {
    pub struct RegionNumberRegister(u32) {
        u8: pub region, pub set_region: 7, 0;
    }
}

register! {
    pub struct RegionBaseAddressRegister(u32) {
        /// The ADDR field is bits[31:N] of the MPU_RBAR. The region size, as specified by the SIZE field in the MPU_RASR, defines the value of N:
        ///    N = Log2(Region size in bytes),
        /// If the region size is configured to 4GB, in the MPU_RASR, there is no valid ADDR field. In this case, the region occupies the complete memory map, and the base address is 0x00000000.
        /// The base address is aligned to the size of the region. For example, a 64KB region must be aligned on a multiple of 64KB, for example, at 0x00010000 or 0x00020000.
        u32: pub addr, pub set_addr: 31, 5;
        bool: _, pub valid: 4;
        u8: pub region, pub set_region: 3, 0;
    }
}

register! {
    /// TODO: This register requires some special mechanics to validate the values before writing
    /// to memory as some fields aren't available depending on others value.
    /// e.g.: SubRegionDisable is only available if Size > 128bytes
    pub struct RegionAttributeAndSizeRegister(u32) {
        bool: pub enabled, pub enable: 0;
        u8: pub size, pub set_size: 5, 1;
        // missing fields here !

    }
}

#[repr(C)]
pub struct MemoryProtectionUnit {
    pub mpu_type: RegisterCell<TypeRegister>,
    pub control: RegisterCell<ControlRegister>,
    pub rnr: RegisterCell<RegionNumberRegister>,
    pub rbar: RegisterCell<RegionBaseAddressRegister>,
    pub rsar: RegisterCell<RegionAttributeAndSizeRegister>,
    pub rbar_a1: RegisterCell<RegionBaseAddressRegister>,
    pub rsar_a1: RegisterCell<RegionAttributeAndSizeRegister>,
    pub rbar_a2: RegisterCell<RegionBaseAddressRegister>,
    pub rsar_a2: RegisterCell<RegionAttributeAndSizeRegister>,
    pub rbar_a3: RegisterCell<RegionBaseAddressRegister>,
    pub rsar_a3: RegisterCell<RegionAttributeAndSizeRegister>,
}
