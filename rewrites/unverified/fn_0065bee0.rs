// original: 0x0065bee0 rage::DayLighting::vf5
use lf_checker_rt::{export, relocated, tls_slot};

/// Allocate a record through the current thread's allocator vtable,
/// initialize its self-linked list head and constant fields, and return
/// the allocation pointer. The allocator is a thiscall target in slot 2.
export!(cdecl, rw_0065bee0() -> *mut u8 {
    const ALLOCATOR_SLOT: usize = 2;
    const ALLOCATION_SIZE: u32 = 0x14;
    const OBJECT_VTABLE_VA: u32 = 0x00FE3004;

    unsafe {
        let context = tls_slot(0) as *mut u8;
        let manager = context.add(8).cast::<*mut u8>().read_unaligned();
        let table = manager.cast::<*mut u32>().read_unaligned();
        let address = table.add(ALLOCATOR_SLOT).read_unaligned();
        let allocate: extern "thiscall" fn(*mut u8, u32, u32, u32) -> *mut u8 =
            core::mem::transmute(address as usize);
        let object = allocate(manager, ALLOCATION_SIZE, 0x10, 0);
        if object.is_null() {
            return object;
        }
        let head = object.add(8);
        head.cast::<u32>().write_unaligned(head as u32);
        head.add(4).cast::<u32>().write_unaligned(head as u32);
        object.cast::<u32>().write_unaligned(relocated(OBJECT_VTABLE_VA));
        object
    }
});
