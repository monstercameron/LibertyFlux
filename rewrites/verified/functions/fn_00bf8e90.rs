// original: 0x00bf8e90 task_blend_1c_18

/// Blend two fields toward a target by a factor and hand each result to the
/// field writer: out = (target - base) * t + base, in that operation order.
///
/// Original: 0x00bf8e90 (thiscall, 3 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8e90(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
        const C1: u32 = 0x00EBC520;
        const C2: u32 = 0x00EBC528;
        let a = rd_f32(a1.wrapping_add(0x1c));
        let b = rd_f32(this.wrapping_add(0x1c));
        let t = f32::from_bits(a2);
        let r0 = fadd(fmul(fsub(a, b), t), b);
        lf_checker_rt::callee_thiscall!(1, u32, a0, lf_checker_rt::relocated(C1), r0.to_bits());
        let a = rd_f32(a1.wrapping_add(0x18));
        let b = rd_f32(this.wrapping_add(0x18));
        let r1 = fadd(fmul(fsub(a, b), t), b);
        lf_checker_rt::callee_thiscall!(2, u32, a0, lf_checker_rt::relocated(C2), r1.to_bits());
        0
    }
});
