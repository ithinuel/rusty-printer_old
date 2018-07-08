# cargo clean && \

TARGET=thumbv7m-none-eabi

cargo fmt || exit
cargo clippy --target=$TARGET || exit
cargo build --target=$TARGET $@ || exit

arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/debug/rusty_bootloader target/thumbv7m-none-eabi/debug/rusty_bootloader.bin || exit
arm-none-eabi-objcopy -O ihex target/thumbv7m-none-eabi/debug/rusty_bootloader target/thumbv7m-none-eabi/debug/rusty_bootloader.hex || exit
arm-none-eabi-objdump -DSC target/thumbv7m-none-eabi/debug/rusty_bootloader | less
