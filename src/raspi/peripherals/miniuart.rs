//
// Code to use the miniuart on the raspberry pi board
//
use crate::mach::logger::LoggingDevice;
use core::ptr;

pub struct MiniUART {}

pub enum BaudRate {
    BaudRate115200 = 270,
}

//
// Memory structure
//
impl MiniUART {
    //

    // Registers
    //
    // General Pins
    const GPFSEL1: *mut u32 = 0x3F200004 as _;
    // const GPSET0: *mut u32 = 0x3F20001C as _;
    // const GPCLR0: *mut u32 = 0x3F200028 as _;
    const GPPUD: *mut u32 = 0x3F200094 as _;
    const GPPUDCLK0: *mut u32 = 0x3F200098 as _;

    // Auxiallaries
    const AUX_ENABLES: *mut u32 = 0x3F215004 as _;
    const AUX_MU_IO_REG: *mut u32 = 0x3F215040 as _;
    const AUX_MU_IER_REG: *mut u32 = 0x3F215044 as _;
    // const AUX_MU_IIR_REG: *mut u32 = 0x3F215048 as _;
    const AUX_MU_LCR_REG: *mut u32 = 0x3F21504C as _;
    const AUX_MU_MCR_REG: *mut u32 = 0x3F215050 as _;
    const AUX_MU_LSR_REG: *mut u32 = 0x3F215054 as _;
    // const AUX_MU_MSR_REG: *mut u32 = 0x3F215058 as _;
    // const AUX_MU_SCRATCH: *mut u32 = 0x3F21505C as _;
    const AUX_MU_CNTL_REG: *mut u32 = 0x3F215060 as _;
    // const AUX_MU_STAT_REG: *mut u32 = 0x3F215064 as _;
    const AUX_MU_BAUD_REG: *mut u32 = 0x3F215068 as _;

    //
    // Initialise the miniuart
    //
    pub fn init(&self, baud: BaudRate) {
        let mut selector: u32;

        unsafe {
            selector = ptr::read_volatile(Self::GPFSEL1);
        }

        selector &= !(7 << 12);
        selector |= 2 << 2;
        selector &= !(7 << 15);
        selector |= 2 << 15;

        unsafe {
            ptr::write_volatile(Self::GPFSEL1, selector);

            ptr::write_volatile(Self::GPPUD, 0);

            ptr::write_volatile(Self::GPPUDCLK0, (1 << 14) | (1 << 15));

            ptr::write_volatile(Self::GPPUDCLK0, 0);

            ptr::write_volatile(Self::AUX_ENABLES, 1);

            ptr::write_volatile(Self::AUX_MU_CNTL_REG, 0);

            ptr::write_volatile(Self::AUX_MU_IER_REG, 0);

            ptr::write_volatile(Self::AUX_MU_LCR_REG, 3);

            ptr::write_volatile(Self::AUX_MU_MCR_REG, 0);

            ptr::write_volatile(Self::AUX_MU_BAUD_REG, baud as u32);

            ptr::write_volatile(Self::AUX_MU_CNTL_REG, 3);
        }
    }

    //
    // Send a character
    //
    pub fn send(&self, c: u8) {
        unsafe {
            loop {
                if ptr::read_volatile(Self::AUX_MU_LSR_REG) & 0x20 > 0 {
                    break;
                }
            }

            ptr::write_volatile(Self::AUX_MU_IO_REG, c as u32);
        }
    }

    //
    // Receive a character
    pub fn recv(&self) -> u8 {
        unsafe {
            loop {
                if ptr::read_volatile(Self::AUX_MU_LSR_REG) & 0x01 > 0 {
                    break;
                }
            }

            let trunc = ptr::read_volatile(Self::AUX_MU_IO_REG) as u8 & 0xFF;

            trunc
        }
    }

    //
    // Send a string
    //
    pub fn send_string(&self, s: &str) {
        for b in s.bytes() {
            self.send(b);
        }
    }
}

///
/// Implement the logging device trait
///
impl LoggingDevice for MiniUART {
    // Implement the write trait
    fn write(&mut self, buf: &[u8]) -> () {
        for b in buf {
            self.send(*b)
        }
    }
}
