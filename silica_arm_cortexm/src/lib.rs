#![cfg_attr(not(test), no_std)]

extern crate silica;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
