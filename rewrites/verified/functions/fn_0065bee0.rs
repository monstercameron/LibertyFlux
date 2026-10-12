// original: 0x0065bee0 rage::DayLighting::vf5
use lf_checker_rt::{export, relocated, tls_slot};

/// Allocate a 0x14-byte record through TLS slot 0's allocator vtable.
/// The allocator receives the byte size, 16-byte alignment, and a zero flag.
/// A null allocation is returned unchanged. Otherwise, only its vtable
/// word is initialized before the allocation pointer is returned.
/// The external entry is a zero-argument cdecl function.
export!(cdecl, rw_0065bee0() -> *mut u8 {
    const ALLOCATOR_SLOT: usize = 2;
    const CONTEXT_MANAGER_OFFSET: usize = 0x08;
    const MANAGER_VTABLE_OFFSET: usize = 0x00;
    const LIST_HEAD_OFFSET: usize = 0x08;
    const NEXT_LINK_OFFSET: usize = 0x00;
    const PREVIOUS_LINK_OFFSET: usize = 0x04;
    const OBJECT_VTABLE_OFFSET: usize = 0x00;
    const ALLOCATION_SIZE: u32 = 0x14;
    const ALLOCATION_ALIGNMENT: u32 = 0x10;
    const ALLOCATION_FLAGS: u32 = 0;
    const OBJECT_VTABLE_VA: u32 = 0x00FE3004;

    unsafe {
        let context = tls_slot(0) as *mut u8;
        let manager = context.add(CONTEXT_MANAGER_OFFSET).cast::<*mut u8>().read_unaligned();
        let table = manager.add(MANAGER_VTABLE_OFFSET).cast::<*mut u32>().read_unaligned();
        let address = table.add(ALLOCATOR_SLOT).read_unaligned();
        let allocate: extern "thiscall" fn(*mut u8, u32, u32, u32) -> *mut u8 =
            core::mem::transmute(address as usize);
        let object = allocate(manager, ALLOCATION_SIZE, ALLOCATION_ALIGNMENT, ALLOCATION_FLAGS);
        if object.is_null() {
            return object;
        }
        object.add(OBJECT_VTABLE_OFFSET).cast::<u32>().write_unaligned(relocated(OBJECT_VTABLE_VA));
        object
    }
});
