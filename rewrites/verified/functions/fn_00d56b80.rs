// original: 0x00d56b80 ccam_compute_blend

/// Blend the camera toward its target, capped by the global tick and `CAP`.
///
/// `this` points to the object and `a0`..`a2` are opaque words forwarded to
/// the final routine. The elapsed ticks since `STAMP` are divided by `BOUND`
/// (signed) to form a ratio, capped above by the constant `CAP`
/// (single-precision 1.0); when the elapsed count already reaches the bound
/// the cap is used directly. The pair routine fills two scratch words and
/// returns a float in ST0, which is stored to `OUT` and forwarded to the
/// attach routine; then the final routine runs with (scratch words, capped
/// ratio, `a0`, `a1`, `a2`). Returns the final routine's answer. The float
/// operations keep the original's operand order.
///
/// Original: 0x00d56b80 (thiscall, three stack arguments, four calls).
lf_checker_rt::export!(thiscall, rw_00d56b80(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Global tick the elapsed count is measured from.
        const TICK: u32 = 0x011735c4;
        /// Stamp the elapsed count is measured against.
        const STAMP: u32 = 0x28c;
        /// Bound that skips the ratio when reached.
        const BOUND: u32 = 0x284;
        /// Slot receiving the pair routine's result.
        const OUT: u32 = 0x228;
        /// Read-only cap for the ratio.
        const CAP: u32 = 0x00fe88e8;
        /// Shared setup (intercepted; thiscall, no arguments).
        const SETUP: u32 = 1;
        /// Pair routine (intercepted; thiscall, two scratch words, ST0 float).
        const PAIR: u32 = 2;
        /// Attach routine (intercepted; thiscall, one stack word).
        const ATTACH: u32 = 3;
        /// Final routine (intercepted; thiscall, six stack words).
        const FINAL: u32 = 4;
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let tick = (lf_checker_rt::global::<u32>(TICK) as *const u32).read();
        lf_checker_rt::callee_thiscall!(SETUP, u32, this);
        let dt = tick.wrapping_sub(((this + STAMP) as *const u32).read_unaligned());
        let bound = ((this + BOUND) as *const u32).read_unaligned();
        let cap = f32::from_bits((lf_checker_rt::global::<u32>(CAP) as *const u32).read_unaligned());
        let level = if (dt as i32) < (bound as i32) {
            let ratio = div(dt as i32 as f32, bound as i32 as f32);
            if cap > ratio { ratio } else { cap }
        } else {
            cap
        };
        let mut r0 = 0u32;
        let mut r1 = 0u32;
        let got: f32 = lf_checker_rt::callee_thiscall!(PAIR, f32, this,
            &mut r1 as *mut u32 as u32,
            &mut r0 as *mut u32 as u32);
        ((this + OUT) as *mut u32).write_unaligned(got.to_bits());
        lf_checker_rt::callee_thiscall!(ATTACH, u32, this, got.to_bits());
        lf_checker_rt::callee_thiscall!(FINAL, u32, this, r1, r0, level.to_bits(), a0, a1, a2)
    }
});
