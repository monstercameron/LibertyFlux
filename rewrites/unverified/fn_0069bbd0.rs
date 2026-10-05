// original: 0x0069BBD0 rage::crAnimChannelCurveFloat::eval_segment

/// Evaluates one curve segment polynomial at `t` into an output.
///
/// `coeff` points at the coefficient array, `order` (in `edx`) selects the
/// degree through a jump table, `t` arrives in `xmm0` and the stack argument
/// is the output pointer. Orders 0-3 evaluate Horner forms of that degree
/// (`c0`; `c0*t+c1`; `(c0*t+c1)*t+c2`; cubic); higher orders run a generic
/// `order`-step loop (`s = s*t + c[i]`). All arithmetic is `f32` in the
/// original's operand order. No return value.
///
/// Original: 0x0069BBD0 (fastcall: coeff in ecx, order in edx; one stack
/// word; `t` in xmm0).
lf_checker_rt::export!(fastcall, rw_0069BBD0(coeff: u32, order: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        let t = f32::from_bits(lf_checker_rt::xmm_word(0, 0));
        let c0 = f32::from_bits(rd32(coeff));
        if order == 0 {
            wr32(out, c0.to_bits());
            return 0;
        }
        if order == 1 {
            let r = fadd(fmul(c0, t), f32::from_bits(rd32(coeff + 4)));
            wr32(out, r.to_bits());
            return 0;
        }
        if order == 2 {
            let s1 = fadd(fmul(c0, t), f32::from_bits(rd32(coeff + 4)));
            let r = fadd(fmul(s1, t), f32::from_bits(rd32(coeff + 8)));
            wr32(out, r.to_bits());
            return 0;
        }
        if order == 3 {
            let s1 = fadd(fmul(c0, t), f32::from_bits(rd32(coeff + 4)));
            let s2 = fadd(fmul(s1, t), f32::from_bits(rd32(coeff + 8)));
            let r = fadd(fmul(s2, t), f32::from_bits(rd32(coeff + 12)));
            wr32(out, r.to_bits());
            return 0;
        }
        let mut s = c0;
        let mut left = order;
        let mut p = coeff.wrapping_add(4);
        while left != 0 {
            s = fmul(s, t);
            p = p.wrapping_add(4);
            s = fadd(s, f32::from_bits(rd32(p.wrapping_sub(4))));
            left -= 1;
        }
        wr32(out, s.to_bits());
        0
    }
});
