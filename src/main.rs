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

    // Create a MB2 Information structure - this is a singleton - how to implement it!
    use multiboot2::Multiboot2;
    let mbinfo: Multiboot2 = Multiboot2::create(mbinfoaddr, mbmagic);

    if mbinfo.valid() == true {

        mu.send_string("Multiboot2 Info is Valid\r\n");

        if mbinfo.tags.is_some() {
            for mb2tag in mbinfo.tags.unwrap() {

                use multiboot2::Tag;
                
                match mb2tag {

                    Tag::CmdLine {value, .. } => {
                        mu.send_string("Found Command Line MB2 Tag\r\n");
                    }

                    Tag::EFI64SystemTable{addr, .. } => {
                        use heapless::String;
                        use core::fmt::Write;

                        let mut systab_text : String::<128> = String::<128>::new();

                        writeln!(systab_text,"EFI System table is at {:x}\r\n", addr);

                        mu.send_string(&systab_text);
                    }

                    _ => {
                    }
                }
            }
        }
    }


    // Start the Kernel itself
    loop {
        mu.send(mu.recv());
    }
}
