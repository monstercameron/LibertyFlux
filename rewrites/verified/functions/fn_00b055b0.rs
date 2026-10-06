// original: 0x00b055b0 sort_span_capped
/// Sort an 8-byte-record span with a recursion cap.
///
/// cdecl `(first, last, aux)`: when `first == last` returns at once (the
/// original passes the incoming return register through, which the
/// contract does not compare). Otherwise `count = (last - first) >> 3`
/// (SIGNED shift) must be at least 1 -- the original halves it down to 1
/// to get `floor(log2(count))` and never terminates for `count < 1`, so
/// the contract only feeds `count >= 1` -- then calls the introsort
/// driver (helper 1, cdecl `(first, last, 0, 2*depth, aux)`) and the
/// insertion sort (helper 2, cdecl `(first, last, aux)`), returning the
/// latter's answer.
export!(cdecl, rw_00b055b0(a0: u32, a1: u32, a2: u32) -> u32 {
    if a0 == a1 {
        return 0; // unchecked passthrough, see doc comment
    }
    let count = ((a1.wrapping_sub(a0)) as i32) >> 3;
    // 31 - leading_zeros == floor(log2(count)) for count >= 1, which is
    // what the original's halving loop computes.
    let depth = 31 - (count as u32).leading_zeros();
    let _: u32 = callee_cdecl!(1, u32, a0, a1, 0, depth.wrapping_mul(2), a2);
    let r: u32 = callee_cdecl!(2, u32, a0, a1, a2);
    r
});
