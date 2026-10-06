// original: 0x00bf6b30 lerp_forward_18
/// Blend one field toward a peer value and forward it to the tagged helper.
///
/// `this` and the peer object `src` both carry the field at `+0x18`. `t` is
/// the blend factor. Computes `(src - base) * t + base` in that SSE order
/// (subss, mulss, addss) and calls the helper as thiscall with ECX = `chan`,
/// the blended float and the tag address `TAG`. Returns the helper's answer.
/// No comparison is made; NaN inputs propagate through the SSE op order.
///
/// Original: 0x00BF6B30 (thiscall, three stack words: chan, src, t).
lf_checker_rt::export!(thiscall, rw_00bf6b30(this: u32, chan: u32, src: u32, t: f32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
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
        const VAL_OFF: u32 = 0x18;
        const TAG: u32 = 0x00ebbe0c;
        let base = rdf(this + VAL_OFF);
        let far = rdf(src + VAL_OFF);
        let v = fadd(fmul(fsub(far, base), t), base);
        lf_checker_rt::callee_thiscall!(1, u32, chan, lf_checker_rt::relocated(TAG),
            v.to_bits())
    }
});
