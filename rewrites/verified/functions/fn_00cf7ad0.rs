// original: 0x00cf7ad0 climb_anchor_solve (proposed)

/// Solves a climb anchor from five inputs: classifies the float argument
/// twice through float-in-xmm0 helpers (answers ignored, passed values
/// compared), scales the 0.42 constant by the negated float and by the float,
/// then combines the picked source block (fourth argument when the mode word
/// equals 4, else the fifth) with the constants: `out[0] = src[0] - 0.42*-f`,
/// `out[4] = src[4] - 0.42*f`, `out[8]` starts from `src[8]`, minus
/// `0.42*0.0`, minus 1.0 and 0.08 on the mode-4 path or plus 1.0 otherwise,
/// and `out[0xc]` copies `src[0xc]`. Returns the output pointer.
///
/// Original: 0x00cf7ad0 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00cf7ad0(out: u32, mode: u32, f: u32, src4: u32, srcn: u32) -> u32 {
    unsafe {
        const MASK_ADDR: u32 = 0x00fe8fa0;
        const K_ADDR: u32 = 0x010539e8; // 0.42f
        const ZERO_ADDR: u32 = 0x00fe8628; // 0.0f
        const ONE_ADDR: u32 = 0x00fe88e8; // 1.0f
        const BIAS_ADDR: u32 = 0x00fe878c; // 0.08f
        const CLASS_A: u32 = 1;
        const CLASS_B: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let fv = f32::from_bits(f);
        lf_checker_rt::callee_cdecl!(CLASS_A, u32, f);
        let mask = (lf_checker_rt::global::<u32>(MASK_ADDR)).read_unaligned();
        let neg = f32::from_bits(f ^ mask);
        lf_checker_rt::callee_cdecl!(CLASS_B, u32, f);
        let k = f32::from_bits((lf_checker_rt::global::<u32>(K_ADDR)).read_unaligned());
        let z = f32::from_bits((lf_checker_rt::global::<u32>(ZERO_ADDR)).read_unaligned());
        let one = f32::from_bits((lf_checker_rt::global::<u32>(ONE_ADDR)).read_unaligned());
        let t1 = mul(k, neg);
        let t3 = mul(k, z);
        let t2 = mul(k, fv);
        let src = if mode == 4 { src4 } else { srcn };
        let mut w8 = f32::from_bits(((src + 8) as *const u32).read_unaligned());
        w8 = sub(w8, t3);
        if mode == 4 {
            let bias = f32::from_bits((lf_checker_rt::global::<u32>(BIAS_ADDR)).read_unaligned());
            w8 = sub(w8, one);
            w8 = sub(w8, bias);
        } else {
            w8 = add(w8, one);
        }
        let w0 = f32::from_bits((src as *const u32).read_unaligned());
        let wc = ((src + 0xc) as *const u32).read_unaligned();
        let r0 = sub(w0, t1);
        (out as *mut u32).write_unaligned(r0.to_bits());
        ((out + 8) as *mut u32).write_unaligned(w8.to_bits());
        ((out + 0xc) as *mut u32).write_unaligned(wc);
        let w4 = f32::from_bits(((src + 4) as *const u32).read_unaligned());
        let r4 = sub(w4, t2);
        ((out + 4) as *mut u32).write_unaligned(r4.to_bits());
        out
    }
});
