// original: 0x00635fa0 streaming_resource_cleanup_variant_b
/// Releases the resource stored in this record. The manager handle comes from
/// TLS slot zero; a lookup sentinel clears the record, while a successful
/// lookup adjusts its resource pointer with wrapping arithmetic. The nested
/// resource is initialized with this specialization's dispatch table, its
/// pending count is updated through the manager helper, and the finalizer is
/// called with the manager and the live context pointer. The function is a
/// thiscall with no stack arguments and has no specified return value.
lf_checker_rt::export!(thiscall, rw_00635fa0(this: u32) -> u32 {
    const RECORD_RESOURCE_OFFSET: u32 = 0;
    const TLS_MANAGER_OFFSET: u32 = 4;
    const MANAGER_HANDLE_OFFSET: u32 = 0;
    const NESTED_COUNT_OFFSET: u32 = 0x10;
    const NESTED_FINALIZER_OFFSET: u32 = 0x18;
    const DISPATCH_TABLE_ADDRESS: u32 = 0x00fe25b4;
    const TLS_SLOT: usize = 0;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value); }
    }

    if this == 0 {
        return 0;
    }
    let resource = unsafe { read_u32(this.wrapping_add(RECORD_RESOURCE_OFFSET)) };
    let tls_root = lf_checker_rt::tls_slot(TLS_SLOT);
    let manager = unsafe { read_u32(tls_root.wrapping_add(TLS_MANAGER_OFFSET)) };
    if manager == 0 || resource == 0 {
        unsafe { write_u32(this.wrapping_add(RECORD_RESOURCE_OFFSET), 0); }
        return 0;
    }

    let manager_handle = unsafe { read_u32(manager.wrapping_add(MANAGER_HANDLE_OFFSET)) };
    let lookup = lf_checker_rt::callee_thiscall!(1, u32, manager_handle, this);
    if lookup == u32::MAX {
        unsafe { write_u32(this.wrapping_add(RECORD_RESOURCE_OFFSET), 0); }
        return 0;
    }

    let pointer_delta = lf_checker_rt::callee_thiscall!(2, u32, manager, resource);
    let nested_resource = resource.wrapping_add(pointer_delta);
    unsafe { write_u32(this.wrapping_add(RECORD_RESOURCE_OFFSET), nested_resource); }
    if nested_resource == 0 {
        unsafe { write_u32(this.wrapping_add(RECORD_RESOURCE_OFFSET), 0); }
        return 0;
    }

    // The original reloads this manager from TLS after the pointer helper.
    let tls_root = lf_checker_rt::tls_slot(TLS_SLOT);
    let manager = unsafe { read_u32(tls_root.wrapping_add(TLS_MANAGER_OFFSET)) };
    let _constructor = lf_checker_rt::callee_thiscall!(3, u32, nested_resource);
    unsafe { write_u32(nested_resource, lf_checker_rt::relocated(DISPATCH_TABLE_ADDRESS)); }

    let count_address = nested_resource.wrapping_add(NESTED_COUNT_OFFSET);
    let pending_count = unsafe { read_u32(count_address) };
    let live_context = if pending_count == 0 {
        nested_resource
    } else {
        let count_delta = lf_checker_rt::callee_thiscall!(2, u32, manager, pending_count);
        unsafe { write_u32(count_address, pending_count.wrapping_add(count_delta)); }
        manager
    };
    let _finalizer = lf_checker_rt::callee_thiscall!(
        4,
        u32,
        nested_resource.wrapping_add(NESTED_FINALIZER_OFFSET),
        manager,
        live_context,
    );
    0
});
