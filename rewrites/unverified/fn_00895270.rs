// original: 0x00895270 audio_set_effect_float_3 (proposed)

/// Set effect slot 3's float and remember it on the voice.
///
/// Looks up the effect parameter object for slot 3 from `this + 0xac`
/// (patched callee); when non-null, writes the float bits `f` through it
/// with index 0 (patched callee). Always stores `f` at `this + 0x538`.
/// Returns the writer's result, or 0 when the lookup found nothing.
///
/// Original: 0x00895270 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00895270(this: u32, f: u32) -> u32 {
    unsafe {
        const PARAM_LOOKUP: u32 = 1;
        const SLOT_WRITER: u32 = 2;
        const PARAMS_OFF: u32 = 0xac;
        const SLOT_INDEX: u32 = 3;
        const CACHE_OFF: u32 = 0x538;

        let obj = lf_checker_rt::callee_thiscall!(
            PARAM_LOOKUP,
            u32,
            this.wrapping_add(PARAMS_OFF),
            SLOT_INDEX
        );
        let ret = if obj != 0 {
            lf_checker_rt::callee_thiscall!(SLOT_WRITER, u32, obj, f, 0)
        } else {
            0
        };
        ((this + CACHE_OFF) as *mut u32).write_unaligned(f);
        ret
    }
});
