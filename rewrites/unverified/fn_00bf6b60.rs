// original: 0x00bf6b60 lerp_forward_18_1c
/// Blend two fields toward peer values and forward each to the tagged helper.
///
/// Two lanes of the same blend as `lerp_forward_18`: the field at `+0x18`
/// with tag `TAG0`, then the field at `+0x1c` with tag `TAG1`, each computed
/// as `(src - base) * t + base` in SSE order. Both calls are thiscall with
/// ECX = `chan`. Returns the second call's answer; the first is discarded.
///
/// Original: 0x00BF6B60 (thiscall, three stack words: chan, src, t).
lf_checker_rt::export!(thiscall, rw_00bf6b60(this: u32, chan: u32, src: u32, t: f32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const VAL0: u32 = 0x18;
        const VAL1: u32 = 0x1c;
        const TAG0: u32 = 0x00ebbdb0;
        const TAG1: u32 = 0x00ebbdb8;
        let v0 = fadd(fmul(fsub(rdf(src + VAL0), rdf(this + VAL0)), t),
            rdf(this + VAL0));
        lf_checker_rt::callee_thiscall!(1, u32, chan, v0.to_bits(),
            lf_checker_rt::relocated(TAG0));
        let v1 = fadd(fmul(fsub(rdf(src + VAL1), rdf(this + VAL1)), t),
            rdf(this + VAL1));
        lf_checker_rt::callee_thiscall!(2, u32, chan, v1.to_bits(),
            lf_checker_rt::relocated(TAG1))
    }
});
