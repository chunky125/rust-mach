#![no_std]
#![no_main]

mod mach;
mod multiboot2;
mod raspi;

//
// Main entry point into the kernel from the ASM initialisation code
// a, b, c and d are variables passed according to multiboot spec
//
#[no_mangle]
fn boot_entry(a: u64, b : u64, c : u64, d : u64) {

    use raspi::peripherals::miniuart;

    let mu = miniuart::MiniUART {};
    let br = miniuart::BaudRate::BaudRate115200;

    mu.init(br);

    loop {

        mu.send(mu.recv());
    }

}

