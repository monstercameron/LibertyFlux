// original: 0x0092CA40 matrix44_multiply_inplace (proposed)

/// Multiply two 4x4 row-major float matrices in place: `this = this * rhs`.
///
/// `this` points to the 64-byte accumulator (four rows of four floats),
/// `rhs` to the right-hand matrix laid out the same. Each accumulator row
/// `(x, y, z, w)` becomes `x*rhs.row0 + y*rhs.row1 + z*rhs.row2 + w*rhs.row3`
/// computed lane-wise as `((w*row3 + z*row2) + y*row1) + x*row0`, the
/// original's SSE order, pinned with `black_box`. Rows are updated one at a
/// time, so a row is fully read before it is overwritten.
///
/// Original: 0x0092CA40 (thiscall, one stack word). Leaf: no calls.
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_0092CA40(this: u32, rhs: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { (p as *mut u32).write_unaligned(v.to_bits()) }
        }
        for row in 0..4u32 {
            let b = this.wrapping_add(row * 16);
            let x = rdf(b);
            let y = rdf(b.wrapping_add(4));
            let z = rdf(b.wrapping_add(8));
            let w = rdf(b.wrapping_add(12));
            for lane in 0..4u32 {
                let a0 = rdf(rhs.wrapping_add(lane * 4));
                let a1 = rdf(rhs.wrapping_add(0x10 + lane * 4));
                let a2 = rdf(rhs.wrapping_add(0x20 + lane * 4));
                let a3 = rdf(rhs.wrapping_add(0x30 + lane * 4));
                let s = add(mul(w, a3), mul(z, a2));
                let s = add(s, mul(y, a1));
                let s = add(s, mul(x, a0));
                wrf(b.wrapping_add(lane * 4), s);
            }
        }
        this
    }
});
