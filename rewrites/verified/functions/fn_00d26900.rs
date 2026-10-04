// original: 0x00d26900 ped_stat_to_timer (proposed)

/// Convert a callee-provided float into a millisecond timer value.
///
/// Calls the stat helper (intercepted) with `arg` to get `f`, then computes
/// `t = trunc((1.0 - f) * -3000.0)` with x86 convert-with-truncation
/// semantics (out of range or NaN yields `i32::MIN`), and forms
/// `-1500 - t`, clamped to 0 from below (the original's conditional move on
/// the subtraction's sign flag is exactly "0 when `t > -1500`"). When the
/// raw `f` is strictly below 0.1 (NaN skips: the original's jump-if-below-
/// or-equal is taken on unordered), a second intercepted query runs, and a
/// result whose low two bits are clear upgrades the timer to 3000.
///
/// Original: 0x00D26900 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00d26900(arg: u32) -> u32 {
    unsafe {
        const ONE_BITS: u32 = 0x3f80_0000; // 1.0f
        const GAIN_BITS: u32 = 0xc53b_8000; // -3000.0f
        const FLOOR_BITS: u32 = 0x3dcc_cccd; // 0.1f
        const BASE: i32 = -1500;
        const UPGRADE: u32 = 3000;
        #[inline(always)]
        fn cvttss2si(x: f32) -> i32 {
            // x86 convert-with-truncation: NaN and out-of-range yield MIN.
            if x.is_nan() || x >= 2_147_483_648.0 || x < -2_147_483_648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        let f: f32 = lf_checker_rt::callee_cdecl!(1, f32, arg);
        let shifted = core::hint::black_box(f32::from_bits(ONE_BITS)) - core::hint::black_box(f);
        let scaled = core::hint::black_box(shifted) * core::hint::black_box(f32::from_bits(GAIN_BITS));
        let t = cvttss2si(scaled);
        let mut timer = if t > BASE { 0u32 } else { BASE.wrapping_sub(t) as u32 };
        if f32::from_bits(FLOOR_BITS) > f {
            let q: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            if q & 3 == 0 {
                timer = UPGRADE;
            }
        }
        timer
    }
});
