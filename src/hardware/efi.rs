#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFITableHeader {
    signature: u64,
    revision: u32,
    headersize: u32,
    crc32: u32,
    reserved: u32,
}

const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFISystemTable {
    header: EFITableHeader,
    fw_vendor: u64,
    fw_rev: u32,
    _pad1: u32,
    con_in_handle: u64,
    con_in: u64,
    con_out_handle: u64,
    con_out: u64,
    stderr_handle: u64,
    stderr: u64,
    runtime: u64,
    boottime: u64,
    nr_tables: u32,
    _pad2: u32,
    tables: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
struct EFIConfigTable {
    guid: EFIGuid,
    table_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
struct EFIGuid {
    b: [u8; 16],
}

/// Macro to simplify generation of GUIDS for EFI
macro_rules! efi_guid {
    ($a:expr; $b:expr; $c:expr;$d0:expr;$d1:expr;$d2:expr;$d3:expr;$d4:expr;
     $d5:expr;$d6:expr;$d7:expr) => {{
        EFIGuid {
            b: [
                ($a & 0xff) as u8,
                (($a >> 8) & 0xff) as u8,
                (($a >> 16) & 0xff) as u8,
                (($a >> 24) & 0xff) as u8,
                ($b & 0xff) as u8,
                (($b >> 8) & 0xff) as u8,
                ($c & 0xff) as u8,
                (($c >> 8) & 0xff) as u8,
                $d0,
                $d1,
                $d2,
                $d3,
                $d4,
                $d5,
                $d6,
                $d7,
            ],
        }
    }};
}

const EFI_GUID_DEVICE_TREE: EFIGuid =
    efi_guid!(0xb1b621d5 as u32;0xf19c;0x41a5;0x83;0x0b;0xd9;0x15;0x2c;0x69;0xaa;0xe0);

use super::devicetree::DeviceTree;

/// Create an EFI system table structure from the memory address
pub unsafe fn from_addr(addr: u64, min_major_version: u16) -> Option<EFISystemTable> {
    let sys_tab = addr as *const EFISystemTable;

    if (*sys_tab).header.signature != EFI_SYSTEM_TABLE_SIGNATURE {
        return None;
    }

    if (((*sys_tab).header.revision >> 16) as u16) < min_major_version {
        return None;
    }

    Some(*sys_tab)
}

impl EFISystemTable {
    /// See if our EFI config tables include a FDT pointer
    pub unsafe fn contains_device_tree(&self) -> Option<DeviceTree> {
        for i in 0..self.nr_tables as isize {
            let config_table = self.tables as *const EFIConfigTable;

            if (*config_table.offset(i)).guid == EFI_GUID_DEVICE_TREE {
                let dev_tree = super::devicetree::DeviceTree::from_fdt_addr((*config_table).table_addr);

                return dev_tree;
            }
        }

        None
    }
}
