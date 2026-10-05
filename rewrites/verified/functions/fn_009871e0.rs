// original: 0x009871E0 audEmitter_transform_point (proposed)

/// Point transform of the emitter entity: applies a 3x4 matrix when one is
/// attached, else forwards to a fallback callee.
///
/// `this` points at the entity; `out` receives four floats and `inp` supplies
/// three. When the matrix pointer at `+MATRIX_OFF` of `this` is null, the
/// fallback callee (id 1, cdecl) is called as `fallback(out, this + 0x10, inp)`
/// and its answer ignored. Otherwise each output is one row of the matrix
/// (columns `M0_OFF`/`M1_OFF`/`M2_OFF` word triples, bias at `BIAS_OFF`)
/// dotted with the input in the original's operand order (pinned against
/// commuting), and the fourth output is the function's own uninitialised
/// stack slot, which the checker fills with a defined value.
/// Original: thiscall, two stack words, callee pops 8, no return value.
lf_checker_rt::export!(thiscall, rw_009871E0(this: u32, out: u32, inp: u32) -> u32 {
    const MATRIX_OFF: u32 = 0x20;
    const FALLBACK_THIS_OFF: u32 = 0x10;
    const FALLBACK: u32 = 1;
    const M0_OFF: u32 = 0x00;
    const M1_OFF: u32 = 0x10;
    const M2_OFF: u32 = 0x20;
    const BIAS_OFF: u32 = 0x30;

    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
    }
    #[inline(always)]
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
    }

    unsafe {
        let mat = ((this + MATRIX_OFF) as *const u32).read_unaligned();
        if mat == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                FALLBACK,
                u32,
                out,
                this.wrapping_add(FALLBACK_THIS_OFF),
                inp
            );
            return 0;
        }
        let i0 = rdf(inp);
        let i1 = rdf(inp + 4);
        let i2 = rdf(inp + 8);
        // Row 0: ((m10*i1 + m00*i0) + m20*i2) + b0.
        let t00 = mul(rdf(mat + M0_OFF), i0);
        let t10 = mul(rdf(mat + M1_OFF), i1);
        let t20 = mul(rdf(mat + M2_OFF), i2);
        let o0 = add(add(add(t10, t00), t20), rdf(mat + BIAS_OFF));
        // Row 1: ((m11*i1 + m01*i0) + m21*i2) + b1.
        let t01 = mul(rdf(mat + M0_OFF + 4), i0);
        let t11 = mul(rdf(mat + M1_OFF + 4), i1);
        let t21 = mul(rdf(mat + M2_OFF + 4), i2);
        let o1 = add(add(add(t11, t01), t21), rdf(mat + BIAS_OFF + 4));
        // Row 2: ((m12*i1 + m02*i0) + m22*i2) + b2.
        let t02 = mul(rdf(mat + M0_OFF + 8), i0);
        let t12 = mul(rdf(mat + M1_OFF + 8), i1);
        let t22 = mul(rdf(mat + M2_OFF + 8), i2);
        let o2 = add(add(add(t12, t02), t22), rdf(mat + BIAS_OFF + 8));
        wrf(out, o0);
        wrf(out + 4, o1);
        wrf(out + 8, o2);
        // The original copies its own uninitialised stack slot here; the
        // checker defines that slot's fill, zero below.
        wrf(out + 0xc, f32::from_bits(0));
    }
    0
});
