// original: 0x00E68BC0 veh_slot_array_init (proposed)
/// Initialise an array of vehicle slots through the slot callee.
///
/// Calls the slot callee (`CALLEE`, thiscall, object pointer in `ecx`, no
/// stack arguments) `COUNT` times with the addresses `BASE + i * STRIDE`.
/// The countdown runs `COUNT - 1` down to `-1`, so exactly `COUNT` calls
/// fire. No memory is touched directly.
///
/// Original: 0x00E68BC0 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68bc0() -> u32 {
    unsafe {
        const BASE: u32 = 0x015DBE10;
        const COUNT: u32 = 150;
        const STRIDE: u32 = 0x40;
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
