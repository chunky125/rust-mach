/// Basic memory allocator for the kernel, uses a reserved area (in linker) before we get memory
/// information from grub/EFI


use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::ptr;

enum AllocatorState {
    NotInit,
    EarlyAlloc,
    FullAlloc,
}


struct Allocator  {
    state : UnsafeCell<AllocatorState>,
    early_alloc_start : UnsafeCell<usize>,
    early_alloc_head : UnsafeCell<usize>,
    early_alloc_end : UnsafeCell<usize>,
}


/// The allocator for Mach, needs quite a bit of work
#[global_allocator]
static ALLOCATOR : Allocator = Allocator {
    state : UnsafeCell::new(AllocatorState::NotInit),
    early_alloc_start : UnsafeCell::new(0x0),
    early_alloc_head : UnsafeCell::new(0x0),
    early_alloc_end : UnsafeCell::new(0x0),
};


unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for Allocator {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let state = self.state.get();

        match *state {

            AllocatorState::NotInit => {
                ptr::null_mut()
            }

            AllocatorState::EarlyAlloc => {
                // This is a bump allocator copied from embedded rust book
                // needs alot of work but should help me getting moving!
                let head_value = self.early_alloc_head.get();
                let size = layout.size();
                let align = layout.align();
                let align_mask = !(align - 1);

                let start = (*head_value + align - 1) & align_mask;

                if start + size > *self.early_alloc_end.get() {
                    ptr::null_mut() // No space left
                } else {
                    *head_value = start + size;
                    start as *mut u8
                }
            }

            AllocatorState::FullAlloc {} => {
                ptr::null_mut() // Not implemented yet
            }
        }

    }

    unsafe fn dealloc(&self, _: *mut u8, _ : Layout) {

    }

}

pub unsafe fn init_early_alloc() -> bool {

    let state = ALLOCATOR.state.get();

    match &*state {

        AllocatorState::NotInit => {

            extern "C" {
               static _heap_start : u8;
               static _heap_end : u8;
            }

            let heap_start_addr = &_heap_start as *const u8 as usize;
            let heap_end_addr = &_heap_end as *const u8 as usize;

            if heap_start_addr < heap_end_addr {
                let head = ALLOCATOR.early_alloc_head.get();
                let start = ALLOCATOR.early_alloc_start.get();
                let end = ALLOCATOR.early_alloc_start.get();

                *head = heap_start_addr;
                *start = heap_start_addr;
                *end = heap_end_addr;

                *state = AllocatorState::EarlyAlloc;

                true

            } else {
                false
            }
        },

        other => false
    }

}


#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
