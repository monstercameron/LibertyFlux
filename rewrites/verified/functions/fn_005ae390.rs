// original: 0x005ae390 hud_text_attrs_update (proposed)

/// Refresh the HUD/menu text attribute block from the current pad, HUD
/// colour and configuration values, then commit it.
///
/// Reads (file VAs): the two mode bytes at `MODE_A`/`MODE_B`; the pad-state
/// byte at `+0x328D` of the block returned by callee 8; two scale pairs
/// (`SCALE_HI_*` when the pad byte is non-zero, `SCALE_LO_*` otherwise);
/// the gain float at `GAIN`; the colour word written for id `0x41`; two
/// float pairs fetched for ids `0x45` (callee 6) and `0x4D` (callee 19);
/// and two single-shot floats answered on the x87 stack (callees 17, 20).
///
/// Behaviour: pick mode `2` when `MODE_A == 0x6A` or `MODE_B != 0`, else
/// `0` (both equality compares, no signedness); push the colour word, the
/// fetched pairs and several literals through the attribute setters
/// (callees 1, 2, 4-7, 9-16, 18, 21); derive two adjusted scales as
/// `(pair0 - st0a) - C44` and `pair1 - C40`, a truncated colour byte from
/// the `0x4D` fetch (`cvttss2si` semantics: NaN, infinities and values
/// outside `i32` give `0x80000000`, otherwise truncation toward zero, low
/// byte kept and shifted to the top); combine the constants with the gain
/// and the x87 answers in the original's SSE order; commit through callee
/// 22 and return its word. All float arithmetic is pinned to the
/// original's operand order with `black_box`.
///
/// Original: 0x005ae390 (cdecl, no arguments, returns the commit word).
/// Callee 9 cleans its own 2 words (stdcall); every other callee is cdecl.
/// Callee 21 reads a fifth word left over from callee 20's argument slot.
lf_checker_rt::export!(cdecl, rw_005ae390() -> u32 {
    unsafe {
        const MODE_A: u32 = 0x0116C250;
        const MODE_B: u32 = 0x0116C253;
        const MODE_VALUE: u8 = 0x6A;
        const SCALE_HI_1: u32 = 0x01161810;
        const SCALE_HI_0: u32 = 0x01161814;
        const SCALE_LO_1: u32 = 0x01161500;
        const SCALE_LO_0: u32 = 0x01161504;
        const GAIN: u32 = 0x00FE8A24;
        const PAD_BYTE: u32 = 0x328D;
        const TLS_OFF: u32 = 0x78;
        const C44: u32 = 0x3BE56042;
        const C40: u32 = 0x3BA3D70A;
        const NEG1: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(a) as *const u32).read_unaligned()) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// What `cvttss2si` leaves in eax for `x`, as low byte shifted up.
        fn cvt_top(x: f32) -> u32 {
            let v: u32 = if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000
            } else {
                (x as i32) as u32
            };
            (v & 0xFF) << 24
        }

        let g1 = rd8(MODE_A);
        let g2 = rd8(MODE_B);
        let mode: u32 = if g1 == MODE_VALUE || g2 != 0 { 2 } else { 0 };
        lf_checker_rt::callee_cdecl!(1, u32, mode);
        lf_checker_rt::callee_cdecl!(2, u32, 0);

        let mut s48 = [0u32; 1];
        let h1: u32 = lf_checker_rt::callee_cdecl!(3, u32, s48.as_mut_ptr() as u32, 0x41);
        lf_checker_rt::callee_cdecl!(4, u32, (h1 as *const u32).read_unaligned());

        let mut a = [0u32; 2];
        lf_checker_rt::callee_cdecl!(5, u32, a.as_mut_ptr() as u32);
        let mut b = [0u32; 2];
        lf_checker_rt::callee_cdecl!(6, u32, b.as_mut_ptr() as u32, 0x45);
        lf_checker_rt::callee_cdecl!(7, u32, 3, a.as_mut_ptr() as u32, b.as_mut_ptr() as u32, 0);

        let pad: u32 = lf_checker_rt::callee_cdecl!(8, u32, 1);
        let padbyte: u8 = ((pad.wrapping_add(PAD_BYTE)) as *const u8).read();
        let (f0, f1): (f32, f32) = if padbyte != 0 {
            (rdf(SCALE_HI_1), rdf(SCALE_HI_0))
        } else {
            (rdf(SCALE_LO_1), rdf(SCALE_LO_0))
        };
        let r94: u32 = lf_checker_rt::callee_stdcall!(9, u32, f0.to_bits(), f1.to_bits());

        let tls: u32 = lf_checker_rt::tls_slot(0).wrapping_add(TLS_OFF);
        lf_checker_rt::callee_cdecl!(10, u32, tls, r94, NEG1);
        lf_checker_rt::callee_cdecl!(11, u32, tls, tls);

        lf_checker_rt::callee_cdecl!(12, u32, b[0], b[1]);
        lf_checker_rt::callee_cdecl!(13, u32, 0, 0);
        lf_checker_rt::callee_cdecl!(14, u32, 0xb4000000);
        lf_checker_rt::callee_cdecl!(15, u32, 2);
        lf_checker_rt::callee_cdecl!(16, u32, 0, a[0]);

        let st0a: f32 = lf_checker_rt::callee_cdecl!(17, f32, tls, 1);
        let consts = [C44, C40];
        lf_checker_rt::callee_cdecl!(18, u32, 7, 0, consts.as_ptr() as u32, 0);

        let a0f = f32::from_bits(a[0]);
        let a1f = f32::from_bits(a[1]);
        let c44f = f32::from_bits(C44);
        let c40f = f32::from_bits(C40);
        let r1: f32 = sub(sub(a0f, st0a), c44f);
        let r2: f32 = sub(a1f, c40f);

        let mut e12 = [0u32; 2];
        let h2: u32 = lf_checker_rt::callee_cdecl!(19, u32, e12.as_mut_ptr() as u32, 0x4D);
        let color: u32 = cvt_top(f32::from_bits((h2 as *const u32).read_unaligned()));
        let st0b: f32 = lf_checker_rt::callee_cdecl!(20, f32, color);

        let gain = rdf(GAIN);
        let t1 = mul(c40f, gain);
        let arg3 = add(st0b, t1);
        let t2 = mul(c44f, gain);
        let arg2 = add(t2, st0a);
        lf_checker_rt::callee_cdecl!(21, u32, r1.to_bits(), r2.to_bits(), arg2.to_bits(), arg3.to_bits(), color);

        lf_checker_rt::callee_cdecl!(22, u32, 0, a[1], tls, NEG1, NEG1)
    }
});
