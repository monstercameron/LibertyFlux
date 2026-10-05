// original: 0x00bf8ef0 task_blend_18

/// Blend one field toward a target by a factor and hand the result to the
/// field writer.
///
/// Original: 0x00bf8ef0 (thiscall, 3 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8ef0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd_f32(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        const C: u32 = 0x00EBC4E0;
        let a = rd_f32(a1.wrapping_add(0x18));
        let b = rd_f32(this.wrapping_add(0x18));
        let t = f32::from_bits(a2);
        let r = fadd(fmul(fsub(a, b), t), b);
        lf_checker_rt::callee_thiscall!(1, u32, a0, lf_checker_rt::relocated(C), r.to_bits());
        0
    }
});
