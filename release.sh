cargo clean && \
    cargo build --target=thumbv7m-none-eabi --release $@ && \
    arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/rusty_bootloader target/thumbv7m-none-eabi/release/rusty_bootloader.bin && \
    arm-none-eabi-objcopy -O ihex target/thumbv7m-none-eabi/release/rusty_bootloader target/thumbv7m-none-eabi/release/rusty_bootloader.hex && \
    arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/release/rusty_bootloader | less
