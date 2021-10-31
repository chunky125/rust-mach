//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//


pub mod tag;

// Use the Tag structure throughout this module
use tag::Tag;
use heapless::{Vec,String};

/// Valid status for the Multiboot2 structure
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Multiboot2Status {
    Multiboot2StatusValid,
    Multiboot2StatusBadMagic,
    Multiboot2StatusBadAlign,
    Multiboot2StatusNotInit,
}

/// Structure for the Multiboot2 Structure
pub struct Multiboot2Info {
    pub status: Multiboot2Status, // Have we checked that this data is valid
    pub tags : Option<Vec<Tag,10>>, // Multiboot tags
}

/// Global singleton for the Multiboot2 structure
static mut CurrentMultiboot2Info : Multiboot2Info = Multiboot2Info {
    status : Multiboot2Status::Multiboot2StatusNotInit,
    tags : None,
};

impl Multiboot2Info {
    const MULTIBOOT2_BOOTLOADER_MAGIC: u64 = 0x36d76289;

    //
    // Initialise the multiboot2 structure
    //
    pub unsafe fn init(init_addr: u64, init_magic: u64) -> Multiboot2Status {
        // Check the multiboot2 number
        if init_magic != Self::MULTIBOOT2_BOOTLOADER_MAGIC {
            // FIXME: Add log message here
            CurrentMultiboot2Info.status = Multiboot2Status::Multiboot2StatusBadMagic;
        }

        // Parse the data
        if init_addr & 0x7 != 0 {
            // Multiboot2 data is not aligned correct
            // FIXME: Add log message here
            CurrentMultiboot2Info.status = Multiboot2Status::Multiboot2StatusBadAlign;
        }

        if CurrentMultiboot2Info.status == Multiboot2Status::Multiboot2StatusNotInit {

            // Do some processing on it
            let mb2infosizeptr: *const u32 = init_addr as *const u32;
            let mut mb2currtagptr: u64 = init_addr + 8;
            let mut new_tags = Vec::<Tag,10>::new();

            // We have to do some unsafe memory accesses onto the data provided
            // by the bootloader here, then we copy to the various places
            while mb2currtagptr < init_addr + *mb2infosizeptr as u64 {
                
                let new_tag : Tag = Tag::from_addr(mb2currtagptr);

                mb2currtagptr = mb2currtagptr + new_tag.size_align(8) as u64;

                let push_result = new_tags.push (new_tag);

                if push_result.is_err() {
                    panic!();
                }
            }

            // Setup the global static
            CurrentMultiboot2Info.tags = Some(new_tags);
            CurrentMultiboot2Info.status = Multiboot2Status::Multiboot2StatusValid;
        }

        // Return the status
        CurrentMultiboot2Info.status
    }


}

