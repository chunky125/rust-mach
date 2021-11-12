//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//

pub mod tag;

// Use the Tag structure throughout this module
use heapless::Vec;
use tag::Tag;

/// Valid status for the Multiboot2 structure
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Multiboot2Status {
    Valid,
    BadMagic,
    BadAlign,
    NotInit,
    //    TagError,
}

const MULTIBOOT2_BOOTLOADER_MAGIC: u64 = 0x36d76289;

/// Structure for the Multiboot2 Structure
pub struct Multiboot2Info {
    pub status: Multiboot2Status, // Have we checked that this data is valid
    pub tags: Option<Vec<Tag, 10>>, // Multiboot tags
}

/// Create a multiboot2 structure
pub unsafe fn from_addr(init_addr: u64, init_magic: u64) -> Multiboot2Info {
    let mut multiboot2_info = Multiboot2Info {
        status: Multiboot2Status::NotInit,
        tags: None,
    };

    // Check the multiboot2 magic number
    if init_magic != MULTIBOOT2_BOOTLOADER_MAGIC {
        // Invalid magic number
        multiboot2_info.status = Multiboot2Status::BadMagic;
    }

    // Parse the data
    if init_addr & 0x7 != 0 {
        // Multiboot2 data is not aligned correct
        multiboot2_info.status = Multiboot2Status::BadAlign;
    }

    if multiboot2_info.status == Multiboot2Status::NotInit {
        // Do some processing on it
        let mb2infosizeptr: *const u32 = init_addr as *const u32;
        let mut mb2currtagptr: u64 = init_addr + 8;
        let mut new_tags = Vec::<Tag, 10>::new();

        // We have to do some unsafe memory accesses onto the data provided
        // by the bootloader here, then we copy to the various places
        while mb2currtagptr < init_addr + *mb2infosizeptr as u64 {
            let new_tag = tag::from_addr(mb2currtagptr);

            if new_tag.is_some() {
                let tag_value = new_tag.unwrap();

                mb2currtagptr = mb2currtagptr + tag_value.size_align(8) as u64;

                let push_result = new_tags.push(tag_value);

                if push_result.is_err() {
                    panic!();
                }
            }
        }

        // Setup the global static
        multiboot2_info.tags = Some(new_tags);
        multiboot2_info.status = Multiboot2Status::Valid;
    }

    // Return the status
    multiboot2_info
}

impl Multiboot2Info {}
