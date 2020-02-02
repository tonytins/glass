# Glass

![Rust](https://github.com/tonytins/glass/workflows/Rust/badge.svg)

Glass is a hobby operating system written in Rust.

## Prerequisites

- Bootimage: ``cargo install bootimage --version "^0.7.7"``
- QEMU

 ## Compiling & Running
 
1. Compile: ``cargo bootimage``
2. Running: ``qemu-system-x86_64 -drive format=raw,file=target/x86_64-glass/debug/bootimage-glass.bin``

## License

