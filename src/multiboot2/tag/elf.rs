//
// Module for implementing Multiboot2 information decoding and storage
//
// (c) Chris Plant 2021
//

/// Enum for the ELF Section Type
/// Forced to u32 so we can use enum directly
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum SectionType {
    Unused = 0,
    ProgramData = 1,
    SymbolTable = 2,
    StringTable = 3,
    RelocationAddEnds = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    ProgramNoData = 8,
    RelocationNoAddEnds = 9,
    Reserved = 10,
    DynamicSymbolTable = 11,
}

/// Structure for Elf Section Table Entry
/// Mirrors ELF section headers to simplify creation
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ELFSectionEntry {
    section_name_addr: u32,
    pub section_type: SectionType,
    section_flags: u64,
    pub section_addr: u64,
    pub section_size: u64,
    section_link: u32,
    section_info: u32,
    section_addr_align: u64,
    section_entsize: u64,
    _pad: u64, // Drive overall size to 0x40
}

impl ELFSectionEntry {}
