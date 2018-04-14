cargo clean && cargo build --release -vvv && arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/release/rusty_bootloader | less
