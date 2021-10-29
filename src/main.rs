#![no_std]
#![no_main]

mod mach;
mod multiboot2;
mod raspi;

//
// Main entry point into the kernel from the ASM initialisation code
// mbmagic should be the bootloader magic, mbinfoaddr is the address of
// the multiboot info.
//
// The 3rd and 4th arguments could be passed by Multuboot but we're
// not using them and neither is GRUB
//
#[no_mangle]
fn boot_entry(mbmagic: u64, mbinfoaddr: u64, _bootarg3: u64, _bootarg4: u64) {
    use raspi::peripherals::miniuart;

    // Initialise the serial port
    let mu = miniuart::MiniUART {};
    let br = miniuart::BaudRate::BaudRate115200;
    mu.init(br);

    // Say Hello!
    mu.send_string("\x1b[2JRust Mach OS, initialising\r\n");

    // Create a MB2 Information structure - this is a singleton
    use multiboot2::Multiboot2;
    let mbinfo: Multiboot2 = Multiboot2::create(mbinfoaddr, mbmagic);

    if mbinfo.valid() == true {
        mu.send_string("Multiboot2 Info is Valid\r\n");
    }

    // Start the Kernel itself
    loop {
        mu.send(mu.recv());
    }
}
