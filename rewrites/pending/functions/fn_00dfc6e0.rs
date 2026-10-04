// original: 0x00dfc6e0 vsnprintf
/// `vsnprintf` front end: forward with a null locale argument.
export!(cdecl, rw_00dfc6e0(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, d) }
});
