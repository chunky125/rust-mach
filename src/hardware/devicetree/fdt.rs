use heapless::{String, Vec};

/// The FDT Header Structure
#[repr(C)]
#[derive (Copy,Clone)]
struct Header {
    magic: u32,
    total_size: u32,
    offset_dt_struct: u32,
    offset_dt_strings: u32,
    offset_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

const FDT_MAGIC_NUMBER: u32 = 0xd00dfeed;

/// Representation of the memory reservation entry
struct MemReserveEntry {
    addr: u64,
    size: u64,
}

/// Node type constants
const FDT_BEGIN_NODE: u32 = 0x1;
const FDT_END_NODE: u32 = 0x2;
const FDT_PROP_NODE: u32 = 0x3;
const FDT_NOP_NODE: u32 = 0x4;
const FDT_END_STRUCT: u32 = 0x9;

/// Property structure
struct Property {
    name: String<64>,
    value: Vec<u8, 128>,
}

pub struct FDT {
    header : Header,
}

impl FDT {

    /// Create a structure from the FDT location in memory
    pub unsafe fn from_fdt_addr(addr : u64) -> Option<FDT> {
        
        let new_fdt = FDT {
            header : *(addr as *const Header),
        };

        if new_fdt.header.magic != FDT_MAGIC_NUMBER {
            return None;
        }

        Some(new_fdt)

    }
}
