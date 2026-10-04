// original: 0x00655250 rows_rotate_about_x (proposed)

/// Rotate the four row-triples of the 3x4 matrix at `this` about its first axis.
///
/// `this` points to twelve floats at `+0x00..+0x38` (four triples; the words
/// at `+0x0c`, `+0x1c` and `+0x2c` are never touched). The rotation angle
/// arrives in the low lane of xmm1. Two helpers are each called once with the
/// angle in xmm0, answering floats f1 and f2 in xmm0; every triple keeps its
/// first element recomputed from the triple alone (no angle terms) while the
/// other two elements are mixed with f1 and f2, each in the original's exact
/// multiply/add order (the multiply-by-zero and add-zero forms are kept
/// verbatim: they are observable on signed zeros and infinities). The
/// original aligns its stack frame and restores it, so the net stack change
/// is zero, and it leaves eax untouched, so there is no return value.
///
/// Original: 0x00655250 (thiscall, no stack words; angle in xmm1).
lf_checker_rt::export!(thiscall, rw_00655250(this: u32) -> u32 {
    unsafe {
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        let esi = this;
        let angle_bits = lf_checker_rt::xmm_word(1, 0);
        let f1 = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, angle_bits));
        let f2 = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, angle_bits));
        let angle = f32::from_bits(angle_bits);
        let mut x0: f32;
        let mut x1 = angle;
        let mut x2: f32;
        let mut x3: f32;
        let mut x4: f32;
        let mut x5: f32;
        let mut x6: f32;
        let mut x7: f32;
        let mut s_08: f32;
        let mut s_0c: f32;
        let mut s_10: f32;
        let mut s_14: f32;
        let mut s_18: f32;
        let mut s_1c: f32;
        let mut s_20: f32;
        let mut s_24: f32;
        let mut s_28: f32;
        let mut s_2c: f32;
        x0 = x1;
        s_0c = x0;
        x0 = f1;
        s_08 = x0;
        x0 = s_0c;
        x0 = f2;
        x4 = rdf(esi + 0x4);
        x1 = rdf(esi + 0x0);
        x7 = x0;
        x0 = rdf(esi + 0x8);
        x6 = 0.0;
        x1 = mul(x1, x6);
        x2 = x4;
        x2 = mul(x2, s_08);
        x5 = x7;
        x5 = neg(x5);
        x2 = add(x2, x1);
        x0 = mul(x0, x5);
        s_14 = x4;
        s_0c = x7;
        x2 = add(x2, x0);
        x0 = rdf(esi + 0x8);
        x0 = mul(x0, s_08);
        s_10 = x5;
        s_18 = x2;
        x2 = x4;
        x4 = rdf(esi + 0x14);
        x2 = mul(x2, x7);
        x3 = x4;
        x3 = mul(x3, x6);
        x2 = add(x2, x1);
        x1 = rdf(esi + 0x18);
        x2 = add(x2, x0);
        x0 = x1;
        x0 = mul(x0, x6);
        s_1c = x2;
        x2 = rdf(esi + 0x10);
        x3 = add(x3, x2);
        x2 = mul(x2, x6);
        x3 = add(x3, x0);
        x0 = x1;
        x1 = mul(x1, s_08);
        x0 = mul(x0, x5);
        s_20 = x3;
        x3 = x4;
        x3 = mul(x3, s_08);
        x4 = mul(x4, x7);
        x3 = add(x3, x2);
        x4 = add(x4, x2);
        x2 = rdf(esi + 0x20);
        x3 = add(x3, x0);
        x4 = add(x4, x1);
        x1 = rdf(esi + 0x28);
        x0 = x1;
        s_24 = x3;
        x3 = rdf(esi + 0x24);
        x7 = x3;
        x7 = mul(x7, x6);
        x0 = mul(x0, x6);
        x7 = add(x7, x2);
        x2 = mul(x2, x6);
        x6 = x3;
        x3 = mul(x3, s_0c);
        x6 = mul(x6, s_08);
        x7 = add(x7, x0);
        x0 = x1;
        x1 = mul(x1, s_08);
        x3 = add(x3, x2);
        x6 = add(x6, x2);
        x2 = rdf(esi + 0x30);
        x0 = mul(x0, x5);
        x5 = rdf(esi + 0x34);
        x3 = add(x3, x1);
        x1 = rdf(esi + 0x38);
        s_28 = x4;
        x6 = add(x6, x0);
        x4 = x5;
        s_2c = x3;
        x3 = 0.0;
        x4 = mul(x4, x3);
        x0 = x1;
        x0 = mul(x0, x3);
        x4 = add(x4, x2);
        x2 = mul(x2, x3);
        x3 = x5;
        x5 = mul(x5, s_0c);
        x3 = mul(x3, s_08);
        x4 = add(x4, x0);
        x5 = add(x5, x2);
        x0 = x1;
        x0 = mul(x0, s_10);
        x1 = mul(x1, s_08);
        x3 = add(x3, x2);
        x2 = s_14;
        x5 = add(x5, x1);
        x1 = 0.0;
        x2 = mul(x2, x1);
        x3 = add(x3, x0);
        x0 = rdf(esi + 0x8);
        x2 = add(x2, rdf(esi + 0x0));
        x0 = mul(x0, x1);
        x2 = add(x2, x0);
        x0 = s_18;
        wrf(esi + 0x0, x2);
        wrf(esi + 0x4, x0);
        x0 = s_1c;
        wrf(esi + 0x8, x0);
        x0 = s_20;
        wrf(esi + 0x10, x0);
        x0 = s_24;
        wrf(esi + 0x14, x0);
        x0 = s_28;
        wrf(esi + 0x18, x0);
        x0 = s_2c;
        wrf(esi + 0x20, x7);
        wrf(esi + 0x24, x6);
        wrf(esi + 0x28, x0);
        wrf(esi + 0x30, x4);
        wrf(esi + 0x34, x3);
        wrf(esi + 0x38, x5);
        0
    }
});
