// original: 0x00adeb60 ui_forward3_with_two_zeros
/// Forward three values plus two zero defaults to the worker.
export!(cdecl, rw_00adeb60(a: u32, b: u32, c: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, 0) }
});
