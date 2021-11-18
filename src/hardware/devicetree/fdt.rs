use heapless::{String, Vec};
use super::DeviceTreeNode;

/// Macro to convert u32 value from FDT big endian to little endian

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
    base_addr : u64,
    header : Header,
    curr_node : u64,
}

impl FDT {

    /// Create a structure from the FDT location in memory
    pub unsafe fn from_fdt_addr(addr : u64) -> Option<FDT> {
        
        let new_fdt = FDT {
            base_addr : addr,
            header : *(addr as *const Header),
            curr_node : 0,
        };

        if new_fdt.header.magic != FDT_MAGIC_NUMBER {
            return None;
        }

        if new_fdt.header.version < 10 {
            return None;
        }

        Some(new_fdt)

    }

  
    pub unsafe fn get_next_node(&mut self) -> Option<DeviceTreeNode> {

        if self.curr_node == 0 {
            
            // Go to the start
            self.curr_node = self.base_addr + self.header.offset_dt_struct as u64;

            // Get Node Number (Big Endian!!!)
            let node_type = *(self.curr_node as (*const u32));

        } else {
            // Start from point
            
        }

        None

    }
}
