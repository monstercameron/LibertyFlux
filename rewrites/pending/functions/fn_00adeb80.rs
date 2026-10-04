// original: 0x00adeb80 ui_forward4_with_zero
/// Forward four values with a zero in the fourth slot to the worker.
export!(cdecl, rw_00adeb80(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, d) }
});
