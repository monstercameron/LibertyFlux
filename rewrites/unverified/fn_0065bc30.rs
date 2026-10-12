// original: 0x0065bc30 rage::EnviromentDomeLighting::vf5
use lf_checker_rt::{export, relocated, tls_slot};

/// Allocate a twelve-byte environment dome record through TLS slot 0's
/// allocator vtable, store the relocated object vtable pointer in its first
/// word, and return the allocation. A null allocation returns unchanged.
export!(cdecl, rw_0065bc30() -> *mut u8 {
    const ALLOCATOR_TABLE_SLOT: usize = 2;
    const CONTEXT_MANAGER_OFFSET: usize = 0x08;
    const MANAGER_VTABLE_OFFSET: usize = 0x00;
    const OBJECT_VTABLE_OFFSET: usize = 0x00;
    const OBJECT_VTABLE_VA: u32 = 0x00FE_303C;
    const ALLOCATION_SIZE: u32 = 12;
    const ALLOCATION_ALIGNMENT: u32 = 16;
    const ALLOCATION_FLAGS: u32 = 0;

    unsafe {
        let thread_context = tls_slot(0) as *mut u8;
        let allocator_manager = thread_context
            .add(CONTEXT_MANAGER_OFFSET)
            .cast::<*mut u8>()
            .read_unaligned();
        let vtable = allocator_manager
            .add(MANAGER_VTABLE_OFFSET)
            .cast::<*mut u32>()
            .read_unaligned();
        let allocator_address = vtable.add(ALLOCATOR_TABLE_SLOT).read_unaligned();
        let allocate: extern "thiscall" fn(*mut u8, u32, u32, u32) -> *mut u8 =
            core::mem::transmute(allocator_address as usize);
        let object = allocate(
            allocator_manager,
            ALLOCATION_SIZE,
            ALLOCATION_ALIGNMENT,
            ALLOCATION_FLAGS,
        );
        if !object.is_null() {
            object
                .add(OBJECT_VTABLE_OFFSET)
                .cast::<u32>()
                .write_unaligned(relocated(OBJECT_VTABLE_VA));
        }
        object
    }
});
