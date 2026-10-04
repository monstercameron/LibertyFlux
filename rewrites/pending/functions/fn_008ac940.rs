// original: 0x008ac940 rage::audDelayEffect::vf1
/// audDelayEffect::vf1: run shared setup, then mark the delay line clean.
///
/// Forwards `this` and both arguments to the shared initializer (thiscall/2,
/// stubbed by the checker). When the helper reports failure (low byte zero)
/// its answer is returned untouched; otherwise the dirty flag at `this+0x150`
/// is cleared and success is reported in the low byte, preserving the
/// helper's upper bytes exactly as the original's byte-wide store does.
export!(thiscall, rw_008ac940(this_: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        let init: extern "thiscall" fn(*mut u8, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let answer = init(this_, a, b);
        if (answer & 0xFF) == 0 {
            answer
        } else {
            *this_.add(0x150) = 0;
            (answer & 0xFFFF_FF00) | 1
        }
    }
});
