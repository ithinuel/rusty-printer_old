#![cfg_attr(not(test), feature(used))]
#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate silica_duet3d_duet2;

#[no_mangle]
pub fn main() {
    let mut _a: u32 = 0;

    while _a < 4000 {
        _a += 1;
    }
}


