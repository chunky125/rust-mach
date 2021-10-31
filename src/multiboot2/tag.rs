//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//

use heapless::{Vec,String};
use core::mem::size_of;

/// Structure for Elf Section Table Entry
/// Mirrors ELF section headers to simplify creation
#[repr (C)]
#[derive (Clone,Copy)]
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
/// Mirrors Multiboot structure to simplify creation
#[repr (C)]
#[derive (Clone,Copy)]
pub struct MMapEntry {
    base_addr : u64,
    length : u64,
    region_type : u32,
    pad : u32,
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
        table : Vec<ELFSectionEntry, 20>,
    },
    Unknown {
        tag_size : u32,
    },
}

impl Tag {

    /// Generate a new tag from a memory address, which must be 
    /// aligned to an 8 byte boundary
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
                    assert_eq!( *((tag_base_addr + 8) as *const u32) as usize,size_of::<MMapEntry>());
                    let mmap_entry_count = *((tag_base_addr + 12) as *const u32);
                    let mut mmap_table = Vec::<MMapEntry,10>::new();

                    let mmap_curr = (tag_base_addr + 16) as *const MMapEntry;
                    
                    for i in 0..mmap_entry_count as isize {
                        let push_result = mmap_table.push(*(mmap_curr.offset(i)));

                        if push_result.is_err() {
                            panic!();
                        }

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
<<<<<<< HEAD
                    assert_eq!(*((tag_base_addr + 12) as *const u16) as usize,size_of::<ELFSectionEntry>());

                    let mut elf_table = Vec::<ELFSectionEntry,20>::new();
                    let entry = (tag_base_addr + 16) as *const ELFSectionEntry;
                    let count = 
                        *((tag_base_addr + 8) as *const u16);
                                            
                    for i in 0..count as isize {
                        let push_result = elf_table.push(*(entry.offset(i)));

                        if push_result.is_err() {
                            panic!();
                        }
=======
                    let new_table = Vec::<ELFSectionEntry,10>::new();
                    let mut entry : u64 = tag_base_addr + 16;
                    let count : u16 = 
                        *((tag_base_addr + 8) as *const u16);
                    let entry_size : u16 = 
                        *((tag_base_addr + 12) as *const u16);

                    while entry < 
                        (tag_base_addr + 16 + (count * entry_size) as u64) {
                        entry = entry + entry_size as u64;
                    }
>>>>>>> 3c2fcf9732d5d71140624f8eed09a8b118dd44bc

                    }
                        
                    Tag::ELF64Sections {
                        tag_size : new_size,
                        table : elf_table,
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

    /// Return the size of the tag
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
        }
    }

    /// Get the align, aligned size of the tag
    pub fn size_align(&self, align : u32) -> u32 {
        
        (self.size() + (align - 1)) & !(align-1)
    }

}

