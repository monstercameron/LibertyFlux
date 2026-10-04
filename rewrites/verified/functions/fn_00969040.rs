// original: 0x00969040 time_pair_to_grid_slot (proposed)

/// Map a pair of time values to one combined grid slot, returned as a float.
///
/// `pair` points to two consecutive floats. Each is scaled (`v * SCALE +
/// BIAS`, taking it from roughly [-3000, 3000] into [0, LIMIT]), rounded
/// down to a whole number with the original's round-magic sequence, and
/// clamped into [0, LIMIT]. The result is `slot_b * LIMIT + slot_a`, so the
/// pair reads as one two-dimensional cell index. Not-a-number inputs flow
/// through the same operations and stay not-a-number.
///
/// The rounding is the original's exact operation sequence, not a library
/// call: add and subtract a sign-matched round magic (2^23 when the
/// magnitude is below it, else signed zero), then subtract one when the
/// rounded value moved strictly up. The comparisons are ordered (false for
/// NaN), matching the original's conditional jumps, and every arithmetic
/// operation keeps the original's operand order.
///
/// Original: 0x00969040 (stdcall, one stack word holding the pair pointer;
/// float result in ST0; no outgoing calls; reads only read-only constants).
lf_checker_rt::export!(stdcall, rw_00969040(pair: u32) -> f32 {
    unsafe {
        const SCALE: f32 = 0.02; // bits 0x3CA3D70A, measured from the image
        const BIAS: f32 = 60.0;
        const LIMIT: f32 = 120.0;
        const ROUND_MAGIC: f32 = 8_388_608.0; // 2^23
        const SIGN_MASK: u32 = 0x8000_0000;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        /// Scale one input and round it down to a whole number.
        #[inline(always)]
        fn slot(v: f32) -> f32 {
            let t = add(mul(v, SCALE), BIAS);
            let sign = t.to_bits() & SIGN_MASK;
            let mag = f32::from_bits(t.to_bits() ^ sign);
            let m = if mag < ROUND_MAGIC { ROUND_MAGIC } else { 0.0 };
            let magic = f32::from_bits(m.to_bits() | sign);
            let q = sub(add(t, magic), magic);
            let frac = sub(q, t);
            // Original uses "not less-or-equal" against signed zero: subtract
            // one only when the rounded value moved strictly up.
            let down = if frac > 0.0 { 1.0f32 } else { 0.0f32 };
            sub(q, down)
        }

        /// Clamp into [0, LIMIT]; NaN passes through unchanged.
        #[inline(always)]
        fn clamp(x: f32) -> f32 {
            let mut x = x;
            if x < 0.0 {
                x = 0.0;
            }
            if x > LIMIT {
                x = LIMIT;
            }
            x
        }

        let a = (pair as *const f32).read_unaligned();
        let b = ((pair + 4) as *const f32).read_unaligned();
        add(mul(clamp(slot(b)), LIMIT), clamp(slot(a)))
    }
});
