#![no_std]
#![no_main]

mod mach;
mod raspi;

fn boot_entry() {

    use raspi::miniuart;
    
    miniuart::init(10);

    miniuart::recv();
    
    loop {}

}

