// original: 0x00a917e0 stream_float_classify

/// Classifies a pair of scaled float sums against two bounds.
///
/// `this` holds floats at `+0, +4, +8, +0xC` and a flag at `+0x24`; `arg`
/// points to two bound floats. Forms `x1 = ([this+8] + [this+0]) * K` and
/// `x0 = ([this+0xC] + [this+4]) * K` with K = 0.5 (global float at file VA
/// 0xFE8830), in the original's operand order. Returns -1 when the flag is
/// 0; otherwise compares `x0` against `bound[1]` and `x1` against
/// `bound[0]` with `comiss` semantics (unordered counts as below-or-equal)
/// and returns 0, 1, 2 or 3. No calls.
/// Original: 0x00A917E0 (thiscall, ECX + one stack word), 95 bytes.
lf_checker_rt::export!(thiscall, rw_00a917e0(this: u32, arg: u32) -> u32 {
    unsafe {
        const K_GLOBAL: u32 = 0xFE8830;
        const FLAG_OFF: u32 = 0x24;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn load_f(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let k = load_f(lf_checker_rt::relocated(K_GLOBAL));
        let x1 = mul(add(load_f(this.wrapping_add(8)), load_f(this)), k);
        let flag = (this.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        if flag == 0 {
            return 0xFFFF_FFFF;
        }
        let x0 = mul(add(load_f(this.wrapping_add(0xC)), load_f(this.wrapping_add(4))), k);
        let m1 = load_f(arg.wrapping_add(4));
        let m0 = load_f(arg);
        // comiss setbe/jbe: true when below-or-equal OR unordered.
        let below0 = !(x0 > m1);
        if (x1 > m0) {
            if below0 { 0 } else { 2 }
        } else if below0 {
            1
        } else {
            3
        }
    }
});
