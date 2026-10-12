// original: 0x00667230 create_ptx_keyframe4

/// Allocates a 0x40-byte ptx keyframe object through the thread-local allocator. If allocation succeeds, it calls the base initializer, installs the relocated vtable pointer, and clears six words at offsets 0x18 through 0x30 in four-byte steps. A failed allocation returns null; otherwise the object pointer is returned.
lf_checker_rt::export!(cdecl, rw_00667230() -> u32 {
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        0x40u32, 16u32, 0u32);
    if allocated != 0 {
        let object = allocated as *mut u32;
        let _base_result = lf_checker_rt::callee_thiscall!(2, u32, allocated);
        const VTABLE_FILE_VA: u32 = 0x00FE3554;
        unsafe { *object = lf_checker_rt::relocated(VTABLE_FILE_VA); }
        unsafe { *((allocated.wrapping_add(0x18)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x1C)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x20)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x24)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x28)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x2C)) as *mut u32) = 0; }
        unsafe { *((allocated.wrapping_add(0x30)) as *mut u32) = 0; }
    }
    allocated
});
