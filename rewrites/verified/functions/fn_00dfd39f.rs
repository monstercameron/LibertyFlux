// original: 0x00dfd39f forward_4arg_zero3
// rs03f03: three-argument forward to the shared core helper (cdecl/3).
//
// Passes its three arguments through to the helper with a constant zero
// inserted before the last one, returning the helper's answer.
export!(cdecl, rw_rs03f03(a: u32, b: u32, c: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, 0, c) }
});
