use core::ptr;
use core::ops::{BitOr, BitAnd, Not};
use core::mem::size_of;

use bitfield::BitRange;

#[derive(Debug)]
pub struct VolatileCell<T>(T);
impl<T> VolatileCell<T> where
    T: Not<Output = T> + BitAnd<Output = T> + BitOr<Output = T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }
    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(&mut self.0, value) }
    }
    pub fn update(&mut self, value: T, mask: T) {
        let v = self.read() & !mask;
        self.write(v | value);
    }
}
macro_rules! impl_bitrange_for_vc_tu {
    ($t:ty, $u:ty) => {
        impl BitRange<$t> for VolatileCell<$u> {
            fn bit_range(&self, msb: usize, lsb: usize) -> $t {
                debug_assert!(msb < size_of::<$u>()*8, "The msb must be smaller than the cell size.");
                let width = msb - lsb + 1;
                debug_assert!(width <= size_of::<$t>()*8,
                              "The field must be smaller that the return type");
                let mask = (1 << width) - 1;
                ((self.read() >> lsb) & mask) as $t
            }
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $t) {
                debug_assert!(msb < size_of::<$u>()*8, "The msb must be smaller than the cell size.");
                let width = msb - lsb + 1;
                let mask = (1 << width) - 1;
                self.update((value as $u) << lsb, mask << lsb)
            }
        }
    }
}
impl_bitrange_for_vc_tu!(u8,  u8);

impl_bitrange_for_vc_tu!(u8,  u16);
impl_bitrange_for_vc_tu!(u16, u16);

impl_bitrange_for_vc_tu!(u8,  u32);
impl_bitrange_for_vc_tu!(u16, u32);
impl_bitrange_for_vc_tu!(u32, u32);

#[cfg(test)]
mod tests {
    use super::VolatileCell;
    use bitfield::{Bit, BitRange};
    #[test]
    #[should_panic(expected = "The msb must be smaller than the cell size.")]
    fn test_erroneous_field_size_on_set_bit_range() {
        let mut vc = VolatileCell(0u16);
        vc.set_bit_range(24, 0, 23u8);
    }

    #[test]
    #[should_panic(expected = "The msb must be smaller than the cell size.")]
    fn test_erroneous_field_size_on_bit_range() {
        let vc = VolatileCell(0u16);
        let _: u16 = vc.bit_range(24, 0);
    }
    
   #[test]
    fn test_volatile_cell() {
        let mut vc = VolatileCell(0u32);
        
        <VolatileCell<u32> as BitRange<u8>>::set_bit_range(&mut vc, 3, 3, 1u8);
        assert_eq!(0x08, vc.0);
        vc.set_bit(3, false);
        assert_eq!(0x00, vc.0);
        vc.set_bit(4, true);
        assert_eq!(0x10, vc.0);
    }
}
