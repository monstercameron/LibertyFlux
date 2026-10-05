// original: 0x009AAB40 audio_enabled_gate (proposed)

/// Audio enable gate: returns 1 only when the global kill switch is clear
/// and the global ready flag is set.
///
/// Reads two global bytes: `AUDIO_DISABLED` (returns 0 at once when nonzero)
/// and `AUDIO_READY` (result is whether it is nonzero). The stack argument
/// is ignored (the callee still pops it: stdcall/1). Returns `al`.
lf_checker_rt::export!(stdcall, rw_009AAB40(_unused: u32) -> u8 {
    unsafe {
        const AUDIO_DISABLED: u32 = 0x0128437d;
        const AUDIO_READY: u32 = 0x012fb3b3;
        if (lf_checker_rt::global::<u8>(AUDIO_DISABLED)).read() != 0 {
            return 0;
        }
        ((lf_checker_rt::global::<u8>(AUDIO_READY)).read() != 0) as u8
    }
});
