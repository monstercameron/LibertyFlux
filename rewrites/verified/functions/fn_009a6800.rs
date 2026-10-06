// original: 0x009A6800 audio_forward_level_default (proposed)

/// Forwards the default level (0.3) and the level global to the voice setup.
///
/// No arguments. Calls the setup routine (callee 1, the function at
/// 0x009A6A10, thiscall on the voice at `VOICE`) with the current value of
/// the level global at `LEVEL` and the constant `DEFAULT_LEVEL`
/// (0.3f, bits 0x3E99999A). Returns the setup's answer.
lf_checker_rt::export!(cdecl, rw_009a6800() -> u32 {
    unsafe {
        const SETUP: u32 = 1;
        const VOICE: u32 = 0x001284A60;
        const LEVEL: u32 = 0x001288550;
        const DEFAULT_LEVEL: u32 = 0x3E99999A;
        let voice = lf_checker_rt::relocated(VOICE);
        let level = lf_checker_rt::global::<u32>(LEVEL).read_unaligned();
        lf_checker_rt::callee_thiscall!(SETUP, u32, voice, level, DEFAULT_LEVEL)
    }
});
