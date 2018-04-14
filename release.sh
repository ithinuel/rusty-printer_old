cargo clean && cargo build --target=thumbv7m-none-eabi --release && arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/release/rusty_bootloader | less
