// original: 0x009a47d0 audio_forward_to_helper
/// Original 0x009a47d0 (unnamed): forward one argument to a helper.
///
/// Tail-forwards `arg` to the callee and returns its answer.
export!(stdcall, rw_009a47d0(arg: u32) -> u32 {
    callee_cdecl!(1, u32, arg)
});
