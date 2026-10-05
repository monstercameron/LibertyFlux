// original: 0x00bf8f20 task_blend_18_ret_a0

/// Blend one field toward a target, hand the result to the field writer, and
/// return the blend as a float (it is reloaded from the incoming a1 argument
/// slot, which the blend overwrote). That slot is dead after the callee-pop
/// return and is not addressable from Rust, so the stack check is off here.
///
/// Original: 0x00bf8f20 (thiscall, 3 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8f20(this: u32, a0: u32, a1: u32, a2: u32) -> f32 {
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
        const C: u32 = 0x00EBC4C0;
        let a = rd_f32(a1.wrapping_add(0x18));
        let b = rd_f32(this.wrapping_add(0x18));
        let t = f32::from_bits(a2);
        let r = fadd(fmul(fsub(a, b), t), b);
        lf_checker_rt::callee_thiscall!(1, u32, a0, lf_checker_rt::relocated(C), r.to_bits());
        r
    }
});
