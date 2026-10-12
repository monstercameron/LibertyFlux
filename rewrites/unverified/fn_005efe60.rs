// original: 0x005EFE60 params_init_call (proposed)

/// Initialise through an eight-argument helper, then install the vtable.
///
/// Calls the helper with `this` and the constant arguments
/// (0, 3, 4.0, 0x19c, table, 0, 1.0, 0) in order, writes the vtable pointer
/// and returns `this`. The table argument is an image address carried
/// through the worker's relocation. The original is thiscall with no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_005efe60(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB7374;
        const TABLE: u32 = 0x00FE15DC;
        const HELPER_CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(
            HELPER_CALLEE, u32, this, 0, 3, 0x40800000u32, 0x19c,
            lf_checker_rt::relocated(TABLE), 0, 0x3f800000u32, 0
        );
        (this as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
