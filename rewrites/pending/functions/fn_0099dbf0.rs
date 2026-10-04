// original: 0x0099dbf0 audio_forward_pad2
/// Forward three arguments plus two zero words to a five-argument helper.
///
/// Calls the callee (cdecl/5, stubbed by the checker) and returns its answer
/// unchanged.
export!(cdecl, rw_0099dbf0(a: u32, b: u32, c: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, 0) }
});
