// original: 0x00a74b70 ped_task_update_float_state (proposed)

/// Per-frame float-state update for a ped task object, driven by a 16-bit
/// state field and a 3-vector scratch triple.
///
/// `obj` (this) holds the state word at `+0x32`, a phase value at `+0x3c`
/// and a rate value at `+0x4c`. `arg0` is a larger record: vtable at `+0x0`,
/// a float matrix at `+0x20`, two link words at `+0x38`/`+0x7b4`, a callee
/// target at `+0x78`, two direction floats at `+0xe0`/`+0xe4` and a mode
/// word at `+0x144`. `dt_bits` is a time-step float.
///
/// Behaviour: dispatch on the state word through a 7-entry table (states
/// 0-3 share the first arm, 4 the second, 5 the third, 6 returns `obj`
/// at once, anything else runs the default path). Each arm seeds a scratch
/// triple from the matrix and the direction floats, consulting scripted
/// lookup callees. The triple is then differenced against the result of a
/// vtable-slot evaluation call, clamped to a step-scaled length, re-based
/// through a second evaluation call and a 10-argument parameter call, and
/// (only in state 5 with a positive answer) used to advance the phase and
/// rate fields. A final gate on an accumulator (seeded at -1) selects one
/// of two convergence tails, each clamping the triple through further
/// evaluation calls. The tail returns the link word when both links agree,
/// otherwise it scales the triple by a vtable-slot factor over `dt` and
/// hands it to a sink call, returning that call's answer.
///
/// Slot note: several stores use `[esp+X]` while a pushed argument is still
/// on the stack, so they land 4 bytes below the written offset; the first
/// arm's blend factor lands in the accumulator slot, its pair in the first
/// two triple slots, and the first lookup answer in the slot its later load
/// reads. There are no uninitialized reads: every live slot is stored on
/// every path that reads it, and the remaining slots read the checker's
/// zero stack fill, reproduced here as plain zero initialisers. The flag
/// byte tested before the tails is zeroed and never written, so it is
/// always clear unless the probe-call branch sets it. All float operation
/// orders are the original's, pinned through `black_box` helpers; NaN
/// branch behaviour follows the original's unsigned flag tests exactly.
///
/// Original: 0x00a74b70 (thiscall, two stack words: record pointer, float).
lf_checker_rt::export!(thiscall, rw_00a74b70(obj: u32, arg0: u32, dt_bits: u32) -> u32 {
    unsafe {
        const CAL_B55: u32 = 1;
        const CAL_F1: u32 = 2;
        const CAL_F2: u32 = 3;
        const CAL_AD: u32 = 4;
        const CAL_A71: u32 = 5;
        const CAL_A32: u32 = 6;
        const CAL_S8: u32 = 7;
        const CAL_VC: u32 = 8;
        const CAL_B23: u32 = 9;

        const OBJ_STATE: u32 = 0x32;
        const OBJ_PHASE: u32 = 0x3c;
        const OBJ_RATE: u32 = 0x4c;
        const A_MAT: u32 = 0x20;
        const A_LINK: u32 = 0x38;
        const A_LOOKUP: u32 = 0x78;
        const A_DIR0: u32 = 0xe0;
        const A_DIR1: u32 = 0xe4;
        const A_MODE: u32 = 0x144;
        const A_LINK2: u32 = 0x7b4;
        const VT_EVAL: u32 = 0xec;
        const VT_FACTOR: u32 = 0x24;
        const RET_OFF: u32 = 0x58;
        const RET_OFF_B: u32 = 0x4c;
        const A71_FLAG: u32 = 0x1304;
        const A71_INNER: u32 = 0x20;
        const A71_VALUE: u32 = 0x28;

        const K_ACC_INIT: f32 = f32::from_bits(0xbf800000); // -1.0
        const K_ONE: f32 = 1.0;
        const K_HALF: f32 = 0.5;
        const K_M0P1: f32 = f32::from_bits(0xbdcccccd); // -0.1
        const K_50: f32 = 50.0;
        const K_0P05: f32 = f32::from_bits(0x3d4ccccd); // 0.05
        const K_900: f32 = 900.0;
        const K_30: f32 = 30.0;
        const K_PI4: f32 = f32::from_bits(0x3f490fdb); // pi/4
        const K_0P1: f32 = f32::from_bits(0x3dcccccd); // 0.1
        const K_0P001: f32 = f32::from_bits(0x3a83126f); // 0.001
        const K_0P95: f32 = f32::from_bits(0x3f733333); // 0.95
        const K_0P025: f32 = f32::from_bits(0x3ccccccd); // 0.025
        const K_4DIVPI: f32 = f32::from_bits(0x3fa2f983); // 4/pi
        const K_0P75: f32 = 0.75;
        const K_M0P33: f32 = f32::from_bits(0xbea8f5c3); // -0.33
        const K_2: f32 = 2.0;
        const K_10: f32 = 10.0;
        const K_M0P5: f32 = -0.5;
        const K_6: u32 = 0x40c00000; // 6.0f bits
        const K_20: u32 = 0x41a00000; // 20.0f bits
        const K_1P75: u32 = 0x3fe00000; // 1.75f bits
        const SIGN: u32 = 0x8000_0000;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        /// The shared vtable-slot evaluation call: `arg0`'s table slot
        /// `VT_EVAL`, this is `arg0`, one out-pointer argument.
        #[inline(always)]
        unsafe fn vcall(arg0: u32, out: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(arg0) + VT_EVAL) as usize);
                f(arg0, out)
            }
        }
        /// One of the two float unary callees (input and output in XMM0 on
        /// the original side; the stub also answers in EAX, which is how the
        /// rewrite observes the same scripted value).
        #[inline(always)]
        unsafe fn fcall(id: u32, x: f32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::callee_cdecl!(id, u32, x.to_bits())) }
        }

        let dt = f32::from_bits(dt_bits);
        let state = rd16(obj + OBJ_STATE) as u16 as i16 as i32;
        let mut scratch = [0u32; 4];
        let scratch_ptr = scratch.as_mut_ptr() as u32;

        let mut s10 = 0.0f32;
        let mut s24 = 0.0f32;
        let mut s28 = 0.0f32;
        // Fill-zero stand-ins: the original leaves these slots unstored on
        // some switch paths, so under the contract's zero stack fill they
        // read 0.0 there.
        let mut s38 = 0.0f32;
        let mut s48 = 0.0f32;
        let mut acc = K_ACC_INIT;

        // The bound check is unsigned, so negative states take the default.
        let case = if (state as u32) > 6 { 7 } else { state };
        match case {
            0 | 1 | 2 | 3 => {
                let r1 =
                    lf_checker_rt::callee_thiscall!(CAL_B55, u32, rd32(arg0 + A_LOOKUP), 0x28u32);
                let r2 =
                    lf_checker_rt::callee_thiscall!(CAL_B55, u32, rd32(arg0 + A_LOOKUP), 0x1cu32);
                let r3 =
                    lf_checker_rt::callee_thiscall!(CAL_B55, u32, rd32(arg0 + A_LOOKUP), 0x32u32);
                s48 = f32::from_bits(r1);
                let mut x2 = K_ONE;
                let mut x4 = 0.0f32;
                if r1 != 0 {
                    x4 = rdf(r1 + RET_OFF);
                    x2 = sub(x2, rdf(r1 + RET_OFF));
                    x4 = mul(x4, K_HALF);
                }
                if r2 != 0 {
                    let t = rdf(r2 + RET_OFF);
                    x2 = sub(x2, t);
                    x4 = add(x4, mul(t, K_HALF));
                }
                if r3 != 0 {
                    let t = rdf(r3 + RET_OFF);
                    x2 = sub(x2, t);
                    x4 = add(x4, mul(t, K_HALF));
                }
                if 0.0 > x2 {
                    x2 = 0.0;
                }
                let mat = rd32(arg0 + A_MAT);
                let f_e0 = rdf(arg0 + A_DIR0);
                let mut x3 = rdf(mat + 0x10);
                x2 = mul(x2, K_HALF);
                let x1 = mul(f_e0, rdf(mat));
                x2 = add(x2, x4);
                x4 = rdf(mat + 0x14);
                let mut x2b = rdf(mat + 4);
                x2b = mul(x2b, f_e0);
                let f_e4 = rdf(arg0 + A_DIR1);
                x3 = mul(x3, f_e4);
                x4 = mul(x4, f_e4);
                x3 = add(x3, x1);
                x4 = add(x4, x2b);
                acc = x2;
                s10 = x3;
                s24 = x4;
                let ret1 = vcall(arg0, scratch_ptr);
                s28 = rdf(ret1 + 8);
            }
            4 => {
                let mat = rd32(arg0 + A_MAT);
                let f_e0 = rdf(arg0 + A_DIR0);
                let mut x2 = rdf(mat + 4);
                let mut x3 = rdf(mat + 8);
                let mut x5 = rdf(mat + 0x10);
                let mut x6 = rdf(mat + 0x14);
                let mut x4 = rdf(mat + 0x18);
                let x1 = mul(f_e0, rdf(mat));
                x2 = mul(x2, f_e0);
                x3 = mul(x3, f_e0);
                let f_e4 = rdf(arg0 + A_DIR1);
                x5 = mul(x5, f_e4);
                x6 = mul(x6, f_e4);
                x4 = mul(x4, f_e4);
                x5 = add(x5, x1);
                x6 = add(x6, x2);
                x4 = add(x4, x3);
                s10 = x5;
                s24 = x6;
                s28 = x4;
                let r =
                    lf_checker_rt::callee_thiscall!(CAL_B55, u32, rd32(arg0 + A_LOOKUP), 0x90u32);
                if r != 0 {
                    let mut jx = rdf(r + RET_OFF_B);
                    jx = mul(jx, K_M0P1);
                    jx = mul(jx, K_50);
                    jx = mul(jx, dt);
                    s28 = jx;
                }
            }
            5 => {
                let f_e0 = rdf(arg0 + A_DIR0);
                let mat = rd32(arg0 + A_MAT);
                s10 = mul(f_e0, rdf(mat));
                s24 = mul(rdf(mat + 4), f_e0);
                s38 = mul(rdf(mat + 8), f_e0);
                let t1 = fcall(CAL_F1, rdf(obj + OBJ_PHASE));
                let mut x2 = rdf(mat + 0x14);
                let mut x3 = rdf(mat + 0x18);
                let mut x1 = mul(t1, rdf(mat + 0x10));
                x2 = mul(x2, t1);
                x3 = mul(x3, t1);
                let f_e4 = rdf(arg0 + A_DIR1);
                x1 = mul(x1, f_e4);
                x3 = mul(x3, f_e4);
                x2 = mul(x2, f_e4);
                s48 = x3;
                s10 = add(s10, x1);
                s24 = add(s24, x2);
                let t2 = fcall(CAL_F2, rdf(obj + OBJ_PHASE));
                let mut x1b = s48;
                let mut jx = mul(t2, f_e4);
                x1b = add(x1b, s38);
                jx = add(jx, x1b);
                jx = add(jx, K_0P05);
                s28 = jx;
            }
            6 => return obj,
            _ => {}
        }

        // Difference the triple against a fresh evaluation.
        let ret2 = vcall(arg0, scratch_ptr);
        let d3 = sub(s10, rdf(ret2));
        let d4 = sub(s24, rdf(ret2 + 4));
        let d2 = sub(s28, rdf(ret2 + 8));
        s10 = d3;
        s24 = d4;
        s28 = d2;
        let q3 = mul(d3, d3);
        let mut len2 = mul(d4, d4);
        len2 = add(len2, q3);
        len2 = add(len2, mul(d2, d2));
        if len2 > mul(mul(dt, K_900), dt) {
            let k = div(mul(dt, K_30), len2.sqrt());
            s10 = mul(k, d3);
            s28 = mul(k, d2);
            s24 = mul(k, d4);
        }

        let ret3 = vcall(arg0, scratch_ptr);
        let mut a0 = add(rdf(ret3), s10);
        let mut a1 = add(rdf(ret3 + 4), s24);
        let mat = rd32(arg0 + A_MAT);
        let mut x2 = rdf(mat + 0x34);
        a0 = mul(a0, dt);
        a1 = mul(a1, dt);
        let mut x3 = rdf(mat + 0x30);
        x3 = add(x3, a0);
        x2 = add(x2, a1);
        let m38 = rdf(mat + 0x38);
        // The call setup zeroes this frame slot just before the call, so the
        // callee always sees 0.0 here (the case-C value above is overwritten
        // before any read) and its out-word becomes the live value.
        s38 = 0.0;
        let answer: u32 = {
            let frame_slot = &mut s38 as *mut f32 as u32;
            lf_checker_rt::callee_cdecl!(
                CAL_AD, u32,
                x3.to_bits(), x2.to_bits(), m38.to_bits(), frame_slot,
                1u32, 0u32, 0u32, K_6, K_20, 0u32
            )
        };
        // The callee's out-word lands in our frame; reload it.
        s38 = unsafe { (&mut s38 as *mut f32).read_volatile() };

        if (answer as u8) != 0 {
            if rd16(obj + OBJ_STATE) == 5 {
                let f4c = rdf(obj + OBJ_RATE);
                s48 = f4c;
                if !(0.0 > f4c) {
                    let mat2 = rd32(arg0 + A_MAT);
                    let x2q = add(rdf(mat2 + 0x38), K_HALF);
                    let mut reset = false;
                    if x2q > s38 {
                        if rdf(obj + OBJ_PHASE) > K_PI4 {
                            wr16(obj + OBJ_STATE, 0);
                            wr32(obj + OBJ_RATE, 0);
                            reset = true;
                        }
                    }
                    if !reset {
                        let x5 = rdf(obj + OBJ_PHASE);
                        if 0.0 > x5 {
                            let t = fcall(CAL_F2, x5);
                            let mut x1 = sub(rdf(mat2 + 0x38), t);
                            x1 = add(x1, K_HALF);
                            if x1 > s38 {
                                let mut xa = mul(dt, K_0P1);
                                xa = add(xa, s48);
                                wrf(obj + OBJ_RATE, xa);
                                if xa > K_0P05 {
                                    wr32(obj + OBJ_RATE, K_0P05.to_bits());
                                }
                            } else if s48 > K_0P001 {
                                wrf(obj + OBJ_RATE, mul(s48, K_0P95));
                            } else {
                                wr32(obj + OBJ_RATE, 0);
                            }
                            let mut xb = rdf(obj + OBJ_RATE);
                            xb = mul(xb, K_50);
                            xb = mul(xb, dt);
                            xb = add(xb, rdf(obj + OBJ_PHASE));
                            wrf(obj + OBJ_PHASE, xb);
                        } else if x2q > s38 {
                            if s48 > K_0P025 {
                                wrf(obj + OBJ_RATE, mul(s48, K_0P95));
                            }
                            if K_0P025 > rdf(obj + OBJ_RATE) {
                                let mut xa = mul(dt, K_0P1);
                                xa = add(xa, rdf(obj + OBJ_RATE));
                                wrf(obj + OBJ_RATE, if xa > K_0P025 { K_0P025 } else { xa });
                            }
                            let mut xb = rdf(obj + OBJ_RATE);
                            xb = mul(xb, K_50);
                            xb = mul(xb, dt);
                            xb = add(xb, rdf(obj + OBJ_PHASE));
                            wrf(obj + OBJ_PHASE, xb);
                            xb = mul(xb, K_4DIVPI);
                            xb = mul(xb, K_0P75);
                            xb = mul(xb, 0.0);
                            xb = add(xb, K_HALF);
                            acc = xb;
                        } else if s48 > K_0P001 {
                            wrf(obj + OBJ_RATE, mul(s48, K_0P95));
                        } else {
                            wr32(obj + OBJ_RATE, 0);
                        }
                    }
                }
            }

            let mut flag: u8 = 0;
            let probe = lf_checker_rt::callee_thiscall!(CAL_A71, u32, obj, arg0, K_1P75);
            if probe != 0 && rd32(probe + A71_FLAG) == 2 && K_M0P33 > rdf(rd32(probe + A71_INNER) + A71_VALUE) {
                acc = add(acc, K_ONE);
                flag = 1;
            }
            if acc > 0.0 {
                let mode = rd32(arg0 + A_MODE);
                if mode == 1 || flag != 0 {
                    // First convergence tail.
                    let mat = rd32(arg0 + A_MAT);
                    let xc = add(rdf(mat + 0x38), acc);
                    let x1 = sub(s38, xc);
                    s38 = x1;
                    s48 = mul(x1, K_10);
                    let ret7 = vcall(arg0, scratch_ptr);
                    let xp = sub(s48, rdf(ret7 + 8));
                    if xp > K_HALF {
                        s28 = xp;
                    } else if K_M0P5 > xp {
                        s28 = K_M0P5;
                    } else {
                        s28 = xp;
                    }
                } else if mode == 2 {
                    // Second convergence tail.
                    let mat = rd32(arg0 + A_MAT);
                    let mut xp = add(s38, acc);
                    xp = sub(xp, rdf(mat + 0x38));
                    if xp > K_2 {
                        let ret4 = vcall(arg0, scratch_ptr);
                        let x1 = sub(K_10, rdf(ret4 + 8));
                        if x1 > K_HALF {
                            s28 = K_HALF;
                        } else {
                            s28 = x1;
                            if K_M0P5 > x1 {
                                s28 = K_M0P5;
                            }
                        }
                        let mut y0 = sub(s38, acc);
                        s38 = y0;
                        y0 = sub(y0, rdf(mat + 0x38));
                        if y0 > K_50 {
                            let ret5 = vcall(arg0, scratch_ptr);
                            s10 = neg(rdf(ret5));
                            let ret6 = vcall(arg0, scratch_ptr);
                            let nx3 = neg(rdf(ret6 + 4));
                            s24 = K_HALF;
                            if s10 > K_HALF {
                                s10 = K_HALF;
                            } else if K_M0P5 > s10 {
                                s10 = K_M0P5;
                            }
                            if !(nx3 > K_HALF) {
                                s24 = nx3;
                                if K_M0P5 > nx3 {
                                    s24 = K_M0P5;
                                }
                            }
                        }
                    }
                }
            }
        }

        let link = rd32(arg0 + A_LINK);
        if link != 0 && link == rd32(arg0 + A_LINK2) {
            return link;
        }
        let inner = lf_checker_rt::callee_thiscall!(CAL_A32, u32, arg0);
        let factor: f32 = unsafe {
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(rd32(rd32(inner) + VT_FACTOR) as usize);
            f(inner)
        };
        let x3 = div(K_ONE, dt);
        let x1 = mul(factor, s10);
        let x2f = mul(factor, s24);
        let x4 = mul(factor, s28);
        let mut out = [
            mul(x3, x1).to_bits(),
            mul(x3, x2f).to_bits(),
            mul(x3, x4).to_bits(),
            0u32,
        ];
        let out_ptr = out.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_B23, u32, arg0, out_ptr)
    }
});
