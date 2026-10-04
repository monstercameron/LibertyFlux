// original: 0x008e6b50 forward3_with_two_zeros
/// Forward three words to the five-word range worker (cdecl/5, stubbed)
/// with two trailing zeros. Returns its answer.
export!(cdecl, rw_008e6b50(a: u32, b: u32, c: u32) -> u32 {
    callee_cdecl!(1, u32, a, b, c, 0, 0)
});

