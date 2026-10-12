// original: 0x006671A0 create_keyframe_auxiliary_block

/// Allocates an 8-byte auxiliary block through the thread-local allocator with 16-byte alignment and zero flags. When allocation succeeds, it clears both words; on failure it returns null without writing. The allocation result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_006671a0() -> u32 {
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        0x8u32, 16u32, 0u32);
    if allocated != 0 {
        unsafe { *((allocated) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(4)) as *mut u32) = 0; }
    }
    allocated
});
