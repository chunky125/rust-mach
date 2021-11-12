#![no_std]
#![no_main]

mod hardware;
pub mod mach;
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
    use hardware::Hardware;
    use multiboot2::{Multiboot2Info, Multiboot2Status};
    use raspi::peripherals::miniuart;

    // Create the state variables
    let multiboot2_info: Multiboot2Info;
    let mut hardware: Hardware;
    let mach: mach::Mach = mach::Mach::new();

    // Initialise the serial port
    let mu = miniuart::MiniUART {};
    let br = miniuart::BaudRate::BaudRate115200;
    mu.init(br);

    // Say Hello!
    mu.send_string("\x1b[2JRust Mach OS, initialising\r\n");

    // Create a MB2 Information structure
    unsafe {
        let multiboot2_info = multiboot2::from_addr(mbinfoaddr, mbmagic);

        if multiboot2_info.status == Multiboot2Status::Valid {
            mu.send_string("Multiboot2 Info is Valid\r\n");

            if multiboot2_info.tags.is_some() {
                for mb2tag in multiboot2_info.tags.unwrap() {
                    use multiboot2::tag::Tag;

                    match mb2tag {
                        Tag::CmdLine { value, .. } => {
                            use core::fmt::Write;
                            use heapless::String;

                            let mut cmdline_text: String<128> = String::<128>::new();

                            if writeln!(cmdline_text, "Command line is {}\r\n", value).is_ok() {
                                mu.send_string(&cmdline_text);
                            }
                        }

                        Tag::BootLoaderName { name, .. } => {
                            use core::fmt::Write;
                            use heapless::String;

                            let mut bootloader_text: String<128> = String::<128>::new();

                            if writeln!(bootloader_text, "Bootloader name is {}\r\n", name).is_ok()
                            {
                                mu.send_string(&bootloader_text);
                            }
                        }

                        Tag::EFI64SystemTable { addr, .. } => {
                            use core::fmt::Write;
                            use heapless::String;

                            let mut systab_text: String<128> = String::<128>::new();

                            if writeln!(systab_text, "EFI System table is at 0x{:x}\r\n", addr)
                                .is_ok()
                            {
                                mu.send_string(&systab_text);
                            }

                            // Try to make EFI work
                            let hardware = hardware::create_from_efi_systab(addr);
                        }

                        Tag::ELF64Sections { table, .. } => {
                            for sheader in table {
                                use core::fmt::Write;
                                use heapless::String;

                                let mut sheader_text: String<128> = String::<128>::new();

                                if writeln!(
                                    sheader_text,
                                    "Section Header, Type {:?}, at 0x{:x}, with length 0x{:x}\r\n",
                                    sheader.section_type,
                                    sheader.section_addr,
                                    sheader.section_size
                                )
                                .is_ok()
                                {
                                    mu.send_string(&sheader_text);
                                }
                            }
                        }

                        _ => {}
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
