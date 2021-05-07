# rust-mach
GNU Mach Compatible Kernel in Rust

## Introduction
This project aims to create a kernel which is compatible with GNU Mach, but is written in Rust.

The primary platform is the Raspberry Pi 3 and above, on the basis that this is ARM64, and easily available.

## Booting
The kernel is Multiboot2 compatible, and hence requires Grub (with a patch), after using 
