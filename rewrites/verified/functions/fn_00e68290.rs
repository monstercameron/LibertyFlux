// original: 0x00e68290 veh_slot_bind_x4_8290

/// Initialise four consecutive vehicle objects, then register one callback.
///
/// Calls the tiny object initializer (a thiscall taking only the object
/// address) with four objects spaced `OBJ_STRIDE` bytes apart starting at
/// `OBJ0`, then registers the callback stub at `CALLBACK` with the registrar
/// (cdecl/1) and returns the registrar's answer. Both callees are intercepted
/// and answered by the checker.
///
/// Original: 0x00E68290 (cdecl/0, five direct calls across four loop iterations).
lf_checker_rt::export!(cdecl, rw_00e68290() -> u32 {
    unsafe {
        /// First of the four objects (file VA).
        const OBJ0: u32 = 0x012FA6F8;
        /// Spacing between the four objects.
        const OBJ_STRIDE: u32 = 0x10;
        /// How many objects are initialised.
        const OBJ_COUNT: u32 = 4;
        /// Callback stub registered afterwards (file VA).
        const CALLBACK: u32 = 0x00E72270;
        let base = lf_checker_rt::relocated(OBJ0);
        let mut i = 0u32;
        while i < OBJ_COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, base.wrapping_add(i.wrapping_mul(OBJ_STRIDE)));
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
