// original: 0x00cb9250 vec3_normalize_return_length
/// Normalise a 3-vector in place and return its length (leaf).
///
/// Takes the vector at `a0` (cdecl, one stack argument; the incoming ecx
/// is ignored and overwritten with `a0`). Computes the squared length in
/// the order `x*x + y*y`, then `+ z*z`. When the squared length is an
/// ordered equal of zero the scale is `0.0`, so each component becomes
/// signed zero and the returned length is `+inf`; otherwise the scale is
/// `1 / sqrt(len2)`, each component is multiplied by it, and the returned
/// length is `1 / scale`. The length returns through the x87 register
/// (the original's `fld` of the slot it just wrote, which is its own
/// incoming argument slot). The float operation order is the original's.
/// No calls.
lf_checker_rt::export!(cdecl, rw_00cb9250(a0: u32) -> f64 {
    unsafe {
        /// Unit constant the scale derivation starts from (1.0).
        const ONE: f32 = 1.0;
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let x = rdf(a0);
        let y = rdf(a0 + 4);
        let z = rdf(a0 + 8);
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let scale = if len2 == 0.0 {
            0.0
        } else {
            div(ONE, core::hint::black_box(len2).sqrt())
        };
        let len = div(ONE, scale);
        wrf(a0, mul(x, scale));
        wrf(a0 + 4, mul(y, scale));
        wrf(a0 + 8, mul(z, scale));
        len as f64
    }
});
