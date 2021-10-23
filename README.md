# rust-mach
Moving my mucking around from GNU Mach on Raspberry Pi into mucking around with Rust on Raspberry Pi

## Introduction
This project is a place for me to continue my GNU Mach inspired mucking around on the Raspberry Pi but in Rust.

The primary platform is the Raspberry Pi 3 and above, on the basis that this is ARM64, and easily available.

## Booting
The kernel is Multiboot2 compatible, and hence requires Grub (with a patch), after using U-Boot to establish a common basis.
