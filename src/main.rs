#![no_std]
#![no_main]

mod mach;
mod multiboot2;
mod raspi;

//
// Main entry point into the kernel from the ASM initialisation code
//
#[no_mangle]
fn boot_entry() {

    use raspi::peripherals::miniuart;

    let mu = miniuart::MiniUART {};
    let br = miniuart::BaudRate::BaudRate115200;

    mu.init(br);

    mu.send_string("Testing");
  
    loop {

        mu.send(mu.recv());
    }

}

