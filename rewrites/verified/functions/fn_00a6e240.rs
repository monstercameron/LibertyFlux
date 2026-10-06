// original: 0x00a6e240 peds_task_update_aim_state (proposed)

/// Gated update of a ped task's aim state, solved through the sibling
/// direction solver and a chain of distance and facing checks.
///
/// `this` is the task owner, `arg` the task. Bytes `arg_c`, `arg_10`,
/// `arg_14` steer the early exits. The task object at `arg+0x228` must have
/// bit 0 of word `+0x560` set, and two xor-obfuscated marker bytes (at
/// `+0x285E`/`+0x285F` of the helper object, xored with the key byte at
/// `+0x285C`, compared UNSIGNED against 0x7F) decide with `arg_c`/`arg_14`
/// whether the update runs at all. Any failed gate returns 0.
///
/// The running update: the direction solver (callee 2, the function at
/// 0x00a6da70, stubbed here) fills a stack aim struct; a position triple is
/// gathered from `arg+0x20` (`+0x30`/`+0x34`/`+0x38`, the last overridable
/// through `arg+0x38`); several distance-squared gates compare gathered
/// points against the task position, with thresholds 1.0, 0.1, 0.0, 0.2,
/// 0.65 and 0.25 (all `comiss` against read-only constants; unordered
/// results take the `jbe` side and skip the `ja`/`jae` side); a global
/// three-float cache with a written-once flag is consulted; a manager word
/// has bits 5-6 cleared; and a facing dot product steers the final stretch.
/// The tail either returns 0 or threads a global manager pointer through
/// two helpers and returns the last answer.
///
/// Calling convention: thiscall, `this` in ECX, four stack words,
/// callee pops 0x10. Several callees take out-pointers into this frame and
/// one takes a double pair in XMM0/XMM1; those are the checker's
/// stack transports. Frame slots the original reads without writing (the
/// three copy source words) hold the checker's stack fill, 0.
lf_checker_rt::export!(thiscall, rw_00a6e240(this_: u32, a_si: u32, a_c: u32, a_10: u32, a_14: u32) -> u32 {
    unsafe {
        const TASK_OF_ARG: u32 = 0x228;
        const FLAG_OFF: u32 = 0x560;
        const POS_OF_ARG: u32 = 0x20;
        const OVERRIDE_OFF: u32 = 0x38;
        const MGR_OFF: u32 = 0xD68;
        const KEY_OFF: u32 = 0x285C;
        const MARK6_OFF: u32 = 0x285E;
        const MARK7_OFF: u32 = 0x285F;
        const CODE_TAG: u32 = 0xA73270;
        const FIVE_BITS: u32 = 0x40A00000;
        const NEG999_BITS: u32 = 0xC479C000;
        const HUNDRED_BITS: u32 = 0x42C80000;
        const GLOB_TH1: u32 = 0x103CEAC;
        const GLOB_TH01: u32 = 0xFE879C;
        const GLOB_ZERO: u32 = 0xFE8628;
        const GLOB_TH02: u32 = 0xFE87D0;
        const GLOB_TH065: u32 = 0xFE8864;
        const GLOB_PT2: u32 = 0x103CECC;
        const GLOB_TH025: u32 = 0xFE87E4;
        const GLOB_ONE: u32 = 0xFE88E8;
        const GLOB_CACHE0: u32 = 0xE9F140;
        const GLOB_CACHE1: u32 = 0xE9F124;
        const GLOB_CACHE2: u32 = 0xE9F128;
        const GLOB_SLOT0: u32 = 0x12FA740;
        const GLOB_SLOT1: u32 = 0x12FA744;
        const GLOB_SLOT2: u32 = 0x12FA748;
        const GLOB_FLAG: u32 = 0x12FA750;
        const GLOB_MODE: u32 = 0x11D6FD4;
        const GLOB_MGRPTR: u32 = 0x167E2A0;
        const SIGN_BIT: u32 = 0x8000_0000;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn glob_f(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(a))) }
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
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }
        /// Original's `comiss a, b; jbe`: taken when a <= b or unordered.
        #[inline(always)]
        fn jbe(a: f32, b: f32) -> bool {
            !(a > b)
        }
        #[inline(always)]
        unsafe fn tail(mgr: u32, esi: u32) -> u32 {
            unsafe {
                if mgr == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(8u32, u32, mgr, esi);
                let gp = rd32(lf_checker_rt::relocated(GLOB_MGRPTR));
                let r17: u32 = lf_checker_rt::callee_thiscall!(20u32, u32, gp);
                if r17 == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(21u32, u32, r17, 0)
            }
        }

        let esi = a_si;
        let arg_c = (a_c & 0xFF) as u8;
        let arg_10 = (a_10 & 0xFF) as u8;
        let arg_14 = (a_14 & 0xFF) as u8;
        let p: u32 = lf_checker_rt::callee_thiscall!(1u32, u32, esi);
        let dl = rd8(p + KEY_OFF);
        // Both marker compares are UNSIGNED (jbe after (an instruction of the original)).
        let b6 = rd8(p + MARK6_OFF) ^ dl;
        let mut cont = false;
        if b6 > 0x7F {
            let b7 = rd8(p + MARK7_OFF) ^ dl;
            if b7 <= 0x7F {
                cont = true;
            }
        }
        if !cont {
            if arg_c != 0 {
                cont = true;
            } else if arg_14 != 0 {
                cont = true;
            } else {
                return 0;
            }
        }
        let t = rd32(esi + TASK_OF_ARG);
        if rd8(t + FLAG_OFF) & 1 == 0 {
            return 0;
        }
        let u = rd32(esi + POS_OF_ARG);
        // Aim struct for the direction solver: its words 0..3 are the
        // solver's outputs (scripted writes here), words 9..11 carry the
        // task pointer and the mode byte at +0x24/+0x28/+0x2C.
        let mut aim = [0u32; 12];
        aim[9] = arg_10 as u32;
        aim[10] = esi;
        aim[11] = arg_10 as u32;
        let mut edi_copy = [0u32; 4];
        if arg_10 != 0 {
            edi_copy[0] = rd32(this_ + 0x60);
            edi_copy[1] = rd32(this_ + 0x64);
            edi_copy[2] = rd32(this_ + 0x68);
            edi_copy[3] = rd32(this_ + 0x6C);
        }
        // The copies above are never read back downstream.
        let _ = &edi_copy;
        let aim_ptr = aim.as_mut_ptr() as u32;
        let c2r: u32 = lf_checker_rt::callee_cdecl!(2u32, u32, aim_ptr);
        if (c2r & 0xFF) == 0 {
            return 0;
        }
        let mut pos = [0u32; 4];
        pos[0] = rd32(u + 0x30);
        pos[1] = rd32(u + 0x34);
        pos[2] = rd32(u + 0x38);
        pos[3] = 0;
        let ov = rd32(esi + OVERRIDE_OFF);
        if ov != 0 {
            pos[2] = rd32(ov + 0x48);
        }
        if arg_10 == 0 {
            lf_checker_rt::callee_cdecl!(3u32, u32, esi, pos.as_mut_ptr() as u32, 1, NEG999_BITS, 0);
        }
        let m = rd32(esi + MGR_OFF);
        // One shared out-triple for the three mode calls below: the
        // original reuses the same three frame slots, so each call's entry
        // snapshot sees the previous call's results.
        let mut c4t = [0u32; 3];
        if m != 0 && arg_10 != 0 {
            lf_checker_rt::callee_thiscall!(4u32, u32, m, c4t.as_mut_ptr() as u32, 0);
            let q0 = f32::from_bits(c4t[0]);
            let q1 = f32::from_bits(c4t[1]);
            let mut q2 = f32::from_bits(c4t[2]);
            q2 = add(q2, glob_f(GLOB_ONE));
            c4t[2] = q2.to_bits();
            let d1 = sub(q0, rdf(u + 0x30));
            let d0 = sub(q1, rdf(u + 0x34));
            let d2 = sub(q2, rdf(u + 0x38));
            let sum = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
            if glob_f(GLOB_TH1) > sum {
                lf_checker_rt::callee_thiscall!(5u32, u32, esi, 0);
            }
        }
        let edi2: u32 = lf_checker_rt::callee_cdecl!(
            6u32, u32,
            u.wrapping_add(0x30), FIVE_BITS, 0, 0,
            lf_checker_rt::relocated(CODE_TAG), 0, aim_ptr
        );
        if edi2 == 0 {
            return tail(m, esi);
        }
        let mut flag04 = 0u32;
        if rd32(lf_checker_rt::relocated(GLOB_MODE)) == 2 {
            let c4b: u32 = lf_checker_rt::callee_thiscall!(4u32, u32, edi2, c4t.as_mut_ptr() as u32, 0);
            if (c4b & 0xFF) != 0 {
                let flag_addr = lf_checker_rt::relocated(GLOB_FLAG);
                let flag = rd32(flag_addr);
                let (s0, s1, s2);
                if flag & 1 != 0 {
                    s0 = rdf(lf_checker_rt::relocated(GLOB_SLOT0));
                    s1 = rdf(lf_checker_rt::relocated(GLOB_SLOT1));
                    s2 = rdf(lf_checker_rt::relocated(GLOB_SLOT2));
                } else {
                    s0 = glob_f(GLOB_CACHE0);
                    s1 = glob_f(GLOB_CACHE1);
                    s2 = glob_f(GLOB_CACHE2);
                    wr32(flag_addr, flag | 1);
                    wr32(lf_checker_rt::relocated(GLOB_SLOT0), s0.to_bits());
                    wr32(lf_checker_rt::relocated(GLOB_SLOT1), s1.to_bits());
                    wr32(lf_checker_rt::relocated(GLOB_SLOT2), s2.to_bits());
                }
                let e0 = f32::from_bits(c4t[0]);
                let e1 = f32::from_bits(c4t[1]);
                let e2 = f32::from_bits(c4t[2]);
                let dd2 = sub(e0, s0);
                let dd0 = sub(e1, s1);
                let dd1 = sub(e2, s2);
                let sum = add(add(mul(dd2, dd2), mul(dd0, dd0)), mul(dd1, dd1));
                if glob_f(GLOB_TH01) > sum {
                    flag04 = 1;
                }
            }
        }
        let mut cur7 = [0u32; 4];
        lf_checker_rt::callee_thiscall!(7u32, u32, cur7.as_mut_ptr() as u32);
        let mut db = [0u32; 3];
        let mut eb = [0u32; 3];
        if m != 0 {
            lf_checker_rt::callee_thiscall!(9u32, u32, edi2, db.as_mut_ptr() as u32, 0);
            lf_checker_rt::callee_thiscall!(10u32, u32, m, eb.as_mut_ptr() as u32, 0);
            lf_checker_rt::callee_thiscall!(4u32, u32, m, c4t.as_mut_ptr() as u32, 0);
            if (flag04 & 0xFF) == 0 {
                let c0 = f32::from_bits(c4t[0]);
                let c1 = f32::from_bits(c4t[1]);
                let c2v = add(f32::from_bits(c4t[2]), glob_f(GLOB_ONE));
                c4t[2] = c2v.to_bits();
                let e0 = f32::from_bits(eb[0]);
                let e1 = f32::from_bits(eb[1]);
                let e2 = f32::from_bits(eb[2]);
                let w0 = f32::from_bits(aim[0]);
                let w1 = f32::from_bits(aim[1]);
                let w2 = f32::from_bits(aim[2]);
                let d1 = add(add(mul(e0, w0), mul(e1, w1)), mul(e2, w2));
                if d1 >= 0.0 {
                    return tail(m, esi);
                }
                let f0 = f32::from_bits(db[0]);
                let f1 = f32::from_bits(db[1]);
                let f2 = f32::from_bits(db[2]);
                let d2 = add(add(mul(f1, e1), mul(f0, e0)), mul(f2, e2));
                if d2 > glob_f(GLOB_TH02) {
                    return tail(m, esi);
                }
                let g1 = sub(c1, rdf(u + 0x34));
                let g0 = sub(c0, rdf(u + 0x30));
                let g2 = sub(c2v, rdf(u + 0x38));
                let d3 = add(add(mul(g1, g1), mul(g0, g0)), mul(g2, g2));
                if glob_f(GLOB_TH065) >= d3 {
                    return tail(m, esi);
                }
            }
        }
        lf_checker_rt::callee_thiscall!(5u32, u32, esi, edi2);
        lf_checker_rt::callee_thiscall!(8u32, u32, m, esi);
        let mut cp = [0u32; 4];
        lf_checker_rt::callee_cdecl!(12u32, u32, m, esi, aim_ptr, cp.as_mut_ptr() as u32, 0);
        lf_checker_rt::callee_thiscall!(13u32, u32, m, esi);
        // The original copies the three still-uninitialized words over the
        // middle triple (all stack fill, 0.0), killing the scripted values.
        db[0] = cp[0];
        db[1] = cp[1];
        db[2] = cp[2];
        let c12r: u32 = lf_checker_rt::callee_cdecl!(
            14u32, u32, db.as_mut_ptr() as u32, 0, 0, 0, HUNDRED_BITS, 0xE, 0
        );
        if (c12r & 0xFF) != 0 {
            cp[0] = db[0];
            cp[1] = db[1];
            cp[2] = db[2];
            cp[3] = 0;
        }
        let mut fb = [0u32; 2];
        lf_checker_rt::callee_thiscall!(11u32, u32, m, fb.as_mut_ptr() as u32, aim_ptr);
        let f0 = f32::from_bits(fb[0]);
        let f1 = f32::from_bits(fb[1]);
        let d0 = f64::from(neg(f0));
        let d1lo = f64::from(f1);
        let d0b = d0.to_bits();
        let d1b = d1lo.to_bits();
        let h_bits: u64 = lf_checker_rt::callee_cdecl!(
            15u32, u64,
            (d0b & 0xFFFF_FFFF) as u32, (d0b >> 32) as u32,
            (d1b & 0xFFFF_FFFF) as u32, (d1b >> 32) as u32
        );
        let h = f64::from_bits(h_bits) as f32;
        let st: f32 = lf_checker_rt::callee_cdecl!(16u32, f32, h.to_bits());
        let m0 = rd32(m);
        core::hint::black_box(m0); // pin: original faults here for null M
        let mut s20 = (m0 >> 5) & 3;
        let c3b: u32 = lf_checker_rt::callee_cdecl!(3u32, u32, esi, cp.as_mut_ptr() as u32, 1, st.to_bits(), 0);
        if (c3b & 0xFF) != 0 {
            let m0b = rd32(m);
            wr32(m, m0b ^ ((((s20 & 0xFF) << 5) ^ m0b) & 0x60));
            lf_checker_rt::callee_thiscall!(8u32, u32, m, esi);
            lf_checker_rt::callee_cdecl!(12u32, u32, m, esi, aim_ptr, cp.as_mut_ptr() as u32, 0);
            lf_checker_rt::callee_thiscall!(13u32, u32, m, esi);
            let t30 = add(f32::from_bits(cp[2]), glob_f(GLOB_ONE));
            let t28 = f32::from_bits(cp[0]);
            let t2c = f32::from_bits(cp[1]);
            let dy = sub(t2c, rdf(u + 0x34));
            let dx = sub(t28, rdf(u + 0x30));
            s20 = t30.to_bits();
            let dz = sub(t30, rdf(u + 0x38));
            let norm = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            if jbe(norm, glob_f(GLOB_TH025)) {
                return tail(m, esi);
            }
            let mut eb2 = [dx.to_bits(), dy.to_bits(), dz.to_bits()];
            lf_checker_rt::callee_thiscall!(17u32, u32, eb2.as_mut_ptr() as u32);
            let sc = glob_f(GLOB_PT2);
            let v2 = mul(dx, sc);
            let v0 = mul(dy, sc);
            let v1 = mul(dz, sc);
            let n8 = sub(t28, v2);
            let nC = sub(t2c, v0);
            let n10 = sub(f32::from_bits(s20), v1);
            let mut post = [n8.to_bits(), nC.to_bits(), n10.to_bits()];
            let mut cur15 = [0u32; 2];
            lf_checker_rt::callee_thiscall!(18u32, u32, cur15.as_mut_ptr() as u32);
            let c16r: u32 = lf_checker_rt::callee_cdecl!(
                19u32, u32, u.wrapping_add(0x30), post.as_mut_ptr() as u32,
                sc.to_bits(), 0, 6, 0, 0, 0
            );
            if c16r == 0 {
                return tail(m, esi);
            }
        }
        lf_checker_rt::callee_thiscall!(13u32, u32, m, esi);
        lf_checker_rt::callee_thiscall!(5u32, u32, esi, 0);
        tail(m, esi)
    }
});
