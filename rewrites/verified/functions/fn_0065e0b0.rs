// original: 0x0065e0b0 create_skyhat_perlin_noise
use lf_checker_rt::{export, relocated, tls_slot};

/// Allocate a 0x30-byte record through TLS slot 0's allocator vtable.
/// The allocator receives the byte size, 16-byte alignment, and a zero flag.
/// A null allocation is returned unchanged. Otherwise, the list links at
/// +0x08 and +0x0C point to the head at +0x08; the vtable and fields at
/// +0x10, +0x14, +0x18, +0x1C, +0x24, +0x28, +0x2C are initialized before the allocation pointer is returned.
/// The external entry is a zero-argument cdecl function.
export!(cdecl, rw_0065e0b0() -> *mut u8 {
    const ALLOCATOR_SLOT: usize = 2;
    const CONTEXT_MANAGER_OFFSET: usize = 0x08;
    const MANAGER_VTABLE_OFFSET: usize = 0x00;
    const LIST_HEAD_OFFSET: usize = 0x08;
    const NEXT_LINK_OFFSET: usize = 0x00;
    const PREVIOUS_LINK_OFFSET: usize = 0x04;
    const OBJECT_VTABLE_OFFSET: usize = 0x00;
    const ALLOCATION_SIZE: u32 = 0x30;
    const ALLOCATION_ALIGNMENT: u32 = 0x10;
    const ALLOCATION_FLAGS: u32 = 0;
    const OBJECT_VTABLE_VA: u32 = 0x00FE2404;
    const FIELD_10_OFFSET: usize = 0x10;
    const FIELD_10_VALUE: u32 = 0x00000000;
    const FIELD_14_OFFSET: usize = 0x14;
    const FIELD_14_VALUE: u32 = 0x00000000;
    const FIELD_18_OFFSET: usize = 0x18;
    const FIELD_18_VALUE: u32 = 0x3DCCCCCD;
    const FIELD_1C_OFFSET: usize = 0x1C;
    const FIELD_1C_VALUE: u32 = 0x41400000;
    const FIELD_24_OFFSET: usize = 0x24;
    const FIELD_24_VALUE: u32 = 0x00000000;
    const FIELD_28_OFFSET: usize = 0x28;
    const FIELD_28_VALUE: u32 = 0x00000000;
    const FIELD_2C_OFFSET: usize = 0x2C;
    const FIELD_2C_VALUE: u8 = 0x01;

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
        let head = object.add(LIST_HEAD_OFFSET);
        head.add(NEXT_LINK_OFFSET).cast::<u32>().write_unaligned(head as u32);
        head.add(PREVIOUS_LINK_OFFSET).cast::<u32>().write_unaligned(head as u32);
        object.add(OBJECT_VTABLE_OFFSET).cast::<u32>().write_unaligned(relocated(OBJECT_VTABLE_VA));
        object.add(FIELD_10_OFFSET).cast::<u32>().write_unaligned(FIELD_10_VALUE);
        object.add(FIELD_14_OFFSET).cast::<u32>().write_unaligned(FIELD_14_VALUE);
        object.add(FIELD_18_OFFSET).cast::<u32>().write_unaligned(FIELD_18_VALUE);
        object.add(FIELD_1C_OFFSET).cast::<u32>().write_unaligned(FIELD_1C_VALUE);
        object.add(FIELD_24_OFFSET).cast::<u32>().write_unaligned(FIELD_24_VALUE);
        object.add(FIELD_28_OFFSET).cast::<u32>().write_unaligned(FIELD_28_VALUE);
        object.add(FIELD_2C_OFFSET).write(FIELD_2C_VALUE);
        object
    }
});
