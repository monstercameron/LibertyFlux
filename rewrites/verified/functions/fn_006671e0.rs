// original: 0x006671E0 create_rmptfx_keyframe

/// Allocates a 0x28-byte keyframe object through the thread-local allocator. If allocation succeeds, it calls the base initializer, installs the relocated vtable pointer, and clears four words at offsets 0x18, 0x1c, 0x20, and 0x24. A failed allocation returns null; otherwise the object pointer is returned.
lf_checker_rt::export!(cdecl, rw_006671e0() -> u32 {
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        0x28u32, 16u32, 0u32);
    if allocated != 0 {
        let object = allocated as *mut u32;
        let _base_result = lf_checker_rt::callee_thiscall!(2, u32, allocated);
        const VTABLE_FILE_VA: u32 = 0x00FE355C;
        unsafe { *object = lf_checker_rt::relocated(VTABLE_FILE_VA); }
        unsafe { *((allocated.wrapping_add(0x18)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x1C)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x20)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x24)) as *mut u32) = 0; }
    }
    allocated
});
