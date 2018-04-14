cargo clean && cargo build --target=thumbv7m-none-eabi && arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/debug/rusty_bootloader | less
