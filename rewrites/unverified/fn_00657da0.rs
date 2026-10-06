// original: 0x00657DA0 shaft_scaled_copy (proposed)

/// Copy four table words and four scaled measurements from a source record
/// into this object, asking the sampler before each measurement.
///
/// `this` points to the destination (8 words) and `src` to the source record.
/// Words at `src+0x2C0`, `+0x2C4`, `+0x2CC` and `+0x2C8` are copied to
/// `this+0x00`, `+0x04`, `+0x08` and `+0x0C` (note the third and fourth source
/// offsets are swapped). Then four lanes run: each calls the sampler
/// (callee 1, reached indirectly through the function pointer in the global
/// slot, no arguments) for an unsigned 32-bit sample, compares the reference
/// global against the sample for exact equality, and picks the lane's scale
/// integer from the primary global or, when equal, the alternate global
/// (lanes 0 and 2 share one pair, lanes 1 and 3 another). The scale, treated
/// as signed, is converted to float, multiplied by the source float at
/// `src+0x280+4*lane` in that order, truncated toward zero with `cvttss2si`
/// semantics (NaN, infinities and out-of-range magnitudes yield `i32::MIN`,
/// never saturation), converted back to float and stored at
/// `this+0x10+4*lane`. Returns the truncated integer of the last lane.
///
/// Original: 0x00657DA0 (thiscall, one stack word holding the source pointer).
lf_checker_rt::export!(thiscall, rw_00657DA0(this: u32, src: u32) -> u32 {
    unsafe {
        const SAMPLER_SLOT: u32 = 0x00E731AC;
        const REFERENCE: u32 = 0x0110DD14;
        const SCALE_A0: u32 = 0x0105C884;
        const SCALE_A1: u32 = 0x0105C880;
        const ALT_A0: u32 = 0x0105C888;
        const ALT_A1: u32 = 0x0105C87C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn g(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate `bits` (an f32) toward zero, matching `cvttss2si`: any
        /// NaN, infinity or magnitude at or beyond 2^31 yields `i32::MIN`.
        fn cvtt(mut bits: u32) -> i32 {
            bits = core::hint::black_box(bits);
            let f = f32::from_bits(bits);
            if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
                return i32::MIN;
            }
            core::hint::black_box(f) as i32
        }

        let ebx = this;
        let edi = src;
        wr32(ebx, rd32(edi + 0x2C0));
        wr32(ebx + 4, rd32(edi + 0x2C4));
        wr32(ebx + 8, rd32(edi + 0x2CC));
        wr32(ebx + 0x0C, rd32(edi + 0x2C8));
        let sampler: extern "cdecl" fn() -> u32 =
            unsafe { core::mem::transmute(g(SAMPLER_SLOT) as usize) };
        let sample0 = sampler();
        let mut scale = g(SCALE_A0);
        if g(REFERENCE) == sample0 {
            scale = g(ALT_A0);
        }
        let t0 = cvtt(mul(scale as i32 as f32, rdf(edi + 0x280)).to_bits());
        wrf(ebx + 0x10, t0 as f32);
        let sample1 = sampler();
        scale = g(SCALE_A1);
        if g(REFERENCE) == sample1 {
            scale = g(ALT_A1);
        }
        let t1 = cvtt(mul(scale as i32 as f32, rdf(edi + 0x284)).to_bits());
        wrf(ebx + 0x14, t1 as f32);
        let sample2 = sampler();
        scale = g(SCALE_A0);
        if g(REFERENCE) == sample2 {
            scale = g(ALT_A0);
        }
        let t2 = cvtt(mul(scale as i32 as f32, rdf(edi + 0x288)).to_bits());
        wrf(ebx + 0x18, t2 as f32);
        let sample3 = sampler();
        scale = g(SCALE_A1);
        if g(REFERENCE) == sample3 {
            scale = g(ALT_A1);
        }
        let t3 = cvtt(mul(scale as i32 as f32, rdf(edi + 0x28C)).to_bits());
        wrf(ebx + 0x1C, t3 as f32);
        t3 as u32
    }
});
