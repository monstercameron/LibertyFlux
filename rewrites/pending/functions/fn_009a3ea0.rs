// original: 0x009a3ea0 audio_helper_test_nonzero
/// Original 0x009a3ea0 (unnamed): test whether a helper's answer is nonzero.
///
/// Calls the helper with `arg`; returns 1 when its answer is nonzero, else 0.
export!(stdcall, rw_009a3ea0(arg: u32) -> u32 {
    let r = callee_cdecl!(1, u32, arg);
    u32::from(r != 0)
});
