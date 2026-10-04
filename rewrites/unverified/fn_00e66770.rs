// original: 0x00E66770 init_array_50_stride40 (proposed)

/// Run the element constructor over every entry of one global array.
///
/// Calls the constructor (thiscall, element address in ecx, no stack
/// arguments) for 50 elements starting at the array base, stepping
/// 0x00000040 bytes (a down-counting loop). No arguments; returns the last
/// call's answer in eax (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66770() -> u32 {
    const BASE: u32 = 0x0128E960;
    const COUNT: u32 = 50;
    const STRIDE: u32 = 0x00000040;
    const CTOR_CALLEE: u32 = 1;
    let mut p = lf_checker_rt::relocated(BASE);
    let mut ans = 0u32;
    let mut i = COUNT;
    while i != 0 {
        ans = lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, p);
        p = p.wrapping_add(STRIDE);
        i -= 1;
    }
    ans
});
