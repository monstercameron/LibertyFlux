// original: 0x00a51830 vehicle_tune_gears (proposed)

/// Tune up to four gears whose thresholds exceed the measured ratio.
///
/// Runs the tune callee once with the base pair, then divides the measured
/// value (+0x50) by its scale (+0x4c) and compares each of four thresholds
/// (+0x70 step 4) against the ratio: a strictly greater threshold triggers
/// another tune call with that gear's pair (an equal or unordered one
/// skips). All calls share the constant object in ecx. Thiscall, no stack
/// words, one callee at five sites, no result.
lf_checker_rt::export!(thiscall, rw_00a51830(this: u32) -> u32 {
    unsafe {
        const TUNE: u32 = 1;
        const TUNE_OBJ: u32 = 0x171f9b4;
        const NUM: u32 = 0x50;
        const DEN: u32 = 0x4c;
        const THR: u32 = 0x70;
        const PAIRS: [(u32, u32); 4] = [(0x4c1, 0x4c2), (0x4c8, 0x4c9), (0x1a2, 0x1a3), (0x1a7, 0x1a8)];
        const BASE: (u32, u32) = (0x36a1, 0x4b3);
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        lf_checker_rt::callee_thiscall!(TUNE, u32, TUNE_OBJ, this, 0, BASE.1, BASE.0);
        let ratio = div(rdf(this.wrapping_add(NUM)), rdf(this.wrapping_add(DEN)));
        let mut k: u32 = 0;
        while k < 4 {
            let t = rdf(this.wrapping_add(THR).wrapping_add(k.wrapping_mul(4)));
            if t > ratio {
                let (a3, a2) = PAIRS[k as usize];
                lf_checker_rt::callee_thiscall!(TUNE, u32, TUNE_OBJ, this, k.wrapping_add(1), a2, a3);
            }
            k = k.wrapping_add(1);
        }
        0
    }
});
