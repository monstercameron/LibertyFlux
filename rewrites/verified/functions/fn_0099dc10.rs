// original: 0x0099dc10 audio_forward_zero4th
/// Forward four arguments with a zero fourth word to a five-argument helper.
///
/// Calls the callee (cdecl/5, stubbed by the checker) as (a, b, c, 0, d) and
/// returns its answer unchanged.
export!(cdecl, rw_0099dc10(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b, c, 0, d) }
});
