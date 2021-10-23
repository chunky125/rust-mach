//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//
// FIXME: This should be a singleton

mod tag;

use tag::Tag;

pub struct Multiboot2 {
    addr: u64,   // Pointer to rawdata in memory
    magic: u64,  // Magic number
    valid: bool, // Have we checked that this data is valid
}

impl Multiboot2 {
    const MULTIBOOT2_BOOTLOADER_MAGIC: u64 = 0x36d76289;

    //
    // Initialise the multiboot2 structure
    //
    pub fn create(init_addr: u64, init_magic: u64) -> Multiboot2 {
        // Check the multiboot2 number
        if init_magic != Self::MULTIBOOT2_BOOTLOADER_MAGIC {
            // FIXME: Add log message here
            return Multiboot2 {
                addr: init_addr,
                magic: init_magic,
                valid: false,
            };
        }

        // Parse the data
        if init_addr & 0x7 != 0 {
            // Multiboot2 data is not aligned correct
            // FIXME: Add log message here
            return Multiboot2 {
                addr: init_addr,
                magic: init_magic,
                valid: false,
            };
        }

        // Do some processing on it
        let mb2infosizeptr: *const u32 = init_addr as *const u32;
        let mut mb2currtagptr: u64 = init_addr + 8;

        // We have to do some unsafe memory accesses onto the data provided
        // by the bootloader here, then we copy to the various places
        unsafe {
            while mb2currtagptr < init_addr + *mb2infosizeptr as u64 {

                let currtag : Tag = Tag::from_addr(mb2currtagptr);

                mb2currtagptr = mb2currtagptr + (currtag.size() as u64);
            }
        }

        // return new info
        return Multiboot2 {
            addr: init_addr,
            magic: init_magic,
            valid: true,
        };
    }

    pub fn valid(&self) -> bool {
        return self.valid;
    }
}
