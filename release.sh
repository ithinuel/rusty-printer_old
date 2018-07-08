# cargo clean

TARGET=thumbv7m-none-eabi

cargo fmt || exit
cargo clippy --target=$TARGET || exit
cargo build --target=$TARGET --release $@ || exit

arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/rusty_bootloader target/thumbv7m-none-eabi/release/rusty_bootloader.bin && \
arm-none-eabi-objcopy -O ihex target/thumbv7m-none-eabi/release/rusty_bootloader target/thumbv7m-none-eabi/release/rusty_bootloader.hex && \
arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/release/rusty_bootloader | less
