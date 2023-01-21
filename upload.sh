cargo build --release && \
riscv64-unknown-elf-objcopy -O binary \
    target/riscv32i-unknown-none-elf/release/orangecrab_blink target/orangecrab_blink.dfu && \
dfu-suffix -v 1209 -p 5bf0 -a target/orangecrab_blink.dfu && \
dfu-util -a 0 -D target/orangecrab_blink.dfu
