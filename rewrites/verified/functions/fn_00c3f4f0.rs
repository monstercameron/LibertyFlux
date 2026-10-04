// original: 0x00c3f4f0 timing_channel_eval (proposed)

/// Evaluate one timing channel: blend ratio, fraction and shaped output.
///
/// `this` points to a channel with gains at `+0x00`/`+0x04`/`+0x08`, a bias
/// at `+0x0c`, a duration at `+0x14`, a start stamp at `+0x1c` and an active
/// flag byte at `+0x2c`. `out_ratio` and `out_fract` receive two outputs.
/// An inactive flag returns 0.0 at once.
///
/// Otherwise the elapsed ticks (a global clock minus the start stamp) over
/// the duration (1 when not positive) form the ratio, stored to `out_ratio`;
/// a negative duration, or an elapsed past the duration (which also clears
/// the flag), clamps it to 1.0. `out_fract` gets the elapsed minus its
/// hundredth, scaled by 0.001. Two evaluator callees run (the second through
/// a frame slot the test script fills), then a shaper callee, a math callee
/// over the gain chain, and a final shaper; the product of the two shaper
/// answers with the mixed value is returned.
///
/// Original: 0x00c3f4f0 (thiscall, two stack words). Returns the float in
/// `st0`.
lf_checker_rt::export!(thiscall, rw_00c3f4f0(this: u32, out_ratio: u32, out_fract: u32) -> f32 {
    unsafe { f4f0_core(this, out_ratio, out_fract, false) }
});

unsafe fn f4f0_core(this: u32, out_ratio: u32, out_fract: u32, keep_flag: bool) -> f32 {
    unsafe {
        const F_G0: u32 = 0x0;
        const F_G1: u32 = 0x4;
        const F_G2: u32 = 0x8;
        const F_BIAS: u32 = 0xc;
        const F_DUR: u32 = 0x14;
        const F_START: u32 = 0x1c;
        const F_FLAG: u32 = 0x2c;
        const K_MILLI: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const K_TWO: f32 = 2.0;
        const K_PI: f32 = f32::from_bits(0x4049_0fdb);
        const C_EVAL1: u32 = 1;
        const C_EVAL2: u32 = 2;
        const C_SHAPE1: u32 = 3;
        const C_MATH: u32 = 4;
        const C_SHAPE2: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if (this as *const u8).add(F_FLAG as usize).read() == 0 {
            return 0.0;
        }
        // The original keeps no frame of its own: it borrows the two
        // incoming argument slots as scratch (the second evaluator call
        // takes the address of the first slot; the shaper answers land in
        // the second). This rewrite uses its own locals for those words:
        // the values are identical and flow to the same outputs, but the
        // original's clobbering of its caller slots is not reproduced
        // (see the contract's narrowed field).
        let clock = lf_checker_rt::global::<i32>(0x11735c4).read();
        let elapsed = clock.wrapping_sub(rd32(this + F_START) as i32);
        let dur = rd32(this + F_DUR) as i32;
        let denom = if dur <= 0 { 1 } else { dur };
        wrf(out_ratio, div(elapsed as f32, denom as f32));
        let dur2 = rd32(this + F_DUR) as i32;
        if dur2 < 0 {
            wrf(out_ratio, 1.0);
        } else if elapsed > dur2 {
            if !keep_flag {
                (this as *mut u8).add(F_FLAG as usize).write(0);
            }
            wrf(out_ratio, 1.0);
        }
        // Truncated division by 1000 (verified against the original's
        // multiply-and-shift sequence over 200k values plus edges).
        let thou = elapsed / 1000;
        wrf(out_fract, mul(sub(elapsed as f32, thou as f32), K_MILLI));
        lf_checker_rt::callee_thiscall!(C_EVAL1, u32, this, out_ratio, 0);
        let mut slot = 0.0f32;
        lf_checker_rt::callee_thiscall!(
            C_EVAL2,
            u32,
            this,
            core::ptr::addr_of_mut!(slot) as u32,
            1
        );
        let f1: f32 = lf_checker_rt::callee_thiscall!(C_SHAPE1, f32, this);
        let x = add(
            mul(mul(mul(rdf(this + F_G2), rdf(out_fract)), K_TWO), K_PI),
            rdf(this + F_BIAS),
        );
        let w: u32 = lf_checker_rt::callee_cdecl!(C_MATH, u32, x.to_bits());
        let y = f32::from_bits(w);
        let t1 = mul(y, rdf(this + F_G1));
        let t2 = mul(rdf(this + F_G0), slot);
        let z = add(mul(t1, rdf(out_ratio)), t2);
        let mixed = mul(f1, z);
        let f2: f32 = lf_checker_rt::callee_thiscall!(C_SHAPE2, f32, this);
        mul(mixed, f2)
    }
}
