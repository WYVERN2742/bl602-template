# PineNut/PineCone Rust Template

A simple project as a template for creating rust applications for the Pine64 "Pinenut" BL602 Wifi/Bluetooth SoC.
PineNut Devkits with included USB-C interface are also known as "PineCones".

Thanks to all the existing documentation and repos of other rust projects used in the basis of this.

## Installation

Install rust and cargo with [rustup](https://rustup.rs/ )

Install other dependencies;
```
rustup install nightly-2022-12-25
rustup component add llvm-tools-preview --toolchain nightly-2022-12-25
rustup target add riscv32imac-unknown-none-elf

cargo install cargo-binutils
cargo install --git https://github.com/spacemeowx2/blflash cargo-blflash
```

These commands add the required toolchain targets along with binary flashing for easy development (No need for GUIs anymore 😉).

## Building and Flashing

For a simple compilation which will create a `bl602-template.bin` file ready for flashing
```bash
cargo objcopy --release -- -O binary bl602-template.bin
```

The preferred method is to use `blflash` directly with cargo. (I recommend picking a baudrate across all projects and sticking to it)
```bash
cargo blflash --release --initial-baud-rate 1000000 --baud-rate 1000000 --port COM3
```
