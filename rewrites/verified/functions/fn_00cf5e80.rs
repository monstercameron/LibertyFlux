// original: 0x00cf5e80 ped_task_climb_update (proposed)

/// Climb-task update: solve placement, run notifications, advance state.
///
/// `this` is the task object, `arg0`/`arg1` related objects and `arg2` a
/// float carried to several callees. Original convention: thiscall with
/// three stack words (the callee pops 0xc bytes).
///
/// Layout used: task words at `+0x10` (child, selects the entry path when
/// zero), `+0x14` (state), `+0x40`..`+0x5c` (pose floats), `+0x64` (matrix
/// object or zero), `+0x74` (mode word the first callee fills), `+0x89`
/// (flag byte, bit 1 set); `arg0` words at `+0x20` (float-triple pointer),
/// `+0x26c` (mask, bit 0 cleared) and `+0xa80` (notify object); `arg1` three
/// floats at `+0x00`/`+0x04`/`+0x08`. The matrix object exposes `+0x20` (row
/// pointer, filled by an init callee on the cold path), flag bits at `+0x28`
/// and a mode at `+0x1304`. The child exposes a selector at `+0x0c`, a flag
/// word at `+0x46` and a float at `+0x4c`.
///
/// Behaviour. With no child, a setup callee fills the mode word, a solve
/// callee fills two frame words, and either a constant pair (mode 0x69) or
/// the solved pair feeds a sine/cosine pair whose scaled results join
/// differenced position floats in the pose slots. Two guarded matrix blocks
/// then transform the pose (the cold path runs the init pair first). A
/// notify chain of four calls follows. The tail (also taken directly when a
/// child is present) measures a segment length, picks a cap from tuned
/// globals, clamps it, scales the direction by an arc callee's answer and
/// an optional fit callee, and adds it to the pose. A late gate on a global
/// and the matrix flags can exit with state 8. With no child the tail calls
/// an action callee and returns its answer when it fails, else it faults on
/// a null selector read (mirrored for parity); with a child it returns the
/// tracked integer answer. The far region re-solves two frame words, runs
/// the action callee, and dispatches on the child selector through a
/// 12-entry map into a 5-case table (states 2/1/5/5/8, one with a clip
/// call), merging into a final callee whose answer is returned.
///
/// Edge cases: only low bytes drive branches except one full-`eax` test;
/// float compares are ordered; the sine/cosine take `f32` in XMM0 (4-byte
/// transports) and return `f32` in XMM0, consumed via EAX which the stub
/// sets to the same bits; the cosine result is negated (sign-bit xor);
/// `[F+0x6C]` is unwritten scratch (zero fill) stored twice; the action
/// argument on the stale path is the still-live 8.0 load; faulting trials
/// (null selector reads) pass by fault-code parity.
lf_checker_rt::export!(thiscall, rw_00cf5e80(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const C_SETUP: u32 = 1;
        const C_SOLVE: u32 = 2;
        const C_SIN: u32 = 3;
        const C_COS: u32 = 4;
        const C_INIT: u32 = 5;
        const C_MAT: u32 = 6;
        const C_N1: u32 = 7;
        const C_N2: u32 = 8;
        const C_N3: u32 = 9;
        const C_N4: u32 = 10;
        const C_RANGE: u32 = 11;
        const C_ARC: u32 = 12;
        const C_FIT: u32 = 13;
        const C_ACTION: u32 = 14;
        const C_CLIP: u32 = 15;
        const C_FINAL: u32 = 17;

        const T_CHILD: u32 = 0x10;
        const T_STATE: u32 = 0x14;
        const T_P0: u32 = 0x40;
        const T_P1: u32 = 0x44;
        const T_P2: u32 = 0x48;
        const T_P3: u32 = 0x4c;
        const T_Q0: u32 = 0x50;
        const T_Q1: u32 = 0x54;
        const T_Q2: u32 = 0x58;
        const T_Q3: u32 = 0x5c;
        const T_X: u32 = 0x64;
        const T_MODE: u32 = 0x74;
        const T_FLAGS: u32 = 0x89;
        const A0_VEC: u32 = 0x20;
        const A0_MASK: u32 = 0x26c;
        const A0_NOT: u32 = 0xa80;
        const X_MAT: u32 = 0x20;
        const X_FL: u32 = 0x28;
        const X_MODE: u32 = 0x1304;
        const CH_SEL: u32 = 0x0c;
        const CH_FL: u32 = 0x46;
        const CH_F: u32 = 0x4c;
        const G_D: u32 = 0x011d_6fd4;
        const G_17: u32 = 0x0117_35bc;
        const G_38: u32 = 0x0105_3938;
        const G_3C: u32 = 0x0105_393c;
        const G_40: u32 = 0x0105_3940;
        const G_44: u32 = 0x0105_3944;
        const G_48: u32 = 0x0105_3948;

        const K40: f32 = f32::from_bits(0x3ecc_cccd); // 0.4
        const KN95: f32 = f32::from_bits(0xbf73_3333); // -0.95
        const K001: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const K8: f32 = f32::from_bits(0x4100_0000); // 8.0
        const K1000: f32 = f32::from_bits(0x447a_0000); // 1000.0
        const K1: f32 = f32::from_bits(0x3f80_0000); // 1.0
        const KCLIP: f32 = f32::from_bits(0x3e19_999a); // 0.15
        const RATE: u32 = 0x447a_0000;
        const SWMAP: [u8; 12] = [0, 4, 4, 1, 2, 4, 4, 4, 4, 4, 3, 3];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        unsafe fn glob(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn globf(va: u32) -> f32 {
            unsafe { f32::from_bits(glob(va)) }
        }

        let edi = this;
        let mut t2c = 0xffff_ffffu32;
        let child0 = rd32(edi + T_CHILD);
        if child0 == 0 {
            let w74 = edi.wrapping_add(T_MODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_SETUP, u32, edi, arg0, arg1, arg2, w74);
            let mode = rd32(w74);
            let mut f38 = 0u32;
            let mut f40 = 0u32;
            let _: u32 = lf_checker_rt::callee_stdcall!(
                C_SOLVE, u32, mode, (&mut f38 as *mut u32) as u32,
                (&mut f40 as *mut u32) as u32);
            t2c = mode;
            let (f28, f30): (u32, u32);
            if mode == 0x69 {
                f28 = K40.to_bits();
                t2c = 0x68;
                f30 = KN95.to_bits();
            } else {
                f30 = f40;
                f28 = f38;
            }
            let esi = arg0;
            let base = rd32(esi + A0_VEC);
            let d0 = sub(rdf(base + 48), rdf(arg1));
            let d1 = sub(rdf(base + 52), rdf(arg1.wrapping_add(4)));
            let d2 = sub(rdf(base + 56), rdf(arg1.wrapping_add(8)));
            wr32(edi + T_P0, d0.to_bits());
            wr32(edi + T_P3, 0);
            wr32(edi + T_P1, d1.to_bits());
            wr32(edi + T_P2, d2.to_bits());
            let s: u32 = lf_checker_rt::callee_cdecl!(C_SIN, u32, arg2);
            let m1 = mul(f32::from_bits(s), f32::from_bits(f28));
            let c: u32 = lf_checker_rt::callee_cdecl!(C_COS, u32, arg2);
            let m2 = mul(f32::from_bits(c), f32::from_bits(f28));
            wr32(edi + T_Q0, m1.to_bits());
            wr32(edi + T_Q1, m2.to_bits() ^ 0x8000_0000);
            wr32(edi + T_Q2, f30);
            wr32(edi + T_Q3, 0);
            let mut x = rd32(edi + T_X);
            if x != 0 {
                let nyb = (rd32(x + X_FL) >> 6) & 0xF;
                if nyb > 1 && nyb < 5 {
                    if rd32(x + X_MAT) == 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(C_INIT, u32, x);
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            C_MAT, u32, x.wrapping_add(16), rd32(x + X_MAT));
                    }
                    let m = rd32(x + X_MAT);
                    let e44 = rdf(edi + T_P1);
                    let e40 = rdf(edi + T_P0);
                    let e48 = rdf(edi + T_P2);
                    let mut x6 = rdf(m + 4);
                    let mut x2 = rdf(m + 20);
                    let mut x1 = rdf(m + 36);
                    let mut x0 = rdf(m);
                    x0 = mul(x0, e40);
                    x6 = mul(x6, e44);
                    x2 = mul(x2, e44);
                    x6 = add(x6, x0);
                    x0 = rdf(m + 8);
                    x0 = mul(x0, e48);
                    x1 = mul(x1, e44);
                    x6 = add(x6, x0);
                    x0 = e40;
                    x0 = mul(x0, rdf(m + 16));
                    x2 = add(x2, x0);
                    x0 = rdf(m + 24);
                    x0 = mul(x0, e48);
                    x2 = add(x2, x0);
                    x0 = rdf(m + 32);
                    x0 = mul(x0, e40);
                    x1 = add(x1, x0);
                    x0 = rdf(m + 40);
                    x0 = mul(x0, e48);
                    wr32(edi + T_P0, x6.to_bits());
                    wr32(edi + T_P1, x2.to_bits());
                    x1 = add(x1, x0);
                    wr32(edi + T_P2, x1.to_bits());
                // Block 2 (inside the same guard: the skips jump past it).
                x = rd32(edi + T_X);
            if rd32(x + X_MAT) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_INIT, u32, x);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_MAT, u32, x.wrapping_add(16), rd32(x + X_MAT));
            }
            {
                let m = rd32(x + X_MAT);
                let e50 = rdf(edi + T_Q0);
                let e54 = rdf(edi + T_Q1);
                let e58 = rdf(edi + T_Q2);
                let mut x6 = rdf(m + 4);
                let mut x2 = rdf(m + 20);
                let mut x1 = rdf(m + 36);
                let mut x0 = e50;
                x0 = mul(x0, rdf(m));
                x6 = mul(x6, e54);
                x2 = mul(x2, e54);
                x6 = add(x6, x0);
                x0 = rdf(m + 8);
                x0 = mul(x0, e58);
                x1 = mul(x1, e54);
                x6 = add(x6, x0);
                x0 = rdf(m + 16);
                x0 = mul(x0, e50);
                let x5 = mul(e50, rdf(m + 32));
                x2 = add(x2, x0);
                x0 = rdf(m + 24);
                x0 = mul(x0, e58);
                x1 = add(x1, x5);
                x2 = add(x2, x0);
                x0 = rdf(m + 40);
                x0 = mul(x0, e58);
                wr32(edi + T_Q0, x6.to_bits());
                wr32(edi + T_Q1, x2.to_bits());
                x1 = add(x1, x0);
                wr32(edi + T_Q2, x1.to_bits());
                }
                }
            }
            wr32(esi + A0_MASK, rd32(esi + A0_MASK) & 0xffff_fffe);
            let _: u32 = lf_checker_rt::callee_thiscall!(C_N1, u32, esi, 0, 0xffff_ffff);
            let mut z = [0u32; 3];
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_N2, u32, esi, z.as_mut_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(C_N3, u32, esi, 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_N4, u32, rd32(esi + A0_NOT), 1);
        }
        // Tail region (esi = arg0 on both entries).
        let esi = arg0;
        let l1 = sub(rdf(edi + T_Q0), rdf(edi + T_P0));
        let l0 = sub(rdf(edi + T_Q1), rdf(edi + T_P1));
        let l3 = sub(rdf(edi + T_Q2), rdf(edi + T_P2));
        let mut q = mul(l0, l0);
        q = add(q, mul(l1, l1));
        q = add(q, mul(l3, l3));
        let r = core::hint::black_box(q).sqrt();
        let g40 = globf(G_40);
        let sel: f32;
        if g40 > r {
            let t: u32 = lf_checker_rt::callee_cdecl!(C_RANGE, u32, rd32(edi + T_MODE));
            if (t as u8) == 0 {
                sel = globf(G_3C);
            } else {
                sel = globf(G_38);
            }
        } else {
            sel = globf(G_38);
        }
        let mut cap = mul(globf(G_17), sel);
        if cap < 0.0 {
            // Far region.
            let mut g40w = cap.to_bits();
            let mut g58w = l3.to_bits();
            let _: u32 = lf_checker_rt::callee_stdcall!(
                C_SOLVE, u32, rd32(edi + T_MODE), (&mut g40w as *mut u32) as u32,
                (&mut g58w as *mut u32) as u32);
            let mode2 = rd32(edi + T_MODE);
            let s4 = if mode2 == 0x69 { K1000 } else { K8 };
            let a14: u32 = lf_checker_rt::callee_thiscall!(
                C_ACTION, u32, edi, esi, 9, mode2, s4.to_bits());
            if (a14 as u8) == 0 {
                wr32(edi + T_STATE, 8);
                return a14;
            }
            let child = rd32(edi + T_CHILD);
            let idx = rd32(child + CH_SEL).wrapping_sub(0x69);
            if idx > 0xB {
                wr32(edi + T_STATE, 8);
                return idx;
            }
            let case = SWMAP[idx as usize];
            if case == 4 {
                wr32(edi + T_STATE, 8);
                return case as u32;
            }
            if case == 0 {
                wr32(edi + T_STATE, 2);
            } else if case == 1 {
                wr32(edi + T_STATE, 1);
            } else if case == 2 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_CLIP, u32, child, KCLIP.to_bits());
                wr32(edi + T_STATE, 5);
            } else {
                wr32(edi + T_STATE, 5);
            }
            wr16(child + CH_FL, rd16(child + CH_FL) & 0xfffd);
            let r2: u32 = lf_checker_rt::callee_thiscall!(
                C_FINAL, u32, edi, arg1, arg2, g40w, g58w);
            return r2;
        }
        if cap > r {
            cap = r;
        }
        if !(cap > K001) {
            // Far region (same as above).
            let mut g40w = cap.to_bits();
            let mut g58w = l3.to_bits();
            let _: u32 = lf_checker_rt::callee_stdcall!(
                C_SOLVE, u32, rd32(edi + T_MODE), (&mut g40w as *mut u32) as u32,
                (&mut g58w as *mut u32) as u32);
            let mode2 = rd32(edi + T_MODE);
            let s4 = if mode2 == 0x69 { K1000 } else { K8 };
            let a14: u32 = lf_checker_rt::callee_thiscall!(
                C_ACTION, u32, edi, esi, 9, mode2, s4.to_bits());
            if (a14 as u8) == 0 {
                wr32(edi + T_STATE, 8);
                return a14;
            }
            let child = rd32(edi + T_CHILD);
            let idx = rd32(child + CH_SEL).wrapping_sub(0x69);
            if idx > 0xB {
                wr32(edi + T_STATE, 8);
                return idx;
            }
            let case = SWMAP[idx as usize];
            if case == 4 {
                wr32(edi + T_STATE, 8);
                return case as u32;
            }
            if case == 0 {
                wr32(edi + T_STATE, 2);
            } else if case == 1 {
                wr32(edi + T_STATE, 1);
            } else if case == 2 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_CLIP, u32, child, KCLIP.to_bits());
                wr32(edi + T_STATE, 5);
            } else {
                wr32(edi + T_STATE, 5);
            }
            wr16(child + CH_FL, rd16(child + CH_FL) & 0xfffd);
            let r2: u32 = lf_checker_rt::callee_thiscall!(
                C_FINAL, u32, edi, arg1, arg2, g40w, g58w);
            return r2;
        }
        let stb: u32 = lf_checker_rt::callee_cdecl!(C_ARC, u32, q.to_bits());
        let st = f32::from_bits(stb);
        let mut eax = stb;
        let f10 = mul(l1, st);
        let f28 = mul(l0, st);
        let f38 = mul(l3, st);
        let child = rd32(edi + T_CHILD);
        let mut x1: f32;
        if child != 0 && rd32(child + CH_SEL) != 0x68 {
            let mut f34 = 0u32;
            let a13: u32 = lf_checker_rt::callee_thiscall!(
                C_FIT, u32, child, 0x4000, (&mut f34 as *mut u32) as u32, 0,
                K1.to_bits());
            eax = a13;
            if (a13 as u8) != 0 {
                eax = child;
                let f34f = f32::from_bits(f34);
                if f34f > rdf(child + CH_F) {
                    x1 = 0.0;
                } else {
                    x1 = f38;
                }
            } else {
                x1 = f38;
            }
        } else {
            x1 = f38;
        }
        let mut x2 = mul(f10, cap);
        let mut x3 = mul(f28, cap);
        x2 = add(x2, rdf(edi + T_P0));
        x1 = mul(x1, cap);
        x3 = add(x3, rdf(edi + T_P1));
        x1 = add(x1, rdf(edi + T_P2));
        wr32(edi + T_P0, x2.to_bits());
        wr32(edi + T_P1, x3.to_bits());
        wr32(edi + T_P2, x1.to_bits());
        ((edi + T_FLAGS) as *mut u8).write(rd8(edi + T_FLAGS) | 2);
        if (glob(G_D) as i32) >= 2 {
            let x = rd32(edi + T_X);
            if x != 0 {
                let fl = rd32(x + X_FL);
                let nyb = (fl >> 6) & 0xF;
                eax = nyb;
                if nyb > 1 && nyb < 5 {
                    let masked = fl & 0x3C0;
                    eax = masked;
                    if masked == 0x80 && rd32(x + X_MODE) == 2 && r > K8 {
                        wr32(edi + T_STATE, 8);
                        return 0x80;
                    }
                }
            }
        }
        if child != 0 {
            return eax;
        }
        let s4 = if rd32(edi + T_MODE) == 0x69 { K1000 } else { K8 };
        let a14: u32 = lf_checker_rt::callee_thiscall!(
            C_ACTION, u32, edi, esi, 9, t2c, s4.to_bits());
        if (a14 as u8) == 0 {
            return a14;
        }
        if t2c == 0x6d {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_CLIP, u32, 0, glob(G_44));
        } else if t2c == 0x68 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_CLIP, u32, 0, glob(G_48));
        }
        // 63D8 null selector read: faults identically on both sides.
        // black_box forces the read (a dead read + unreachable_unchecked
        // optimizes to ud2, which faults with the wrong code).
        core::hint::black_box(rd32(0x0C));
        0
    }
});
