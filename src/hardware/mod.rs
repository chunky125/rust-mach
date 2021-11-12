mod devicetree;
mod efi;

use devicetree::DeviceTree;
use efi::EFISystemTable;

pub struct Hardware {
    efi_system_table: EFISystemTable,
    device_tree: DeviceTree,
}

pub unsafe fn create_from_efi_systab(efi_sys_table_addr: u64) -> Option<Hardware> {
    // Get an EFI Table
    let efi_sys_table = efi::from_addr(efi_sys_table_addr, 0);

    if efi_sys_table.is_some() {
        // Get the FDT from the system table, if it exists
        let init_device_tree = efi_sys_table.unwrap().contains_device_tree();

        if init_device_tree.is_some() {
            return Some(Hardware {
                efi_system_table: efi_sys_table.unwrap(),
                device_tree: init_device_tree.unwrap(),
            });
        }
    }

    None
}

impl Hardware {}
