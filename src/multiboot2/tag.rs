mod efi;
mod string;

//
// Generic Multiboot2 Tag
//
#[repr(C)]
pub struct Tag {
    pub tag_type: u32,
    pub tag_size: u32,
}

impl Tag {
    pub fn new(addr: u64) -> Tag {
        let new_type: *const u32 = addr as *const u32;
        let new_size: *const u32 = (addr + 4) as *const u32;

        unsafe {
            Tag {
                tag_type: *new_type,
                tag_size: *new_size,
            }
        }
    }
}

//
// Enum for the tag type
//
enum Type {
    End = 0,
    CmdLine = 1,
    BootLoaderName = 2,
    Module = 3,
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
    EFI64IH = 20,
    LoadBaseAddress = 21,
    ELF64Sections = 22,
}
