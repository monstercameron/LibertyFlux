// original: 0x00dfe1a6 forward_5arg_zero4
// rs03f14: five-argument forward with a constant middle argument (cdecl/4).
//
// Passes its four arguments through to the worker with a constant zero
// inserted before the last one, returning the worker's answer.
export!(cdecl, rw_rs03f14(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, d) }
});
