// original: 0x00dfc548 vswprintf_p_l
/// Positional wide `vswprintf` front end with a fixed output callback.
///
/// Forwards the five caller arguments plus the shared callback address to
/// the worker, then clamps any negative result to -1.
export!(cdecl, rw_00dfc548(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    unsafe {
        let r = callee_cdecl!(1, u32, relocated(0xE06D3D), a, b, c, d, e);
        if (r as i32) < 0 {
            0xFFFFFFFF
        } else {
            r
        }
    }
});
