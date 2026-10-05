// original: 0x006964A0 rage::crAnimChannelRawVector3::sample_indexed

/// Samples a raw Vector3 channel at a fractional index into an output.
///
/// `this` is the channel object with the sample array at `[this+8]`; the
/// stack arguments are the integer `index`, the blend `t` as `f32` bits and
/// the output pointer. Each component is `lo + (hi - lo) * t` evaluated in
/// the original's order (`hi - lo`, times `t`, plus `lo`), where `lo` is
/// element `index` and `hi` element `index + 1` (16 bytes per element, the
/// second element read 16 bytes past the first). Bit-exact, NaN included.
/// Call-free; no return value.
///
/// Original: 0x006964A0 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_006964A0(this: u32, index: u32, tbits: u32, out: u32) -> u32 {
    unsafe {
        const SAMPLES_OFF: u32 = 8;
        const PAIR_STRIDE: u32 = 0x10;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let base = rd32(this + SAMPLES_OFF);
        let t = f32::from_bits(tbits);
        let row = base.wrapping_add(index.wrapping_mul(2).wrapping_mul(8));
        let mut c = 0u32;
        while c < 3 {
            let off = c * 4;
            let lo = f32::from_bits(rd32(row.wrapping_add(off)));
            let hi = f32::from_bits(rd32(row.wrapping_add(off).wrapping_add(PAIR_STRIDE)));
            let r = fadd(fmul(fsub(hi, lo), t), lo);
            unsafe { ((out + off) as *mut u32).write_unaligned(r.to_bits()) };
            c += 1;
        }
        0
    }
});
