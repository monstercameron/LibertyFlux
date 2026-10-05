// original: 0x00cf57c0 ped_task_aim (proposed)

/// Aim-step of a ped task: pick a firing solution and advance the state.
///
/// `this` is the task object, `arg0` a related object, `arg1` an opaque word
/// carried to callees and `arg2` a float (bit pattern) likewise carried.
/// Original convention: thiscall with three stack words (the callee pops 0xc bytes).
///
/// The task reads a mode word at `+0x84`, a flag byte at `+0x89`, a state
/// word at `+0x14`, a child pointer at `+0x10` and opaque words at `+0x64`
/// and `+0x6c` (the latter cleared on entry); it writes a solution float at
/// `+0x70`. `arg0` carries a flag word at `+0xbe0` (bit 5 set on entry) and a
/// pointer at `+0x20` to fifteen floats consumed below. The child (when
/// present) exposes a selector word at `+0x0c` and takes a constant at
/// `+0x54` on the success path.
///
/// Behaviour. A predicate callee is asked about `arg0` twice (modes 0 and
/// 1); a pass on the first ask, or mode 3, diverts to a setup callee and the
/// shared tail. Otherwise a gate/classify pair runs; if the classifier
/// passes, an action callee runs with code 0x72, a final callee runs with
/// (`arg1`, `arg2`, 0.4, -1.0) and the state becomes 5. If any of that fails
/// the slow section runs: two more predicate asks (modes 2 and 3) choose
/// between two aim branches (or the tail when both fail and the mode is not
/// 2). Each branch scales the fifteen floats (by a tuned global, 0.8 and
/// +0.25 or -0.25), records a selector (0x7c or 0x7b) and a state (4 or 3),
/// and stores a solution (a global quotient, or 1.0). The merge adds the
/// scaled groups, scales by 10, adds three more floats plus 1.0, and hands
/// the resulting triple to a solver callee together with constants; a fit
/// callee then runs with (`arg0`, `arg1`, `arg2`, a branch float, three
/// zero slots, 0). If the fit passes, the state becomes the recorded one
/// and, when a child is present whose selector matches, a measure callee's
/// float is compared against 0: a non-negative result runs a clip callee and
/// continues to a maybe callee, whose pass writes 1.5 to the child and
/// returns the child, while anything else runs the action callee with the
/// recorded selector and 8.0 and then, unless the child is absent, the maybe
/// callee once. The shared tail runs the action callee with code 0x69 when
/// the state is 2 and no child is present. Every path returns its last
/// callee's full `eax` (or the child pointer on the success path).
///
/// Edge cases: all float work is single-precision in the original's order
/// (pinned with `black_box`); only low bytes of answers drive branches; the
/// global quotient may divide by zero (single-precision infinity, no fault);
/// the solver/fit frame pointers aim below the stack pointer so their
/// addresses are skipped while their contents are snapshotted (the triple
/// this also verifies the float chains bit-exactly, since nothing else
/// observes them); the three zero slots are unwritten scratch (zero under
/// the contract's stack fill) that both sides must agree on.
lf_checker_rt::export!(thiscall, rw_00cf57c0(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const C_PREDA: u32 = 1;
        const C_PREDB: u32 = 2;
        const C_PREDC: u32 = 3;
        const C_PREDD: u32 = 4;
        const C_GATE: u32 = 5;
        const C_CLASSIFY: u32 = 6;
        const C_ACTION: u32 = 7;
        const C_FINAL: u32 = 8;
        const C_SOLVE: u32 = 9;
        const C_FIT: u32 = 10;
        const C_MEASURE: u32 = 11;
        const C_CLIP: u32 = 12;
        const C_MAYBE: u32 = 13;
        const C_SETUP: u32 = 14;

        const T_CHILD: u32 = 0x10;
        const T_STATE: u32 = 0x14;
        const T_GATE_ARG: u32 = 0x64;
        const T_CLEAR: u32 = 0x6c;
        const T_SOL: u32 = 0x70;
        const T_MODE: u32 = 0x84;
        const T_FLAGS: u32 = 0x89;
        const A0_FLAG: u32 = 0xbe0;
        const A0_VEC: u32 = 0x20;
        const A0_SOLVE_OBJ: u32 = 0xbb0;
        const CH_SEL: u32 = 0x0c;
        const CH_OUT: u32 = 0x54;
        const G20: u32 = 0x0105_3920;
        const G24: u32 = 0x0105_3924;
        const G28: u32 = 0x0105_3928;

        const K80: f32 = f32::from_bits(0x3f4c_cccd); // 0.8
        const K25: f32 = f32::from_bits(0x3e80_0000); // 0.25
        const KN25: f32 = f32::from_bits(0xbe80_0000); // -0.25
        const K10: f32 = f32::from_bits(0x4120_0000); // 10.0
        const K1: f32 = f32::from_bits(0x3f80_0000); // 1.0
        const K01: f32 = f32::from_bits(0x3dcc_cccd); // 0.1
        const KN01: f32 = f32::from_bits(0xbdcc_cccd); // -0.1
        const K40: f32 = f32::from_bits(0x3ecc_cccd); // 0.4
        const KN1: f32 = f32::from_bits(0xbf80_0000); // -1.0
        const K8: f32 = f32::from_bits(0x4100_0000); // 8.0
        const K15: f32 = f32::from_bits(0x3fc0_0000); // 1.5
        const RATE: u32 = 0x447a_0000; // 1000.0

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(va) as *const u32).read_unaligned()) }
        }

        let esi = this;
        let edi = arg0;
        wr32(edi + A0_FLAG, rd32(edi + A0_FLAG) | 0x20);
        wr32(esi + T_CLEAR, 0);
        let pa: u32 = lf_checker_rt::callee_stdcall!(C_PREDA, u32, edi, 0);
        if !((pa as u8) == 0 && rd32(esi + T_MODE) != 3) {
            wr32(esi + T_MODE, 0);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                C_SETUP, u32, esi, edi, arg1, arg2);
            if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                return r2;
            }
            return r;
        }
        let pb: u32 = lf_checker_rt::callee_stdcall!(C_PREDB, u32, edi, 1);
        if (pb as u8) != 0 || rd32(esi + T_MODE) == 4 || rd8(esi + T_FLAGS) & 1 != 0 {
            let g: u32 = lf_checker_rt::callee_cdecl!(
                C_GATE, u32, edi, rd32(esi + T_GATE_ARG));
            let c: u32 = lf_checker_rt::callee_cdecl!(
                C_CLASSIFY, u32, edi, arg1, arg2, 1, (g & 0xff));
            if (c as u8) != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, 0x72, RATE);
                wr32(esi + T_MODE, 0);
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    C_FINAL, u32, esi, arg1, arg2, K40.to_bits(), KN1.to_bits());
                wr32(esi + T_STATE, 5);
                return r;
            }
        }
        // Slow section.
        let mut f10 = K01;
        let pc: u32 = lf_checker_rt::callee_stdcall!(C_PREDC, u32, edi, 2);
        let branch1 = (pc as u8) != 0 || rd32(esi + T_MODE) == 1;
        let (t0c, t14): (u32, u32);
        if !branch1 {
            let pd: u32 = lf_checker_rt::callee_stdcall!(C_PREDD, u32, edi, 3);
            if (pd as u8) == 0 && rd32(esi + T_MODE) != 2 {
                if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                    let r2: u32 = lf_checker_rt::callee_thiscall!(
                        C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                    return r2;
                }
                return pd;
            }
            t0c = 0x7c;
            t14 = 4;
            let base = rd32(edi + A0_VEC);
            let g28 = glob(G28);
            let mut x4 = rdf(base + 16);
            x4 = mul(x4, g28);
            x4 = mul(x4, K80);
            let mut x5 = rdf(base + 20);
            x5 = mul(x5, g28);
            x5 = mul(x5, K80);
            let mut x6 = rdf(base + 24);
            x6 = mul(x6, g28);
            x6 = mul(x6, K80);
            let mut x3 = rdf(base);
            x3 = mul(x3, K25);
            let mut x1 = rdf(base + 4);
            x1 = mul(x1, K25);
            let mut x2 = rdf(base + 8);
            x2 = mul(x2, K25);
            wr32(esi + T_SOL, div(glob(G20), glob(G24)).to_bits());
            x4 = add(x4, x3);
            x5 = add(x5, x1);
            x6 = add(x6, x2);
            wr32(esi + T_MODE, 0);
            let base2 = rd32(edi + A0_VEC);
            x4 = mul(x4, K10);
            x1 = rdf(base2 + 56);
            x4 = add(x4, rdf(base2 + 48));
            x6 = mul(x6, K10);
            x5 = mul(x5, K10);
            let mut x0 = rdf(base2 + 52);
            x1 = add(x1, x6);
            x0 = add(x0, x5);
            x1 = add(x1, K1);
            let mut triple = [x4.to_bits(), x0.to_bits(), x1.to_bits()];
            let sptr = triple.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_SOLVE, u32, edi.wrapping_add(A0_SOLVE_OBJ), lf_checker_rt::relocated(0x00edefd8), 0, 0,
                0x7d0, 0xffff_ffff, sptr, 0, 0x1f4, 0x1f4, 1);
        } else {
            t0c = 0x7b;
            t14 = 3;
            f10 = KN01;
            let base = rd32(edi + A0_VEC);
            let g28 = glob(G28);
            let mut x4 = rdf(base + 16);
            x4 = mul(x4, g28);
            x4 = mul(x4, K80);
            let mut x5 = rdf(base + 20);
            x5 = mul(x5, g28);
            x5 = mul(x5, K80);
            let mut x6 = rdf(base + 24);
            x6 = mul(x6, g28);
            x6 = mul(x6, K80);
            let mut x3 = rdf(base);
            x3 = mul(x3, KN25);
            let mut x1 = rdf(base + 4);
            x1 = mul(x1, KN25);
            let mut x2 = rdf(base + 8);
            x2 = mul(x2, KN25);
            wr32(esi + T_SOL, K1.to_bits());
            x4 = add(x4, x3);
            x5 = add(x5, x1);
            x6 = add(x6, x2);
            wr32(esi + T_MODE, 0);
            let base2 = rd32(edi + A0_VEC);
            x4 = mul(x4, K10);
            x1 = rdf(base2 + 56);
            x4 = add(x4, rdf(base2 + 48));
            x6 = mul(x6, K10);
            x5 = mul(x5, K10);
            let mut x0 = rdf(base2 + 52);
            x1 = add(x1, x6);
            x0 = add(x0, x5);
            x1 = add(x1, K1);
            let mut triple = [x4.to_bits(), x0.to_bits(), x1.to_bits()];
            let sptr = triple.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_SOLVE, u32, edi.wrapping_add(A0_SOLVE_OBJ), lf_checker_rt::relocated(0x00edefd8), 0, 0,
                0x7d0, 0xffff_ffff, sptr, 0, 0x1f4, 0x1f4, 1);
        }
        let mut z18 = [0u32; 2];
        let mut z30 = 0u32;
        let p18 = z18.as_mut_ptr() as u32;
        let p1c = (z18.as_mut_ptr() as u32).wrapping_add(4);
        let p30 = (&mut z30 as *mut u32) as u32;
        let f: u32 = lf_checker_rt::callee_thiscall!(
            C_FIT, u32, esi, edi, arg1, arg2, f10.to_bits(), p30, p1c, p18, 0);
        if (f as u8) == 0 {
            if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                return r2;
            }
            return f;
        }
        let child = rd32(esi + T_CHILD);
        wr32(esi + T_STATE, t14);
        if child != 0 && rd32(child + CH_SEL) == t0c {
            let st0: f32 = lf_checker_rt::callee_thiscall!(C_MEASURE, f32, child);
            if !(st0 < 0.0) {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_CLIP, u32, child, 0);
                let m: u32 = lf_checker_rt::callee_thiscall!(C_MAYBE, u32, edi);
                if (m as u8) == 0 {
                    if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                        let r2: u32 = lf_checker_rt::callee_thiscall!(
                            C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                        return r2;
                    }
                    return m;
                }
                wr32(child + CH_OUT, K15.to_bits());
                if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                    let r2: u32 = lf_checker_rt::callee_thiscall!(
                        C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                    return r2;
                }
                return child;
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_ACTION, u32, esi, edi, 9, t0c, K8.to_bits());
        // 5B18 re-reads the child into eax, clobbering the answer above.
        let c2 = rd32(esi + T_CHILD);
        if c2 == 0 {
            if rd32(esi + T_STATE) == 2 {
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                return r2;
            }
            return c2;
        }
        let m: u32 = lf_checker_rt::callee_thiscall!(C_MAYBE, u32, edi);
        if (m as u8) == 0 {
            if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, 0x69, RATE);
                return r2;
            }
            return m;
        }
        wr32(child + CH_OUT, K15.to_bits());
        if rd32(esi + T_STATE) == 2 && rd32(esi + T_CHILD) == 0 {
            let r2: u32 = lf_checker_rt::callee_thiscall!(
                C_ACTION, u32, esi, edi, 9, 0x69, RATE);
            return r2;
        }
        child
    }
});
