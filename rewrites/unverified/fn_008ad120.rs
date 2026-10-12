// original: 0x008ad120 audResponseCurve_eval_a

/// Response-curve evaluation through two scripted exponential calls.
///
/// Loads the rate at `this + 4` and the mode at `this + 8`. Forms the first
/// exponent as the negated rate and the second as the negated product of the
/// rate with the incoming factor, widened to doubles, each passed with the
/// image's double base to the scripted helper (low lanes; each call's upper
/// lane carries entry leftovers the helper never reads). When the mode is
/// strictly below zero or unordered, returns the blend of the two narrowed
/// answers `(p2 - p1) / (1 - p1)`; otherwise returns the second answer.
/// 32-bit thiscall of one stack argument, float result. The original spills
/// over its incoming argument word, which a rewrite cannot address, so the
/// proof runs with the stack check off.
lf_checker_rt::export!(thiscall, rw_008ad120(this: u32, factor: u32) -> f32 {
    unsafe {
        const RATE_OFF: u32 = 4;
        const MODE_OFF: u32 = 8;
        const NEG_ONE_VA: u32 = 0x00FE8D94;
        const BASE_VA: u32 = 0x00E7C9E8;
        const ONE_VA: u32 = 0x00FE88E8;

        let rate = ((this + RATE_OFF) as *const f32).read_unaligned();
        let mode = ((this + MODE_OFF) as *const f32).read_unaligned();
        let arg = f32::from_bits(factor);
        let neg_one = lf_checker_rt::global::<f32>(NEG_ONE_VA).read_unaligned();
        let base_bits = lf_checker_rt::global::<u64>(BASE_VA).read_unaligned();
        let f1 = core::hint::black_box(rate) * core::hint::black_box(neg_one);
        let t0 = core::hint::black_box(rate) * core::hint::black_box(arg);
        let f1b = (f1 as f64).to_bits();
        let ans1 = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            (base_bits & 0xFFFF_FFFF) as u32,
            (base_bits >> 32) as u32,
            (f1b & 0xFFFF_FFFF) as u32,
            (f1b >> 32) as u32
        );
        let p1 = core::hint::black_box(f64::from_bits(ans1)) as f32;
        let e2 = core::hint::black_box(t0) * core::hint::black_box(neg_one);
        let e2b = (e2 as f64).to_bits();
        let ans2 = lf_checker_rt::callee_cdecl!(
            2,
            u64,
            (base_bits & 0xFFFF_FFFF) as u32,
            (base_bits >> 32) as u32,
            (e2b & 0xFFFF_FFFF) as u32,
            (e2b >> 32) as u32
        );
        let p2 = core::hint::black_box(f64::from_bits(ans2)) as f32;
        if !(mode >= 0.0) {
            let one = lf_checker_rt::global::<f32>(ONE_VA).read_unaligned();
            let num = core::hint::black_box(p2) - core::hint::black_box(p1);
            let den = core::hint::black_box(one) - core::hint::black_box(p1);
            core::hint::black_box(num) / core::hint::black_box(den)
        } else {
            p2
        }
    }
});
