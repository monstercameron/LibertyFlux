// original: 0x00E69240 veh_slot_array_init (proposed)
/// Initialise an array of vehicle slots through the slot callee.
///
/// Calls the slot callee (`CALLEE`, thiscall, object pointer in `ecx`, no
/// stack arguments) `COUNT` times with the addresses `BASE + i * STRIDE`.
/// The countdown runs `COUNT - 1` down to `-1`, so exactly `COUNT` calls
/// fire. No memory is touched directly.
///
/// Original: 0x00E69240 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e69240() -> u32 {
    unsafe {
        const BASE: u32 = 0x015FFC40;
        const COUNT: u32 = 40;
        const STRIDE: u32 = 0x20;
        const CALLEE: u32 = 1;
        let mut p = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            lf_checker_rt::callee_thiscall!(CALLEE, u32, p);
            p = p.wrapping_add(STRIDE);
            i += 1;
        }
        0
    }
});
