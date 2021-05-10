#![no_std]
#![no_main]

mod mach;
mod multiboot2;
mod raspi;

//
// Main entry point into the kernel from the ASM initialisation code
// 
fn boot_entry() {

    use raspi::peripherals;
   
    loop {}

}

