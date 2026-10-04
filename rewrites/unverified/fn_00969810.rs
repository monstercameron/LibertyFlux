// original: 0x00969810 radial_weight_2d (proposed)

/// Spread a 2D sample point over four weighted accumulators.
///
/// `this` is the owner object, `p` points to two input floats and `out`
/// receives one float. The inputs are centred on the object's reference
/// point (`+0x580/+0x584`), the offset is normalised to a unit direction
/// (a zero-length offset aims along +x instead), and each of four rounds
/// projects that direction onto a global weight pair, clamps negative
/// projections to zero, squares the result and scales one float from each
/// of two object tables (`+0x17d0` and `+0x1718` rows). The first table's
/// running total is stored to `out`; the second table's total is the
/// float result. Not-a-number inputs propagate to both outputs.
///
/// The normalisation test is the original's exact flag dance over
/// `len2 == 0.0` (a zero length keeps a zero scale; anything else divides
/// one by the square root), and the short-vector cutoff compares the
/// threshold first, so NaN lengths take the normalise path. Every
/// arithmetic operation keeps the original's operand order.
///
/// Original: 0x00969810 (thiscall: object in ECX, two stack words; float
/// result in ST0; no outgoing calls; reads eight global weight floats).
lf_checker_rt::export!(thiscall, rw_00969810(this: u32, p: u32, out: u32) -> f32 {
    unsafe {
        const CENTER_X: u32 = 0x580;
        const CENTER_Y: u32 = 0x584;
        const ROW_A: u32 = 0x17d0;
        const ROW_B: u32 = 0x1718;
        const SHORT_THRESH: f32 = 1e-12;
        const WEIGHTS: u32 = 0x0121_F5E0; // file VA of the 8 global weights

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
        #[inline(always)]
        unsafe fn rd(obj: u32, off: u32) -> f32 {
            unsafe { ((obj + off) as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn weight(i: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(WEIGHTS + i * 4).read() }
        }

        let d0 = sub(
            (p as *const f32).read_unaligned(),
            rd(this, CENTER_X),
        );
        let d1 = sub(
            ((p + 4) as *const f32).read_unaligned(),
            rd(this, CENTER_Y),
        );
        let len2 = add(mul(d1, d1), mul(d0, d0));
        let (dir_x, dir_y) = if SHORT_THRESH > len2 {
            (1.0f32, 0.0f32)
        } else {
            let scale = if len2 == 0.0 {
                0.0f32
            } else {
                let r = len2.sqrt();
                core::hint::black_box(1.0f32) / core::hint::black_box(r)
            };
            (mul(scale, d0), mul(d1, scale))
        };

        (out as *mut u32).write(0);
        let mut acc_a = 0.0f32;
        let mut acc_b = 0.0f32;
        let mut round = 0u32;
        while round < 4 {
            let base = round * 4;
            let mut t = add(mul(weight(base + 1), dir_y), mul(weight(base), dir_x));
            if t < 0.0 {
                t = 0.0;
            }
            let w = mul(t, t);
            // The original re-reads [out] for the running total each round.
            let store_a = add(mul(rd(this, ROW_A + round * 4), w), acc_a);
            let store_b = add(mul(rd(this, ROW_B + round * 4), w), acc_b);
            (out as *mut f32).write_unaligned(store_a);
            acc_a = (out as *const f32).read_unaligned();
            acc_b = store_b;
            round += 1;
        }
        acc_b
    }
});
