// original: 0x00969440 bilinear_sample_2d (proposed)

/// Bilinear-style sample over an integer lattice with fractional weights.
///
/// `this` is passed through untouched to the lattice helper, and `a` and
/// `b` are the sample coordinates. Coordinates at or beyond +-4 in either
/// axis, or NaN, sample as zero. Otherwise each axis splits into a
/// truncated integer cell and a `1 - fraction` weight; the four cells
/// around the point are read through the lattice helper (indices stride
/// by nine along the second axis from a base of forty, mirrored by each
/// axis's sign), blended pairwise with the first axis's weight, and the
/// two row blends are combined with the second axis's weight into the
/// float result.
///
/// The truncation is the original's exact chop-mode conversion (exact for
/// the in-range inputs), the integer-to-float trips go through the
/// original's double conversion including its unsigned adjustment, and
/// every arithmetic operation keeps the original's operand order.
///
/// Original: 0x00969440 (thiscall: untouched object in ECX, two float
/// stack words; float result in ST0; four lattice calls plus three
/// blend calls; no globals).
lf_checker_rt::export!(thiscall, rw_00969440(this: u32, a: f32, b: f32) -> f32 {
    unsafe {
        const RANGE: f32 = 4.0;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const LATTICE_BASE: i32 = 0x28;
        const LATTICE_STRIDE: i32 = 9;
        const LATTICE_CALLEE: u32 = 1;
        const BLEND_CALLEE: u32 = 2;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let abs_a = f32::from_bits(a.to_bits() & ABS_MASK);
        if !(abs_a < RANGE) {
            return 0.0;
        }
        let abs_b = f32::from_bits(b.to_bits() & ABS_MASK);
        if !(abs_b < RANGE) {
            return 0.0;
        }
        // Chop-mode truncation of a value in [0, 4): 0..3, exact.
        let t1 = abs_a as i32;
        let t2 = abs_b as i32;
        // The original's int-to-float trip: signed double conversion
        // plus 2^32 when the input is negative (dead here: cells are
        // never negative), narrowed back to float.
        let ft1 = ((t1 as f64) + (if t1 < 0 { 4294967296.0 } else { 0.0 })) as f32;
        let ft2 = ((t2 as f64) + (if t2 < 0 { 4294967296.0 } else { 0.0 })) as f32;
        let f1 = sub(1.0, sub(abs_a, ft1));
        let f2 = sub(1.0, sub(abs_b, ft2));
        let sgn1 = if a > 0.0 { 1i32 } else { -1 };
        let sgn2 = if b < 0.0 { 1i32 } else { -1 };
        let i1 = sgn1 * t1;
        let row0 = LATTICE_STRIDE * (sgn2 * t2);
        let row1 = LATTICE_STRIDE * (sgn2 * (t2 + 1));
        let col1 = sgn1 * (t1 + 1);

        let a1: f32 = lf_checker_rt::callee_thiscall!(
            LATTICE_CALLEE,
            f32,
            this,
            (i1 + LATTICE_BASE + row0) as u32
        );
        let a2: f32 = lf_checker_rt::callee_thiscall!(
            LATTICE_CALLEE,
            f32,
            this,
            (col1 + LATTICE_BASE + row0) as u32
        );
        let a3: f32 = lf_checker_rt::callee_thiscall!(
            LATTICE_CALLEE,
            f32,
            this,
            (i1 + LATTICE_BASE + row1) as u32
        );
        let a4: f32 = lf_checker_rt::callee_thiscall!(
            LATTICE_CALLEE,
            f32,
            this,
            (col1 + LATTICE_BASE + row1) as u32
        );
        let b1: f32 = lf_checker_rt::callee_cdecl!(
            BLEND_CALLEE,
            f32,
            a2.to_bits(),
            a1.to_bits(),
            0u32,
            1.0f32.to_bits(),
            f1.to_bits()
        );
        let b2: f32 = lf_checker_rt::callee_cdecl!(
            BLEND_CALLEE,
            f32,
            a4.to_bits(),
            a3.to_bits(),
            0u32,
            1.0f32.to_bits(),
            f1.to_bits()
        );
        let b3: f32 = lf_checker_rt::callee_cdecl!(
            BLEND_CALLEE,
            f32,
            b2.to_bits(),
            b1.to_bits(),
            0u32,
            1.0f32.to_bits(),
            f2.to_bits()
        );
        b3
    }
});
