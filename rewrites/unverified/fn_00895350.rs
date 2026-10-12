// original: 0x00895350 audio_shutdown_chain (proposed)

/// Run the audio teardown sequence, then tail into the final release.
///
/// Calls the teardown-and-wait step (patched callee) with the same `this`,
/// then tail-calls the final release (patched tail callee). The value ecx
/// holds at the tail is whatever the first callee's stub left, not something
/// this function sets, so the tail call is declared without a register
/// argument. Returns the tail callee's result.
///
/// Original: 0x00895350 (thiscall: `this` in ecx, no stack words; ends in a
/// tail jump with the stack at entry level).
lf_checker_rt::export!(thiscall, rw_00895350(this: u32) -> u32 {
    unsafe {
        const TEARDOWN: u32 = 1;
        const FINAL_RELEASE: u32 = 2;

        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        lf_checker_rt::callee_cdecl!(FINAL_RELEASE, u32)
    }
});
