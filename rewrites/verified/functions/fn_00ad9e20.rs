// original: 0x00ad9e20 ui_pointer_scaler (proposed)

/// Scale one pointer reading into six integers for the handler object.
///
/// `pair` points at two floats (x, y). A global integer `G` is read as a
/// float. Six blocks each call the base callee (no arguments; it answers a
/// handler pointer) and store one integer at a fixed offset from it:
///
/// * `+0x10ac`: x truncated toward zero; `+0x10b0`: y truncated toward zero.
/// * `+0x10b4`: twice the adjusted round of `(x - G) * 0.5`; `+0x10bc`: the
///   same of y. The adjust rounds half-even through the 2^23 add/subtract
///   trick (skipped when already past 2^23), then subtracts one when the
///   rounded value lies above the input, all with the original's exact
///   operand order so NaN payloads match.
/// * `+0x10b8`: twice the truncation of the double callee's answer for
///   `(G + x) * 0.5` widened to f64; `+0x10c0`: the same of y. The double
///   callee takes the f64 bits as two words and answers in ST0.
///
/// Truncation follows x86 `cvttss2si` exactly: NaN and inputs at or past
/// 2^31 yield `i32::MIN`, everything else truncates toward zero, then the
/// result is doubled wrapping.
///
/// Original: 0x00ad9e20 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00ad9e20(pair: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const DBL_CALLEE: u32 = 2;
        const GVAR: u32 = 0x0103_f4bc;
        const HALF: f32 = 0.5;
        const ROUND_MAG: f32 = 8388608.0; // 2^23
        const I32_EDGE: f32 = 2147483648.0; // 2^31

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// x86 cvttss2si semantics.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= I32_EDGE {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// Round-half-even via the 2^23 trick, then step down when the
        /// rounded value sits above the input. Bit tricks, not branches.
        #[inline(always)]
        fn adjust(v: f32) -> f32 {
            let sign = f32::from_bits(v.to_bits() & 0x8000_0000);
            let mag = f32::from_bits(v.to_bits() & 0x7fff_ffff);
            let c: u32 = if mag < ROUND_MAG { 0xffff_ffff } else { 0 };
            let m = f32::from_bits((ROUND_MAG.to_bits() & c) | sign.to_bits());
            let r = sub(add(v, m), m);
            let d = sub(r, v);
            let c2: u32 = if !(d <= sign) { 0xffff_ffff } else { 0 }; // cmpnless: !(d<=s), NaN true
            let e = f32::from_bits(1.0f32.to_bits() & c2);
            sub(r, e)
        }

        let g = (lf_checker_rt::global::<i32>(GVAR) as *const i32).read_unaligned() as f32;
        let x = (pair as *const f32).read_unaligned();
        let y = (pair.wrapping_add(4) as *const f32).read_unaligned();

        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10ac) as *mut u32).write_unaligned(cvtt(x) as u32);
        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10b0) as *mut u32).write_unaligned(cvtt(y) as u32);

        let v = mul(sub(x, g), HALF);
        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10b4) as *mut u32)
            .write_unaligned((cvtt(adjust(v)) as u32).wrapping_mul(2));

        let t = mul(add(g, x), HALF);
        let bits = (t as f64).to_bits();
        let d: f64 = lf_checker_rt::callee_cdecl!(
            DBL_CALLEE, f64, (bits & 0xffff_ffff) as u32, (bits >> 32) as u32
        );
        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10b8) as *mut u32)
            .write_unaligned((cvtt(d as f32) as u32).wrapping_mul(2));

        let v = mul(sub(y, g), HALF);
        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10bc) as *mut u32)
            .write_unaligned((cvtt(adjust(v)) as u32).wrapping_mul(2));

        let t = mul(add(g, y), HALF);
        let bits = (t as f64).to_bits();
        let d: f64 = lf_checker_rt::callee_cdecl!(
            DBL_CALLEE, f64, (bits & 0xffff_ffff) as u32, (bits >> 32) as u32
        );
        let h: u32 = lf_checker_rt::callee_cdecl!(BASE_CALLEE, u32,);
        (h.wrapping_add(0x10c0) as *mut u32)
            .write_unaligned((cvtt(d as f32) as u32).wrapping_mul(2));
        0
    }
});
