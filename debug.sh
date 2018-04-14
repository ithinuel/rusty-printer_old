cargo clean && \
    cargo build --target=thumbv7m-none-eabi && \
    arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/debug/rusty_bootloader target/thumbv7m-none-eabi/debug/rusty_bootloader.bin && \
    arm-none-eabi-objcopy -O ihex target/thumbv7m-none-eabi/debug/rusty_bootloader target/thumbv7m-none-eabi/debug/rusty_bootloader.hex && \
    arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/debug/rusty_bootloader | less
