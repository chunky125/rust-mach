#![no_std]
#![no_main]

#[allow(dead_code)]

mod mach {

    fn boot_entry() {
        loop {}
    }

}

use core::panic::PanicInfo;

// Panic Handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
