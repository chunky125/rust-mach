//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//

mod elf;

use elf::ELFSectionEntry;
use heapless::{String, Vec};

/// Structure for memory map entry
/// Mirrors Multiboot structure to simplify creation
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMapEntry {
    base_addr: u64,
    length: u64,
    region_type: u32,
    _pad: u32,
}

/// Enum for the tag type
pub enum Tag {
    End {
        tag_size: u32,
    },
    CmdLine {
        tag_size: u32,
        value: String<256>,
    },
    BootLoaderName {
        tag_size: u32,
        name: String<64>,
    },
    Module {
        tag_size: u32,
        mod_start: u32,
        mod_end: u32,
        mod_name: String<64>,
    },
    /* BootDev is not implemented */
    Mmap {
        tag_size: u32,
        table: Vec<MMapEntry, 10>,
    },
    /* VBE and Framebuffer not implemented
     * ELFSections tag is not properly implemented in
     * GRUB, instead we use ELF64Sections
     * APM, EFI 32 not implemented */
    EFI64SystemTable {
        tag_size: u32,
        addr: u64,
    },
    /* SMBIOS, ACPIOLD, ACPI, Network, EFI Mmap not implemented */
    EFIBootServicesRunning {
        tag_size: u32,
    },
    /* EFI32IH not implemented */
    //EFI64ImageHandler {
    //    addr : u64
    //},
    LoadBaseAddress {
        tag_size: u32,
        base_address: u32,
    },
    ELF64Sections {
        tag_size: u32,
        table: Vec<ELFSectionEntry, 20>,
    },
    Unknown {
        tag_size: u32,
    },
}

/// Create a new tag from a memory address, must be aligned to 8 byte boundary
pub fn from_addr(tag_base_addr: u64) -> Option<Tag> {
    // Is this tag address 8 byte aligned
    if tag_base_addr & 0x7 > 0 {
        return None;
    }

    unsafe {
        let new_size: u32 = *((tag_base_addr + 4) as *const u32);
        let new_type_no: u32 = *(tag_base_addr as *const u32);

        match new_type_no {
            // End
            0 => Some(Tag::End { tag_size: new_size }),

            // Command Line
            1 => {
                let mut curr_byte: *const u8 = (tag_base_addr + 8) as *const u8;
                let end_byte: *const u8 = (tag_base_addr + new_size as u64) as *const u8;

                let mut cmdline: String<256> = String::<256>::new();

                while curr_byte < end_byte {
                    if cmdline.push(*curr_byte as char).is_err() {
                        break;
                    }

                    curr_byte = curr_byte.offset(1);
                }

                Some(Tag::CmdLine {
                    tag_size: new_size,
                    value: String::from(cmdline),
                })
            }

            // Bootloader Name
            2 => {
                let mut curr_byte: *const u8 = (tag_base_addr + 8) as *const u8;
                let end_byte: *const u8 = (tag_base_addr + new_size as u64) as *const u8;

                let mut bootloader_name: String<64> = String::<64>::new();

                while curr_byte < end_byte {
                    if bootloader_name.push(*curr_byte as char).is_err() {
                        break;
                    }

                    curr_byte = curr_byte.offset(1);
                }

                Some(Tag::BootLoaderName {
                    tag_size: new_size,
                    name: bootloader_name,
                })
            }

            // Module
            3 => Some(Tag::Module {
                tag_size: new_size,
                mod_start: *((tag_base_addr + 8) as *const u32),
                mod_end: *((tag_base_addr + 12) as *const u32),
                mod_name: String::from("Module"),
            }),

            // Memory Map
            6 => {
                let mmap_entry_count = *((tag_base_addr + 12) as *const u32);
                let mut mmap_table = Vec::<MMapEntry, 10>::new();

                let mmap_curr = (tag_base_addr + 16) as *const MMapEntry;

                for i in 0..mmap_entry_count as isize {
                    let push_result = mmap_table.push(*(mmap_curr.offset(i)));

                    if push_result.is_err() {
                        panic!();
                    }
                }

                Some(Tag::Mmap {
                    tag_size: new_size,
                    table: mmap_table,
                })
            }

            // EFI64 System Table Pointer
            12 => Some(Tag::EFI64SystemTable {
                tag_size: new_size,
                addr: *((tag_base_addr + 8) as *const u64),
            }),

            // EFI Boot Services Running
            18 => Some(Tag::EFIBootServicesRunning { tag_size: new_size }),

            // Load Base Address
            21 => Some(Tag::LoadBaseAddress {
                tag_size: new_size,
                base_address: *((tag_base_addr + 8) as *const u32),
            }),

            // ELF64 Headers
            22 => {
                let mut elf_table = Vec::<ELFSectionEntry, 20>::new();
                let entry = (tag_base_addr + 24) as *const ELFSectionEntry;
                let count = *((tag_base_addr + 8) as *const u16);

                for i in 0..count as isize {
                    let push_result = elf_table.push(*(entry.offset(i)));

                    if push_result.is_err() {
                        panic!();
                    }
                }

                Some(Tag::ELF64Sections {
                    tag_size: new_size,
                    table: elf_table,
                })
            }

            // Something has gone wrong
            _ => Some(Tag::Unknown { tag_size: new_size }),
        }
    }
}

impl Tag {
    /// Return the size of the tag
    pub fn size(&self) -> u32 {
        match self {
            Tag::End { tag_size }
            | Tag::CmdLine { tag_size, .. }
            | Tag::BootLoaderName { tag_size, .. }
            | Tag::Module { tag_size, .. }
            | Tag::Mmap { tag_size, .. }
            | Tag::EFI64SystemTable { tag_size, .. }
            | Tag::EFIBootServicesRunning { tag_size, .. }
            | Tag::LoadBaseAddress { tag_size, .. }
            | Tag::ELF64Sections { tag_size, .. }
            | Tag::Unknown { tag_size, .. } => *tag_size,
        }
    }

    /// Get the size of the tag aligned to "align"
    pub fn size_align(&self, align: u32) -> u32 {
        (self.size() + (align - 1)) & !(align - 1)
    }
}
