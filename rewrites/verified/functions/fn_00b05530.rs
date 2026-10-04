// original: 0x00b05530 forward_3_with_two_zeros

/// Forwards three arguments to the worker, padding with two zero words.
export!(cdecl, rw_00b05530(a: u32, b: u32, c: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, 0) }
});
