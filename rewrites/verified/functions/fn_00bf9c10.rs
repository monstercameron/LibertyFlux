// original: 0x00bf9c10 task_blend_vec3_to_190

/// Blend the +0x18 field toward the target and report it to the field
/// writer, expand two packed vectors through scripted scratch buffers, and
/// blend them lane-wise into the output at +0x190. Each lane adds back
/// its own lane of the second buffer.
///
/// Original: 0x00bf9c10 (thiscall, 3 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9c10(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn r8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn w8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn r32u(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32u(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
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
        const C: u32 = 0x00EBC70C;
        let t = f32::from_bits(a2);
        let lerp0 = fadd(fmul(fsub(rd_f32(a1.wrapping_add(0x18)), rd_f32(this.wrapping_add(0x18))), t),
                         rd_f32(this.wrapping_add(0x18)));
        lf_checker_rt::callee_thiscall!(1, u32, a0, lf_checker_rt::relocated(C), lerp0.to_bits());
        let mut buf_b = [0u32; 3];
        let mut buf_a = [0u32; 3];
        lf_checker_rt::callee_thiscall!(2, u32, this, buf_b.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(3, u32, a1, buf_a.as_mut_ptr() as u32);
        let a0f = |i: usize| f32::from_bits(buf_a[i]);
        let b0f = |i: usize| f32::from_bits(buf_b[i]);
        let r0 = fadd(fmul(fsub(a0f(0), b0f(0)), t), b0f(0));
        let r1 = fadd(fmul(fsub(a0f(1), b0f(1)), t), b0f(1));
        let r2 = fadd(fmul(fsub(a0f(2), b0f(2)), t), b0f(2));
        w32u(a0.wrapping_add(0x190), r0.to_bits());
        w32u(a0.wrapping_add(0x194), r1.to_bits());
        w32u(a0.wrapping_add(0x198), r2.to_bits());
        0
    }
});
