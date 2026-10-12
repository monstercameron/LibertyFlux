// original: 0x00667290 create_keyframe_transform_state

/// Allocates a 16-byte transform state through the thread-local allocator with 16-byte alignment and zero flags. When allocation succeeds, it clears the words at offsets eight and twelve, leaving the first eight bytes untouched; on failure it returns null. The allocation result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00667290() -> u32 {
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        0x10u32, 16u32, 0u32);
    if allocated != 0 {
        unsafe { *((allocated.wrapping_add(8)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(12)) as *mut u32) = 0; }
    }
    allocated
});
