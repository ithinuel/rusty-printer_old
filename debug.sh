cargo clean && cargo build -vvv && arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/debug/rusty_bootloader | less
