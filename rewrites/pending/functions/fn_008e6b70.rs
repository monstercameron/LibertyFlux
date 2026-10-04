// original: 0x008e6b70 forward4_with_zero_gap
/// Forward four words to the five-word range worker (cdecl/5, stubbed) as
/// `(a, b, c, 0, d)`. Returns its answer.
export!(cdecl, rw_008e6b70(a: u32, b: u32, c: u32, d: u32) -> u32 {
    callee_cdecl!(1, u32, a, b, c, 0, d)
});

