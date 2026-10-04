// original: 0x009FF7A0 frag_vector_scale_submit (proposed)

/// Scale a three-float vector by the global factor and submit it.
///
/// Reads three floats at `v`, multiplies each by the global scale factor
/// (original operand order: value times factor), and passes a pointer to
/// the three results on the frame to the sink callee. Returns whatever the
/// callee returned.
///
/// Original: 0x009FF7A0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FF7A0(v: u32) -> u32 {
    unsafe {
        const SCALE: u32 = 0x00E99B0C;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let k = ((lf_checker_rt::relocated(SCALE)) as *const f32).read_unaligned();
        let x = ((v) as *const f32).read_unaligned();
        let y = ((v + 4) as *const f32).read_unaligned();
        let z = ((v + 8) as *const f32).read_unaligned();
        let out = [mul(x, k), mul(y, k), mul(z, k)];
        lf_checker_rt::callee_cdecl!(1, u32, out.as_ptr() as u32)
    }
});
