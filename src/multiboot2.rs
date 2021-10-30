//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//
// FIXME: This should be a singleton

use heapless::{Vec,String};

pub struct Multiboot2 {
    addr: u64,   // Pointer to rawdata in memory
    magic: u64,  // Magic number
    valid: bool, // Have we checked that this data is valid
    pub tags : Option<Vec<Tag,10>>, // Multiboot tags
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
                tags: None,
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
                tags: None,
            };
        }

        // Do some processing on it
        let mb2infosizeptr: *const u32 = init_addr as *const u32;
        let mut mb2currtagptr: u64 = init_addr + 8;
        let mut new_tags = Vec::<Tag,10>::new();

        // We have to do some unsafe memory accesses onto the data provided
        // by the bootloader here, then we copy to the various places
        unsafe {
            while mb2currtagptr < init_addr + *mb2infosizeptr as u64 {
                
                let new_tag : Tag = Tag::from_addr(mb2currtagptr);

                mb2currtagptr = mb2currtagptr + new_tag.size_align(8) as u64;

                let push_result = new_tags.push (new_tag);

                if push_result.is_err() {
                    panic!();
                } else {
                    // Log an error
                }
            }
        }

        // return new info
        return Multiboot2 {
            addr: init_addr,
            magic: init_magic,
            valid: true,
            tags: Some(new_tags),
        };
    }

    pub fn valid(&self) -> bool {
        return self.valid;
    }
}

/// Structure for Elf Section Table Entry
pub struct ELFSectionEntry {
    section_name_addr : u32,
    section_type : u32,
    section_flags : u64,
    section_addr : u64,
    section_size : u64,
    section_link : u32,
    section_info : u32,
    section_addr_align : u64,
    section_entsize : u64,
}

/// Structure for memory map entry
pub struct MMapEntry {
    base_addr : u64,
    length : u64,
    region_type : u32,
}

/// Enum for the tag type
pub enum Tag {

    End { 
        tag_size : u32,
    },
    CmdLine { 
        tag_size : u32,
        value : String<256>,
    },
    BootLoaderName {
        tag_size : u32,
        name : String<64>,
    },
    Module {
        tag_size : u32,
        mod_start: u32,
        mod_end: u32,
        mod_name: String<64>,
    },
    /* BootDev is not implemented */
    Mmap {
        tag_size : u32,
        table : Vec<MMapEntry,10>,
    },
    /* VBE and Framebuffer not implemented
     * ELFSections tag is not properly implemented in 
     * GRUB, instead we use ELF64Sections
     * APM, EFI 32 not implemented */
    EFI64SystemTable {
        tag_size : u32,
        addr : u64,
    },
    /* SMBIOS, ACPIOLD, ACPI, Network, EFI Mmap not implemented */
    EFIBootServicesRunning {
        tag_size : u32,
    },
    /* EFI32IH not implemented */
    //EFI64ImageHandler {
    //    addr : u64
    //}, 
    LoadBaseAddress {
        tag_size : u32,
        base_address: u32,
    },
    ELF64Sections {
        tag_size : u32,
        table : Vec<ELFSectionEntry, 10>,
    },
    Unknown {
        tag_size : u32,
    },
}

impl Tag {
    pub fn from_addr(tag_base_addr : u64) -> Tag {
        unsafe {
            let new_size : u32 = *((tag_base_addr + 4) as *const u32);
            let new_type_no : u32 = *(tag_base_addr as *const u32);

            match new_type_no {

                // End 
                0 => {
                    Tag::End { 
                        tag_size : new_size 
                    }
                }

                // Command Line
                1 => {
                    Tag::CmdLine {
                        tag_size : new_size,
                        value : String::from("CommandLine"),
                    }
                }

                // Bootloader Name
                2 => {
                    Tag::BootLoaderName {
                        tag_size : new_size,
                        name : String::from("BootLoaderName"),
                    }
                }

                // Module
                3 => {
                    Tag::Module {
                        tag_size : new_size,
                        mod_start : *((tag_base_addr + 8) as *const u32),
                        mod_end : *((tag_base_addr + 12) as *const u32),
                        mod_name : String::from("Module"),
                    }
                }

                // Memory Map
                6 => {
                    let mmap_entry_size = *((tag_base_addr + 8) as *const u32);
                    let mmap_entry_count = *((tag_base_addr + 12) as *const u32);
                    let mut mmap_table = Vec::<MMapEntry,10>::new();

                    let mut mmap_curr_addr = tag_base_addr + 16;

                    for i in 0..mmap_entry_count {
                        let new_entry = MMapEntry { 
                            base_addr : *(mmap_curr_addr as *const u64),
                            length : *((mmap_curr_addr + 8) as *const u64),
                            region_type : *((mmap_curr_addr + 16) as *const u32),
                        };
                        
                        let push_result = mmap_table.push(new_entry);

                        if push_result.is_err() {
                            panic!();
                        }

                        mmap_curr_addr += mmap_entry_size as u64;
                    }
                        
                    Tag::Mmap {
                        tag_size : new_size,
                        table : mmap_table,
                    }
                }
        
                
                // EFI64 System Table Pointer
                12 => {
                    Tag::EFI64SystemTable { 
                        tag_size : new_size,
                        addr : *((tag_base_addr + 8) as *const u64) 
                    }
                }  

                // EFI Boot Services Running
                18 => {
                    Tag::EFIBootServicesRunning {
                        tag_size : new_size,
                    }
                }

                // Load Base Address
                21 => {
                    Tag::LoadBaseAddress {
                        tag_size : new_size,
                        base_address : *((tag_base_addr + 8) as *const u32) 
                    }
                }
                
                // ELF64 Headers
                22 => {
                    let new_table = Vec::<ELFSectionEntry,10>::new();

                    Tag::ELF64Sections {
                        tag_size : new_size,
                        table : new_table,
                    }
                }
                
                // Something has gone wrong
                _ => {
                    
                    Tag::Unknown {
                        tag_size : new_size,
                    }
                }
            }   
        }
    }

    pub fn size(&self) -> u32 {

        match self {
            Tag::End { tag_size } |
            Tag::CmdLine { tag_size, .. } |
            Tag::BootLoaderName { tag_size, .. } |
            Tag::Module { tag_size, .. } |
            Tag::Mmap { tag_size, .. } | 
            Tag::EFI64SystemTable { tag_size, .. } |
            Tag::EFIBootServicesRunning { tag_size, .. } |
            Tag::LoadBaseAddress { tag_size, .. } | 
            Tag::ELF64Sections { tag_size, .. } |           
            Tag::Unknown { tag_size, .. } => {
                *tag_size
            }
            
            _ => {
                0
            }
        }
    }

    pub fn size_align(&self, align : u32) -> u32 {
        
        (self.size() + (align - 1)) & !(align-1)
    }

}

