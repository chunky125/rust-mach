mod fdt;

use heapless::String;

pub struct DeviceTree {
}

pub enum DeviceTreeValue {
}

pub struct DeviceTreeProperty {
    name : String<64>,
    value : DeviceTreeValue,
}

pub struct DeviceTreeNode {
    name : String<64>,
    has_value : bool,
    has_properties : bool,
}

impl DeviceTree {

    pub fn from_fdt_addr( addr : u64 ) -> Option<DeviceTree> {

        use fdt::FDT;

        unsafe {
            let fdt = FDT::from_fdt_addr(addr);
        }

        None
    }
}
