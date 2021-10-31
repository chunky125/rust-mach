
#[repr (C)]
struct EFISystemTable {
    signature : u64,
    revision : u64,
    headersize : u32,
    crc32 : u32,
    reserved : u32,
    fw_vendor : u64,
    fw_rev : u32,
    _pad1 : u32,
    con_in_handle : u64,
    con_in : u64,
    con_out_handle : u64,
    con_out : u64,
    stderr_handle : u64,
    stderr : u64,
    runtime : u64,
    boottime : u64,
    nr_tables : u32,
    _pad2 : u32,
    tables : *const EFIConfigTable,
}

struct EFIConfigTable {
    guid : EFIGuid,
    table : u64,
}

struct EFIGuid {
    a : u32,
    b : u16,
    c : u16,
    d0 : u8,
    d1 : u8,
    d2 : u8,
    d3 : u8,
    d4 : u8,
    d5 : u8,
    d6 : u8,
    d7 : u8,
}


impl EFISystemTable {



}
