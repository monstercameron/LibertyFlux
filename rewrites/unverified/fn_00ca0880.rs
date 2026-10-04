// original: 0x00ca0880 face_param_reset

/// Reset a face-animation parameter row from two global defaults.
///
/// Zeroes the words at `+0xe4` and `+0xec` and loads `+0xe0` / `+0xe8` from
/// two global float slots. The single stack word is ignored. No value is
/// returned.
///
/// Original: 0x00ca0880 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca0880(this: u32, _ignored: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const DST_A: u32 = 0xe0;
        const ZERO_A: u32 = 0xe4;
        const DST_B: u32 = 0xe8;
        const ZERO_B: u32 = 0xec;
        wr32(this + ZERO_A, 0);
        wr32(this + DST_A, *lf_checker_rt::global::<u32>(0x0105_0b28));
        wr32(this + ZERO_B, 0);
        wr32(this + DST_B, *lf_checker_rt::global::<u32>(0x0105_0b34));
        0
    }
});
