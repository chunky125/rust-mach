//
// Enum for the tag type
//
//

pub enum Type {
    End { 
    },
    CmdLine { 
    },
    BootLoaderName {
    },
    /*Module = 3,
    BasicMemInfo = 4,
    BootDev = 5,
    Mmap = 6,
    VBE = 7,
    FrameBuffer = 8,
    ELFSections = 9,
    APM = 10,
    EFI32 = 11,
    EFI64 = 12,
    SMBIOS = 13,
    ACPIOLD = 14,
    ACPI = 15,
    Network = 16,
    EFIMmap = 17,
    EFIBootServices = 18,
    EFI32IH = 19,
    EFI64IH = 20, */
    LoadBaseAddress {
        base_address: u64,
    },
    ELF64Sections {
        base_address: u64,
    },
}


pub struct Tag {
    tag_type: Type,
    tag_size: u32,
}

impl Tag {
    pub fn from_addr(addr : u64) -> Tag {

        Tag::End{tag_type : 0, tag_size : 4}
    }

    pub fn size(&self) -> u32 {

        self.tag_size
    }

}
