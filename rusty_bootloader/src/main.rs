#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate silica_duet3d_duet2;

#[cfg_attr(not(test), no_mangle)]
pub fn main() {
    let mut _a: u32 = 0;

    while _a < 100 {
        _a += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
