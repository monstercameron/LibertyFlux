// original: 0x009A6CF0 audio_forward_default (proposed)

/// Forwards three arguments to the positional emitter with a zero fourth.
///
/// thiscall, three stack words: calls the emitter (callee 1, the function
/// at 0x009A6D10) on the same `this` with (`a0`, `a1`, `a2`, 0).
/// Returns the emitter's answer.
lf_checker_rt::export!(thiscall, rw_009a6cf0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const EMITTER: u32 = 1;
        lf_checker_rt::callee_thiscall!(EMITTER, u32, this, a0, a1, a2, 0)
    }
});
