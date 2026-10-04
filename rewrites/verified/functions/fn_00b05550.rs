// original: 0x00b05550 forward_4_with_zero_gap

/// Forwards four arguments to the worker with a zero word before the last.
export!(cdecl, rw_00b05550(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, d) }
});
