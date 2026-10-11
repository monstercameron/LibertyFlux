// original: 0x00b041a0 pack_floats_to_words
/// Scale four floats into six packed 16-bit words.
///
/// thiscall `(this, f0, f1, f2, f3)`: the first three arguments are each
/// multiplied by the global scale (4.0) and truncated toward zero; the
/// low 16 bits are taken SIGNED and clamped below at 1, stored at
/// `+8`/`+0xa`/`+0x10`. The vector length `sqrt(f0*f0+f1*f1+f2*f2)`
/// (products then adds in argument order) scaled the same way lands at
/// `+6` with the same clamp. Then `f3` is passed (in the vector register)
/// to two helpers in turn; each answer is multiplied by the global
/// factor (16384.0) and truncated with x86 convert semantics (NaN,
/// infinities and out-of-range values yield 0x80000000, whose low word
/// is 0), stored raw at `+0xe` and `+0xc`. Returns the full 32-bit second
/// conversion.
export!(thiscall, rw_00b041a0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    const SCALE: u32 = 0x00FE_8AB8;
    const FACTOR: u32 = 0x00EA_4D40;
    const F32_MAX_PLUS1: f32 = 2147483648.0; // 2^31, exact
    #[inline(always)]
    fn cvtt_full(x: f32) -> i32 {
        // x86 cvttss2si: NaN, +-Inf and out-of-range yield 0x80000000.
        if x.is_nan() || x >= F32_MAX_PLUS1 || x < -F32_MAX_PLUS1 {
            0x8000_0000u32 as i32
        } else {
            x as i32
        }
    }
    #[inline(always)]
    fn clamp1_from_product(p: f32) -> u16 {
        // Low 16 bits of the conversion, SIGNED, clamped below at 1. A
        // plain `as` cast feeds the identical low 16 bits on this path:
        // NaN gives 0 and both overflows give low words (0 / -1) that the
        // clamp maps to 1, exactly like the indefinite value's low word 0.
        let s = (p as i32) as i16;
        (if s >= 1 { s } else { 1 }) as u16
    }
    unsafe {
        let c = f32::from_bits(*global::<u32>(SCALE));
        let k = f32::from_bits(*global::<u32>(FACTOR));
        let f0 = f32::from_bits(a0);
        let f1 = f32::from_bits(a1);
        let f2 = f32::from_bits(a2);
        let p0 = core::hint::black_box(f0) * core::hint::black_box(c);
        ((this + 8) as *mut u16).write_unaligned(clamp1_from_product(p0));
        let p1 = core::hint::black_box(f1) * core::hint::black_box(c);
        ((this + 0xA) as *mut u16).write_unaligned(clamp1_from_product(p1));
        let p2 = core::hint::black_box(f2) * core::hint::black_box(c);
        ((this + 0x10) as *mut u16).write_unaligned(clamp1_from_product(p2));
        let s0 = core::hint::black_box(f0) * core::hint::black_box(f0);
        let s1 = core::hint::black_box(f1) * core::hint::black_box(f1);
        let s2 = core::hint::black_box(f2) * core::hint::black_box(f2);
        let a = core::hint::black_box(s0) + core::hint::black_box(s1);
        let b = core::hint::black_box(a) + core::hint::black_box(s2);
        let r = b.sqrt();
        let q = core::hint::black_box(r) * core::hint::black_box(c);
        ((this + 6) as *mut u16).write_unaligned(clamp1_from_product(q));
        let ans1: u32 = callee_cdecl!(1, u32, a3);
        let g1 = core::hint::black_box(f32::from_bits(ans1)) * core::hint::black_box(k);
        ((this + 0xE) as *mut u16).write_unaligned(cvtt_full(g1) as u16);
        let ans2: u32 = callee_cdecl!(2, u32, a3);
        let g2 = core::hint::black_box(f32::from_bits(ans2)) * core::hint::black_box(k);
        let full = cvtt_full(g2);
        ((this + 0xC) as *mut u16).write_unaligned(full as u16);
        full as u32
    }
});
