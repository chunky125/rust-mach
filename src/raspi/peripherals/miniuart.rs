//
// Code to use the miniuart on the raspberry pi board
// 

use heapless::String;
use volatile_register::{RW};

// 
// Memory structure
//
#[repr(C)]
pub struct MiniUART {
    pub io : RW<u32>,
    pub ier : RW<u32>,
    pub iir : RW<u32>,
    pub lcr : RW<u32>,
    pub mcr : RW<u32>,
    pub lsr : RW<u32>,
    pub msr : RW<u32>,
    pub scratch : RW<u32>,
    pub cntl : RW<u32>,
    pub stat : RW<u32>,
    pub baud : RW<u32>,
}

impl MiniUART {
    //
    // Initialise the miniuart
    // 
    pub fn init(baud : u32) {

        let mut selector : u32;

        selector = 0;

    }   

    // 
    // Send a character
    // 
    pub fn send(c : char) {
    
    }

    //
    // Receive a character
    pub fn recv() -> char {
        'C'
    }

    // 
    // Send a string
    //
    //pub fn send_string(s : String) {
    
    //}
}

