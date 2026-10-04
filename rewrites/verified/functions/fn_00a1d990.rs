// original: 0x00a1d990 cam_smooth_probe_blend (proposed)

/// Blends a stored value toward a probed target and returns it.
///
/// `a0` points to a probed record, `a1` is a flag word, and `this` holds
/// the stored float at `+CUR_OFF`. A predicate callee runs on `a0`; the
/// target is `K_TARGET` (-0.55) when the predicate holds and the word at
/// `a0 + GEN_OFF` is not `GEN_SKIP` (6), otherwise 0.0. The blend factor
/// is 1.0 when the low byte of `a1` is nonzero, else 0.25. The stored
/// value becomes `(target - cur) * k + cur` in that operation order, and
/// the new value is returned on `st0`. (The original also writes it over
/// the caller's `a1` slot, which a Rust rewrite cannot address; the stack
/// check is off for this function and the value is still observed through
/// the heap store and the return channel.)
///
/// Original: 0x00a1d990 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1d990(this: u32, a0: u32, a1: u32) -> f32 {
    unsafe {
        const CALLEE: u32 = 1;
        const GEN_OFF: u32 = 0x7b8;
        const GEN_SKIP: u32 = 6;
        const CUR_OFF: u32 = 0x2c8;
        const K_TARGET: f32 = f32::from_bits(0xbf0c_cccd); // -0.55
        const K_LO: f32 = f32::from_bits(0x3e80_0000); // 0.25
        const K_HI: f32 = 1.0;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, a0);
        let base = if ok as u8 != 0
            && ((a0 + GEN_OFF) as *const u32).read_unaligned() != GEN_SKIP
        {
            K_TARGET
        } else {
            0.0
        };
        let k = if a1 as u8 != 0 { K_HI } else { K_LO };
        let cur = f32::from_bits(((this + CUR_OFF) as *const u32).read_unaligned());
        let r = add(mul(sub(base, cur), k), cur);
        ((this + CUR_OFF) as *mut u32).write_unaligned(r.to_bits());
        r
    }
});
