use heapless::{Vec,String};
use typenum::{U10,U64,U256};

pub struct ElfSectionEntry {
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

//
// Structure for memory map entry
//
pub struct MMapEntry {
    base_addr : u64,
    length : u64,
    region_type : u32,
}
//
// Enum for the tag type
//
pub enum TagType {

    End { 
    },
    CmdLine { 
        value : String<U256>,
    },
    BootLoaderName {
        name : String<U64>,
    },
    Module {
        mod_start: u32,
        mod_end: u32,
        mod_name: String<U64>,
    },
    BasicMemInfo {
        mem_lower: u32,
        mem_upper: u32,
    },
    /* BootDev is not implemented */
    Mmap {
        table : Vec<MMapEntry,U10>,
    },
    /* VBE and Framebuffer not implemented
     * ELFSections tag is not properly implemented in 
     * GRUB, instead we use ELF64Sections
     * APM, EFI 32 not implemented */
    EFI64SystemTable {
        addr : u64,
    },
    /* SMBIOS, ACPIOLD, ACPI, Network, EFI Mmap not implemented */
    EFIBootServicesRunning {
    },
    /* EFI32IH not implemented */
    EFI64ImageHandler {
        addr : u64
    },
    LoadBaseAddress {
        base_address: u64,
    },
    ELF64Sections {
        base_address: u64,
        table : Vec<ElfSectionEntry, U10>,
    },
}


pub struct Tag {
    tag_type: TagType,
    tag_size: u32,
}

impl Tag {
    pub fn from_addr(tag_base_addr : u64) -> Option<Tag> {
        unsafe {
            let new_size : u32 = *((tag_base_addr + 4) as *const u32);
            let new_type_no : u32 = *(tag_base_addr as *const u32);

        let mut new_type : TagType;

        match new_type_no {

            /* End */
            0 => {
                new_type = TagType::End {};
            }
            /* EFI64 System Table Pointer */
            12 => {
                new_type = TagType::EFI64SystemTable { 
                    addr : tag_base_addr + 8 };
            
            }

            _ => {
                return None;
            }
        }
    
        Some( Tag{ tag_size : new_size, tag_type : new_type})
        }
    }

    pub fn size(&self) -> u32 {

        self.tag_size
    }

}

