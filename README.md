# Glass

![Rust](https://github.com/tonytins/glass/workflows/Rust/badge.svg)

Glass is a hobby operating system written in Rust, based on Philipp Oppermann's [Writing an OS in Rust](https://os.phil-opp.com/).

## Prerequisites

- Bootimage: ``cargo install bootimage --version "^0.7.7"``
- QEMU

## Compiling & Running

1. Compile: ``cargo bootimage``
2. Running: ``qemu-system-x86_64 -drive format=raw,file=target/x86_64-glass/debug/bootimage-glass.bin``

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.
