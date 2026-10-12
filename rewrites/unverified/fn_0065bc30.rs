// original: 0x0065bc30 rage::EnviromentDomeLighting::vf5
use lf_checker_rt::{export, tls_slot};

/// Allocate a twelve-byte environment dome record through the current thread's
/// allocator vtable, tag a successful allocation, and return it. The allocator
/// receives size, alignment, and zero-fill arguments in that stack order.
export!(cdecl, rw_0065bc30() -> *mut u8 {
    const ALLOCATOR_TABLE_SLOT: usize = 2;
    const OBJECT_TAG: u32 = 0x00FE_303C;
    const ALLOCATION_SIZE: u32 = 12;
    const ALLOCATION_ALIGNMENT: u32 = 16;

    unsafe {
        let thread_context = tls_slot(0) as *mut u8;
        let allocator_manager = thread_context.add(8).cast::<*mut u8>().read_unaligned();
        let vtable = allocator_manager.cast::<*mut u32>().read_unaligned();
        let allocator_address = vtable.add(ALLOCATOR_TABLE_SLOT).read_unaligned();
        let allocate: extern "thiscall" fn(*mut u8, u32, u32, u32) -> *mut u8 =
            core::mem::transmute(allocator_address as usize);
        let object = allocate(
            allocator_manager,
            ALLOCATION_SIZE,
            ALLOCATION_ALIGNMENT,
            0,
        );
        if !object.is_null() {
            object.cast::<u32>().write_unaligned(OBJECT_TAG);
        }
        object
    }
});
