mod mach {

    mod panic {

       use core::panic::PanicInfo;

        // Panic Handler
        #[panic_handler]
        pub fn panic(_info: &PanicInfo) -> ! {
            loop {}
        }
    }
}
