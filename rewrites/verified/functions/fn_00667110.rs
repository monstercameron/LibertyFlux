// original: 0x00667110 create_keyframe_index_entry

/// Allocates one 12-byte keyframe index entry through the allocator stored in thread-local state. A null allocator result is returned unchanged; otherwise the three words are initialized to zero, zero, and the all-bits-set sentinel. The allocator call uses a 16-byte alignment and zero flags.
lf_checker_rt::export!(cdecl, rw_00667110() -> u32 {
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        0xCu32, 16u32, 0u32);
    if allocated != 0 {
        unsafe { *((allocated) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(4)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(8)) as *mut u32) = 0xFFFFFFFF; }
    }
    allocated
});
