// original: 0x00d19a30 ped_task_aim_update (proposed)

/// Steer one ped task's aim state toward a target, then resolve its flags.
///
/// `a0` is the task (handler link at `+0xd68`, aim block at `+0xe70`,
/// flag bytes at `+0xe94`), `a1` the target pose the function writes
/// (four floats), `a2` a flag byte gating the second half, `a3` a float
/// threshold, `a4` an opaque word forwarded to two intercepted callees.
/// Returns 1 on success, 0 when either intercepted probe callee rejects.
///
/// The entry path samples a float through an intercepted sampler, clamps
/// `a3` against game data (writing the clamped value back to the caller's
/// argument slot, which the rewrite cannot express and the contract does
/// not compare), and runs two intercepted phases that fill three frame
/// regions: a float region, a second float region, and a byte-flag region
/// modelled here as explicit buffers shared across the calls exactly as the
/// original's frame shares them (later phases overwrite earlier ones).
///
/// Two intercepted arctangent callees take their doubles in vector registers,
/// which the checker cannot forward: their inputs are uncompared and the
/// conversion chains feeding them (plus the direction-normalisation block
/// feeding the second one) are omitted here, while each call itself and its
/// scripted answer are verified. The answers reach the rewrite through
/// scratch words carried in a phase callee's out-words, converted exactly as
/// the original's double-to-float instruction converts them.
///
/// The gated second half walks the flag region (a two-pass byte loop, a
/// compare forest, an eleven-triple scan) and folds the results into a
/// bitfield and three floats in the aim block. Float operation order is the
/// original's, pinned through `black_box` helpers. The security-cookie check
/// runs natively, unpatched.
///
/// Original: 0x00d19a30 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00d19a30(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const G_THRESH: u32 = 0x00e9afb4;
        const G_MODE: u32 = 0x01053c64;
        const G_OFF0: u32 = 0x0110db70;
        const G_OFF1: u32 = 0x0110db74;
        const G_OFF2: u32 = 0x0110db78;
        const C_SIGNMASK: u32 = 0x00fe8fa0;
        const C_SCALE: u32 = 0x00eb77c8;
        const OFF_D68: u32 = 0xd68;
        const OFF_AIM: u32 = 0xe70;
        const OFF_E84: u32 = 0xe84;
        const OFF_E94: u32 = 0xe94;
        const OFF_E95: u32 = 0xe95;
        const OFF_E96: u32 = 0xe96;
        // Byte offsets into the flag-region mirror (original frame E+0x60).
        const F60: usize = 0x000;
        const FB0: usize = 0x050;
        const F100: usize = 0x0a0;
        const F1A0: usize = 0x140;
        const F1F0: usize = 0x190;
        const F240: usize = 0x1e0;
        const F290: usize = 0x230;
        const F380: usize = 0x320;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvt_f64_answer(bits: u64) -> f32 {
            core::hint::black_box(f64::from_bits(bits)) as f32
        }
        /// Exact `cvttss2si`: truncate toward zero; NaN, +2^31 and above,
        /// and below -2^31 yield the indefinite 0x80000000. (Rust's `as`
        /// saturates instead, which differs on those inputs.)
        #[inline(always)]
        fn cvttss2si(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        let g = lf_checker_rt::relocated(G_THRESH);
        let fstd1: f32 = lf_checker_rt::callee_cdecl!(1, f32, a0);
        let mut x = f32::from_bits(a3);
        if !(x > rdf(g)) {
            // The original also stores this back into its incoming float
            // slot; the store is not compared (stack check off).
            x = fstd1;
        }
        let mut fr30 = [0u32; 10];
        let mut fr50 = [0u32; 3];
        let mut stride = [0u8; 0x330];
        let s30 = fr30.as_mut_ptr() as u32;
        let s50 = fr50.as_mut_ptr() as u32;
        let s60 = stride.as_mut_ptr() as u32;
        lf_checker_rt::callee_cdecl!(2, u32, a0, a1, x.to_bits(), s60, rd32(lf_checker_rt::relocated(G_MODE)), 1);
        if lf_checker_rt::callee_cdecl!(3, u32, s60, a1, s30, s50, 1, x.to_bits(), a4) as u8 == 0 {
            return 0;
        }
        // Arctangent #1: the conversion chain feeding its vector inputs is
        // omitted (unobservable); the call and its carried answer are kept.
        lf_checker_rt::callee_cdecl!(6, u32,);
        let ans1 = (fr30[4] as u64) | ((fr30[5] as u64) << 32);
        let xmm3 = cvt_f64_answer(ans1);
        wrf(a1, add(f32::from_bits(fr50[0]), rdf(lf_checker_rt::relocated(G_OFF0))));
        wrf(a1.wrapping_add(4), add(f32::from_bits(fr50[1]), rdf(lf_checker_rt::relocated(G_OFF1))));
        wrf(a1.wrapping_add(8), add(f32::from_bits(fr50[2]), rdf(lf_checker_rt::relocated(G_OFF2))));
        wrf(a1.wrapping_add(0xc), f32::from_bits(fr30[3]));
        lf_checker_rt::callee_cdecl!(4, u32, a0, a1, xmm3.to_bits(), s60, 0xffff_ffff, 0);
        // The original passes an uninitialised frame word here; the contract
        // defines that fill as zero.
        if lf_checker_rt::callee_cdecl!(5, u32, s60, a1, s30, s50, 2, 0, a4) as u8 == 0 {
            return 0;
        }
        // Two-pass byte loop over the flag region.
        let mut cl: u8 = 0;
        let mut flag_f: u8 = 1;
        let mut flag_e: u8 = 0;
        for k in 0..2usize {
            let bb = FB0 + k * 0x50;
            let wb = F240 + k * 0x50;
            if stride[bb] != 0 {
                if stride[wb] == 0 {
                    flag_f = 0;
                    break;
                }
                let ch = stride[wb + 2];
                if ch != 0 {
                    if stride[bb + 2] == 0 {
                        flag_f = 0;
                        break;
                    }
                    if stride[F380] == 0 {
                        flag_f = 0;
                        break;
                    }
                    if stride[F380 + 2] == 0 {
                        flag_f = 0;
                        break;
                    }
                }
                if stride[bb + 2] != 0 || ch != 0 {
                    cl = 1;
                }
                flag_e = cl;
            }
        }
        if (a2 as u8) == 0 {
            return 1;
        }
        // Gated half. The direction block feeding arctangent #2 is omitted
        // (unobservable); only the snapshot word it leaves behind is kept.
        let mask = rd32(lf_checker_rt::relocated(C_SIGNMASK));
        fr30[0] = fr30[2] ^ mask;
        let d68 = rd32(a0.wrapping_add(OFF_D68));
        if d68 != 0 {
            lf_checker_rt::callee_thiscall!(7, u32, d68, s30, 0);
        }
        lf_checker_rt::callee_cdecl!(8, u32,);
        let ans2 = (fr30[6] as u64) | ((fr30[7] as u64) << 32);
        let a35arg = cvt_f64_answer(ans2);
        let fstd9: f32 = lf_checker_rt::callee_cdecl!(9, f32, a35arg.to_bits());
        let scaled = mul(fstd9, rdf(lf_checker_rt::relocated(C_SCALE)));
        let mut ecx: u32 = 0;
        if flag_f != 0 {
            ecx = 3;
        }
        let mut edx: u32 = 3;
        let stack28 = ecx;
        let ah: u8 = if stride[F60] == 0 {
            0
        } else if stride[F1F0] == 0 {
            if ecx == edx { 0 } else { 1 }
        } else if stride[F1F0 + 2] == 0 {
            1
        } else if flag_e != 0 {
            1
        } else if ecx == edx {
            0
        } else {
            1
        };
        let al2: u8 = if stride[F1A0] == 0 {
            0
        } else if stride[0x2d0] == 0 {
            if ecx == edx { 0 } else { 1 }
        } else if stride[0x2d2] == 0 {
            1
        } else if flag_e != 0 {
            1
        } else if ecx == edx {
            0
        } else {
            1
        };
        if ah != 0 {
            edx = 0;
            ecx = 2;
        } else {
            ecx = 1;
        }
        wr8(a0.wrapping_add(OFF_E94), stride[F60]);
        wr8(a0.wrapping_add(OFF_E95), stride[F1A0]);
        if al2 != 0 {
            edx = ecx;
        }
        wr8(a0.wrapping_add(OFF_E96), 0);
        for i in 0..11usize {
            if rd8(a0.wrapping_add(OFF_E96)) == 0 {
                let t = F60 + i * 0x50;
                if stride[t] != 0 && stride[t + 1] == 0 && stride[t + 2] == 0 {
                    wr8(a0.wrapping_add(OFF_E96), 1);
                }
            }
        }
        let mut edxb = (((edx & 3) << 2) | (stack28 & 3)) << 3;
        let ival = cvttss2si(scaled);
        let aim = a0.wrapping_add(OFF_AIM);
        let ecx_b = ((((ival as u8) as u32) << 15) | (rd32(aim) & 0xff80_7ffc)) & 0xffff_8007;
        wrf(aim.wrapping_add(4), f32::from_bits(fr50[0]));
        wrf(aim.wrapping_add(8), f32::from_bits(fr50[1]));
        edxb |= ecx_b;
        edxb |= 4;
        wr32(aim, edxb);
        wrf(aim.wrapping_add(0xc), f32::from_bits(fr50[2]));
        let e84 = rd32(a0.wrapping_add(OFF_E84));
        if e84 != 0 {
            lf_checker_rt::callee_thiscall!(10, u32, e84, a0.wrapping_add(OFF_E84));
        }
        wr32(a0.wrapping_add(OFF_E84), 0);
        wr32(aim.wrapping_add(0x10), 0);
        wr32(aim.wrapping_add(0x20), 0);
        wr32(aim.wrapping_add(0x1c), 0);
        wr32(aim.wrapping_add(0x18), 0);
        lf_checker_rt::callee_thiscall!(11, u32, a0);
        lf_checker_rt::callee_thiscall!(12, u32, a0, aim);
        1
    }
});
