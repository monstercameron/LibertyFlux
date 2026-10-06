// original: 0x0096e890 audio_matrix_transform_or_fallback (proposed)

/// Transform a 3-vector by the row matrix at +0x20, or run the fallback.
///
/// When the matrix pointer is null the fallback callee runs on (dst,
/// this+0x10, src). Otherwise dst[0..2] are the 4x3 row-major product in the
/// original's exact add/mul order (pinned), while dst[3] is whatever the
/// original's realigned scratch held: the proof runs with zeroed stack fill
/// and expects 0.0 there. Returns dst in EAX on both paths.
/// Original: 0x0096E890 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0096e890(this: u32, dst: u32, src: u32) -> u32 {
    unsafe {
        const MAT: u32 = 0x20;
        const FALLBACK: u32 = 1;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn lan(base: u32, off: u32) -> f32 {
            unsafe { ((base.wrapping_add(off)) as *const f32).read_unaligned() }
        }
        let m = ((this.wrapping_add(MAT)) as *const u32).read_unaligned();
        if m == 0 {
            lf_checker_rt::callee_cdecl!(FALLBACK, u32, dst, this.wrapping_add(0x10), src);
            return dst;
        }
        let s0 = lan(src, 0);
        let s1 = lan(src, 4);
        let s2 = lan(src, 8);
        let t0 = mul(lan(m, 0), s0);
        let t6 = mul(lan(m, 0x10), s1);
        let t2 = mul(lan(m, 0x14), s1);
        let a6 = add(t6, t0);
        let t1 = mul(lan(m, 0x18), s1);
        let a6 = add(a6, mul(lan(m, 0x20), s2));
        let a6 = add(a6, lan(m, 0x30));
        let a2 = add(t2, mul(lan(m, 4), s0));
        let a2 = add(a2, mul(lan(m, 0x24), s2));
        let a2 = add(a2, lan(m, 0x34));
        let a1 = add(t1, mul(lan(m, 8), s0));
        let a1 = add(a1, mul(lan(m, 0x28), s2));
        let a1 = add(a1, lan(m, 0x38));
        ((dst.wrapping_add(0)) as *mut f32).write_unaligned(a6);
        ((dst.wrapping_add(4)) as *mut f32).write_unaligned(a2);
        ((dst.wrapping_add(0xC)) as *mut f32).write_unaligned(0.0);
        ((dst.wrapping_add(8)) as *mut f32).write_unaligned(a1);
        dst
    }
});
