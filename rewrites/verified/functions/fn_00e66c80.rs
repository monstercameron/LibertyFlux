// original: 0x00E66C80 init_obj_array_00

/// Run one constructor-like callee over a fixed array of 64 objects.
///
/// Calls the callee `COUNT` times with the object pointer in ECX, starting at
/// `BASE` and advancing by `STEP` bytes each iteration, and returns the last
/// call's answer. The callee takes no stack arguments. All object addresses
/// are derived from the relocated image base.
///
/// Original: 0x00E66C80 (cdecl, no arguments, 64 outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e66c80() -> u32 {
    const BASE: u32 = 0x012B5890;
    const COUNT: u32 = 64;
    const STEP: u32 = 0x20;
    let mut last: u32 = 0;
    let mut obj = lf_checker_rt::relocated(BASE);
    let mut k: u32 = 0;
    while k < COUNT {
        last = lf_checker_rt::callee_thiscall!(1, u32, obj);
        obj = obj.wrapping_add(STEP);
        k += 1;
    }
    last
});
