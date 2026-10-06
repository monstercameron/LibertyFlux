// original: 0x00de3e90 UIBasicClip::vf123

/// Probe the clip, then run the zero-argument action when the probe says so.
///
/// `this` is the clip object. The probe at table slot `PROBE (+0x140)` is
/// called with `this` in ECX; its low byte (AL) is tested against zero (an
/// equality test: no signedness involved). Only when it is nonzero, the
/// action at slot `ACTION (+0x13c)` is called with `this` in ECX and one
/// pushed zero word. The last answer produced is left in EAX as the result.
///
/// Original: thiscall, no stack arguments, up to two indirect calls, word
/// result in EAX.
lf_checker_rt::export!(thiscall, rw_00de3e90(this: u32) -> u32 {
    const PROBE: u32 = 0x140;
    const ACTION: u32 = 0x13c;
    unsafe {
        let table = (this as *const u32).read_unaligned();
        let probe = ((table + PROBE) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(probe as usize);
        let ans = f(this);
        if ans & 0xff != 0 {
            let table = (this as *const u32).read_unaligned();
            let action = ((table + ACTION) as *const u32).read_unaligned();
            let g: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(action as usize);
            g(this, 0)
        } else {
            ans
        }
    }
});
