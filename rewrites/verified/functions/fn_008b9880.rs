// original: 0x008B9880 float_banks_periodic_tick (proposed)

use lf_checker_rt as rt;

/// Periodic tick over two banks of global float state, driven by one flag argument.
///
/// `flag` (only its low byte is read) selects the slow path: when nonzero, an
/// index/count global is stepped up or down by comparing two flag bytes of a
/// shared object against 0x7f, and a clamped float is derived; otherwise the
/// clamp inputs come from constants. Either way the function then folds a
/// scaled counter (`G_COUNT * 1000`, truncated to i64 the way x87 `fistp`
/// truncates, low 32 bits read back unsigned) into the first bank, refreshes
/// the second bank from a shadow copy when two state-bit tests pass (otherwise
/// recomputing it from a polled object pair), applies per-lane adjustments
/// scaled by two polled signed amounts and a computed divisor, clamps both
/// lanes into moving windows, and finally min-folds four more polled floats
/// (two negated) into the bank.
///
/// All polled values arrive through scripted callees; all persistent state
/// lives in globals. The original keeps 14 words of scratch in its frame;
/// two call sites pass pointers to that scratch, so this rewrite mirrors the
/// scratch in `fr` and passes pointers into it. Control-word save/restore
/// around the truncations touches only dead scratch and is not mirrored.
/// The stack-cookie check is a preserving call with no observed arguments.
/// The indexed table select can read the stack argument (index 8) and dead
/// scratch (negative indices); indices 6/7 (cookie/return slot) are excluded
/// by the contract.
///
/// Original: cdecl, one stack word, returns the last callee answer observed
/// (the final preserving calls pass it through).
/// Shared body for [`rw_008B9880`] and its deliberately-wrong twin: with
/// `MUTANT` set, the flag-byte branch that selects the slow path is inverted.
unsafe fn rw_008B9880_inner<const MUTANT: bool>(flag: u32) -> u32 {
    unsafe {
        // Globals (file VAs).
        const G09CC: u32 = 0x11609CC; // bank0 accumulator (f32)
        const G09D0: u32 = 0x11609D0; // bank0 cap (f32)
        const G09D4: u32 = 0x11609D4; // bank0 armed flag (byte)
        const G09D8: u32 = 0x11609D8; // bank0 index (i32)
        const G359C: u32 = 0x117359C; // counter scale input (f32)
        const G7A80: u32 = 0x18B7A80; // state sample 0 (i32)
        const G7A84: u32 = 0x18B7A84; // state bits 0 (u32)
        const G7A88: u32 = 0x18B7A88; // state bits 1 (u32)
        const G7A8C: u32 = 0x18B7A8C; // state sample 1 (i32)
        const GCCE8: u32 = 0x17ACCE8; // sample0 scale (f32)
        const GCCF0: u32 = 0x17ACCF0; // sample1 scale (f32)
        const G1500: u32 = 0x1161500; // bank1 lane0 (f32)
        const G1504: u32 = 0x1161504; // bank1 lane1 (f32)
        const G1508: u32 = 0x1161508; // bank1 lane0 base (f32)
        const G150C: u32 = 0x116150C; // bank1 lane1 base (f32)
        const G1810: u32 = 0x1161810; // shadow lane0 base (f32)
        const G1814: u32 = 0x1161814; // shadow lane1 base (f32)
        const G1824: u32 = 0x1161824; // shadow lane0 (f32)
        const G1828: u32 = 0x1161828; // shadow lane1 (f32)
        // Float constants (rdata bits).
        const TABLE: [u32; 6] = [0x45CC6000, 0x45A47800, 0x45792000, 0x45295000, 0x4492E000, 0x44228000];
        const K_ONE: f32 = f32::from_bits(0x3F800000);
        const K_2P5: f32 = f32::from_bits(0x40200000);
        const K_1000: f32 = f32::from_bits(0x447A0000);
        const K_HI: f32 = f32::from_bits(0x45792000);
        const K_LO: f32 = f32::from_bits(0x44228000);
        const K_CLAMP_HI: f32 = f32::from_bits(0x45CC6000);
        const K_SIGN: u32 = 0x80000000;
        const K_MAG8M: f32 = f32::from_bits(0x4B000000);
        const K_EPS: f32 = f32::from_bits(0x3920552D);
        const K_BLEND: f32 = f32::from_bits(0x3E99999A);
        const K_STEP: f32 = f32::from_bits(0x3C000000);
        const K_WIN: f32 = f32::from_bits(0x453B8000);
        const TWO63F: f32 = f32::from_bits(0x5F000000);
        // Callee ids.
        const C_FLAG: u32 = 1;
        const C_FLAG_CLR: u32 = 2;
        const C_POLL0: u32 = 3;
        const C_THIS1: u32 = 4;
        const C_OBJ: u32 = 5;
        const C_PAIR: u32 = 6;
        const C_GATE: u32 = 7;
        const C_DER0: u32 = 8;
        const C_AMT: u32 = 9;
        const C_DER1: u32 = 10;
        const C_SMP: u32 = 11;
        const C_PAIR2: u32 = 12;
        const C_FIN0: u32 = 13;
        const C_FIN1: u32 = 14;
        const C_COOKIE: u32 = 15;
        const THIS_OBJ: u32 = 0x1177A80;
        const SMP_P1: u32 = 0x1161810;
        const SMP_P2: u32 = 0x1190E70;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn grd(a: u32) -> u32 {
            unsafe { rd32(rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn grf(a: u32) -> f32 {
            unsafe { f32::from_bits(grd(a)) }
        }
        #[inline(always)]
        unsafe fn gwr(a: u32, v: u32) {
            unsafe { (rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gwf(a: u32, v: f32) {
            unsafe { gwr(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gr8(a: u32) -> u8 {
            unsafe { rd8(rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn gw8(a: u32, v: u8) {
            unsafe { (rt::relocated(a) as *mut u8).write(v) }
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
        /// x87 `fistp` of an f32 with truncate rounding: invalid (NaN, out
        /// of i64 range) yields the indefinite 0x8000...0.
        #[inline(always)]
        fn trunc_to_i64(x: f32) -> i64 {
            if x.is_nan() || x >= TWO63F || x < -TWO63F {
                i64::MIN
            } else {
                x.trunc() as i64
            }
        }
        /// `comiss a, b` + `jbe`: a <= b, or either unordered.
        #[inline(always)]
        fn jbe_ss(a: f32, b: f32) -> bool {
            !(a > b)
        }
        /// `comiss a, b` + `ja`: ordered strictly-greater.
        #[inline(always)]
        fn ja_ss(a: f32, b: f32) -> bool {
            a > b
        }
        /// `comiss a, b` + `jb`: a < b, or either unordered.
        #[inline(always)]
        fn jb_ss(a: f32, b: f32) -> bool {
            !(a >= b)
        }
        /// `ucomiss` + `lahf` + `(an instruction of the original)` + `jp`: jumps unless the
        /// operands compare ordered-equal.
        #[inline(always)]
        fn jp_lahf(a: f32, b: f32) -> bool {
            !(a == b)
        }

        // Frame scratch mirror (14 words, zero-filled like the worker's fill).
        // fr[i] holds incoming-ESP-relative slot -56+4i.
        let mut fr = [0u32; 14];
        fr[7] = TABLE[0];
        fr[8] = TABLE[1];
        fr[9] = TABLE[2];
        fr[10] = TABLE[3];
        fr[11] = TABLE[4];
        fr[12] = TABLE[5];

        let a_flag: u32 = rt::callee_cdecl!(C_FLAG, u32,);
        let saved_al: u8 = a_flag as u8;
        if saved_al != 0 {
            rt::callee_cdecl!(C_FLAG_CLR, u32,);
        }
        rt::callee_cdecl!(C_POLL0, u32,);
        rt::callee_thiscall!(C_THIS1, u32, rt::relocated(THIS_OBJ), 1u32);
        let mut cl: u8 = (THIS_OBJ & 0xFF) as u8;

        // Indexed float select into the bank cap.
        let idx: i32 = grd(G09D8) as i32;
        let sel: f32 = match idx {
            0..=5 => f32::from_bits(TABLE[idx as usize]),
            -1 => f32::from_bits(fr[6]),
            -2 => f32::from_bits(fr[5]),
            8 => f32::from_bits(flag),
            _ => 0.0,
        };
        gwf(G09D0, sel);
        fr[6] = K_ONE.to_bits();
        let o0: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
        if rd8(o0.wrapping_add(0x328D)) != 0 {
            fr[6] = K_2P5.to_bits();
        }
        // Region 1/2: fold the scaled counter into the bank, or pass through.
        let cap: f32 = grf(G09D0);
        let acc: f32 = grf(G09CC);
        let mut x2: f32;
        if !jbe_ss(cap, acc) {
            let r: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x41u32);
            let f: f32 = rdf(r.wrapping_add(4));
            let p: f32 = mul(grf(G359C), K_1000);
            let t: i64 = trunc_to_i64(core::hint::black_box(p));
            fr[3] = t as u32;
            fr[4] = (t >> 32) as u32;
            let u: f32 = (t as u32) as f32;
            let mut v: f32 = mul(f, f32::from_bits(fr[6]));
            v = mul(v, u);
            let c0: f32 = grf(G09D0);
            v = add(v, grf(G09CC));
            fr[6] = v.to_bits();
            gwf(G09CC, v);
            if jb_ss(v, c0) {
                x2 = v;
            } else {
                x2 = c0;
                fr[6] = x2.to_bits();
                gwf(G09CC, x2);
                gw8(G09D4, 1);
            }
        } else if !jbe_ss(acc, cap) {
            let r: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x41u32);
            let f: f32 = rdf(r.wrapping_add(4));
            let p: f32 = mul(grf(G359C), K_1000);
            let t: i64 = trunc_to_i64(core::hint::black_box(p));
            fr[3] = t as u32;
            fr[4] = (t >> 32) as u32;
            let u: f32 = (t as u32) as f32;
            let mut v: f32 = mul(f, f32::from_bits(fr[6]));
            v = mul(v, u);
            let c0: f32 = grf(G09D0);
            let mut w: f32 = grf(G09CC);
            w = sub(w, v);
            fr[6] = w.to_bits();
            gwf(G09CC, w);
            if jb_ss(c0, w) {
                x2 = w;
            } else {
                x2 = c0;
                fr[6] = x2.to_bits();
                gwf(G09CC, x2);
                gw8(G09D4, 1);
            }
        } else {
            x2 = cap;
            fr[6] = x2.to_bits();
            gwf(G09CC, x2);
            gw8(G09D4, 1);
        }
        // Slow path selected by the flag's low byte.
        if ((flag & 0xFF) == 0) != MUTANT {
            let mut n: u32 = grd(G09D8);
            if ja_ss(K_HI, x2) {
                n = 2;
            }
            cl = 1;
            gwr(G09D8, n);
            gw8(G09D4, 0);
        } else {
            let g: u32 = rt::callee_cdecl!(C_GATE, u32, 0u32);
            if (g as u8) == 0 {
                x2 = grf(G09CC);
                fr[6] = x2.to_bits();
                // Fall-through: L171-172 have no jump after them, so the
                // gate==0 path executes the flag==0 block below as well.
                let mut n0: u32 = grd(G09D8);
                if ja_ss(K_HI, x2) {
                    n0 = 2;
                }
                cl = 1;
                gwr(G09D8, n0);
                gw8(G09D4, 0);
            } else {
                let armed: u8 = gr8(G09D4);
                fr[5] &= 0x00FF_FFFF;
                if armed == 0 {
                    x2 = grf(G09CC);
                    cl = 0;
                    fr[6] = x2.to_bits();
                } else {
                    let o1: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
                    cl = rd8(o1.wrapping_add(0x2BCE)) ^ rd8(o1.wrapping_add(0x2BCC));
                    let mut do_dec: bool = false;
                    if cl > 0x7F {
                        let o2: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
                        cl = rd8(o2.wrapping_add(0x2BDE)) ^ rd8(o2.wrapping_add(0x2BDC));
                        if cl <= 0x7F {
                            do_dec = true;
                        }
                    }
                    if do_dec {
                        let n: i32 = grd(G09D8) as i32;
                        gwf(G09CC, grf(G09D0));
                        if n > 0 {
                            gwr(G09D8, (n - 1) as u32);
                        }
                        gw8(G09D4, 0);
                    }
                    // Slow tail: recheck the flag pairs.
                    let o3: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
                    cl = rd8(o3.wrapping_add(0x2BCE)) ^ rd8(o3.wrapping_add(0x2BCC));
                    if cl > 0x7F {
                        x2 = grf(G09CC);
                        cl = 0;
                        fr[6] = x2.to_bits();
                    } else {
                        let o4: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
                        cl = rd8(o4.wrapping_add(0x2BDE)) ^ rd8(o4.wrapping_add(0x2BDC));
                        if cl <= 0x7F {
                            x2 = grf(G09CC);
                            cl = 0;
                            fr[6] = x2.to_bits();
                        } else {
                            x2 = grf(G09D0);
                            let mut n: i32 = grd(G09D8) as i32;
                            fr[6] = x2.to_bits();
                            gwf(G09CC, x2);
                            if n < 5 {
                                n += 1;
                                gwr(G09D8, n as u32);
                            }
                            cl = 0;
                            gw8(G09D4, 0);
                        }
                    }
                }
            }
        }
        // Clamp x2 into [K_LO, K_CLAMP_HI].
        {
            let mut x0: f32 = K_LO;
            let mut store: bool = false;
            if ja_ss(x0, x2) {
                store = true;
            } else {
                x0 = K_CLAMP_HI;
                if !jbe_ss(x2, x0) {
                    store = true;
                }
            }
            if store {
                gwf(G09CC, x0);
                fr[6] = x0.to_bits();
            }
        }
        // Signed lane amounts (zeroed, then polled on the cl==0 path only).
        let mut esi: i32 = 0;
        let mut edi: i32 = 0;
        if cl == 0 {
            let oa: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
            let da: u32 = rt::callee_thiscall!(C_DER0, u32, oa, 0u32);
            esi = rt::callee_cdecl!(C_AMT, u32, da) as i32;
            let ob: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
            let db: u32 = rt::callee_thiscall!(C_DER1, u32, ob, 0u32);
            edi = rt::callee_cdecl!(C_AMT, u32, db) as i32;
            let oc: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
            if rd8(oc.wrapping_add(0x328D)) != 0 {
                let s0: f32 = (grd(G7A80) as i32) as f32;
                let s0s: f32 = mul(s0, grf(GCCE8));
                fr[3] = s0s.to_bits();
                let s1: f32 = (grd(G7A8C) as i32) as f32;
                let s1s: f32 = mul(s1, grf(GCCF0));
                fr[4] = s1s.to_bits();
                rt::callee_cdecl!(C_SMP, u32, fr.as_mut_ptr().add(3) as u32, rt::relocated(SMP_P1), rt::relocated(SMP_P2));
                let bits: u32 = grd(G7A88);
                let e: u32 = (bits ^ grd(G7A84)) & bits;
                if (e as u8) & 2 != 0 {
                    gwr(G1824, grd(G1500));
                    gwr(G1828, grd(G1504));
                }
            }
            {
                let bits: u32 = grd(G7A88);
                let e: u32 = (bits ^ grd(G7A84)) & bits;
                if (e as u8) & 1 != 0 {
                    gwf(G1508, grf(G1810));
                    gwf(G150C, grf(G1814));
                }
            }
            let od: u32 = rt::callee_cdecl!(C_OBJ, u32, 1u32);
            let mut refresh: bool = false;
            if rd8(od.wrapping_add(0x3289)) != 0
                && (gr8(G7A88) & 1) != 0
            {
                let s1: f32 = (grd(G7A8C) as i32) as f32;
                let q: f32 = mul(s1, grf(GCCF0));
                fr[6] = q.to_bits();
                let r0: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0u32);
                if !jbe_ss(q, rdf(r0)) {
                    let r1: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x16u32);
                    if !jbe_ss(rdf(r1), f32::from_bits(fr[6])) {
                        let bits: u32 = grd(G7A88);
                        esi = 0;
                        let e: u32 = (bits ^ grd(G7A84)) & bits;
                        edi = 0;
                        if (e as u8) & 1 == 0 {
                            refresh = true;
                        }
                    }
                }
            }
            if refresh {
                let mut l0: f32 = sub(grf(G1508), grf(G1810));
                let mut l1: f32 = sub(grf(G150C), grf(G1814));
                l0 = add(l0, grf(G1500));
                l1 = add(l1, grf(G1504));
                gwf(G1500, l0);
                gwf(G1504, l1);
                gwr(G1824, grd(G1500));
                gwr(G1828, grd(G1504));
            }
            fr[6] = grf(G09CC).to_bits();
        }
        // Region 3: divisor from the scaled counter and a bit-level float
        // mix. Upper SSE lanes stay zero throughout (all sources are scalar
        // loads or cvtpd2ps), so only the low lane is modelled.
        let r3: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(0) as u32, 0x41u32);
        let p3: f32 = mul(grf(G359C), K_1000);
        let t3: i64 = trunc_to_i64(core::hint::black_box(p3));
        fr[3] = t3 as u32;
        fr[4] = (t3 >> 32) as u32;
        let mut x3: f32 = (t3 as u32) as f32;
        x3 = mul(x3, rdf(r3));
        x3 = mul(x3, K_EPS);
        x3 = mul(x3, f32::from_bits(fr[6]));
        let mut x2b: u32 = K_SIGN & x3.to_bits();
        let mut x0b: u32 = x3.to_bits() ^ x2b;
        x0b = if f32::from_bits(x0b) < K_MAG8M { 0xFFFF_FFFF } else { 0 };
        let mut x4: f32 = x3;
        let mut x1b: u32 = K_MAG8M.to_bits() & x0b;
        x1b |= x2b;
        let x1f: f32 = f32::from_bits(x1b);
        x4 = add(x4, x1f);
        x4 = sub(x4, x1f);
        let mut x0f: f32 = sub(x4, x3);
        x0b = if x0f < f32::from_bits(x2b) { 0 } else { 0xFFFF_FFFF };
        x0b &= K_ONE.to_bits();
        x4 = sub(x4, f32::from_bits(x0b));
        fr[5] = x4.to_bits();
        let _ = &mut x2b;
        // Object-pair refresh of bank1.
        let a12: u32 = rt::callee_cdecl!(C_PAIR2, u32, 0u32);
        let mut x5: f32 = grf(G1828);
        let mut x6: f32 = grf(G1824);
        let mut ecx: u32;
        let mut edx: u32;
        if a12 != 0 {
            let inner: u32 = rd32(a12.wrapping_add(0x20));
            if !jp_lahf(x6, rdf(inner.wrapping_add(0x30)))
                && !jp_lahf(x5, rdf(inner.wrapping_add(0x34)))
            {
                let mut v4: f32 = sub(x6, grf(G1500));
                let mut v3: f32 = sub(x5, grf(G1504));
                v4 = mul(v4, K_BLEND);
                v3 = mul(v3, K_BLEND);
                v4 = add(v4, grf(G1500));
                v3 = add(v3, grf(G1504));
                fr[3] = v4.to_bits();
                ecx = fr[3];
                fr[4] = v3.to_bits();
                edx = fr[4];
                gwr(G1500, ecx);
                gwr(G1504, edx);
            } else {
                edx = grd(G1504);
                ecx = grd(G1500);
            }
        } else {
            edx = grd(G1504);
            ecx = grd(G1500);
        }
        // Per-lane adjustments.
        let x3s: f32 = K_STEP;
        let x7: f32 = f32::from_bits(fr[5]);
        x2 = grf(G1500);
        if esi < 0 {
            let m: f32 = (esi.wrapping_neg()) as f32;
            let mut v0: f32 = mul(m, x3s);
            gwr(G1828, edx);
            x5 = grf(G1828);
            v0 = mul(v0, x7);
            x2 = sub(x2, v0);
            gwf(G1500, x2);
            ecx = grd(G1500);
            gwr(G1824, ecx);
            x6 = grf(G1824);
        }
        fr[6] = x2.to_bits();
        if esi > 0 {
            let m: f32 = esi as f32;
            let mut v0: f32 = mul(m, x3s);
            gwr(G1828, edx);
            x5 = grf(G1828);
            v0 = mul(v0, x7);
            v0 = add(v0, x2);
            x2 = v0;
            gwf(G1500, x2);
            ecx = grd(G1500);
            gwr(G1824, ecx);
            x6 = grf(G1824);
            fr[6] = x2.to_bits();
        }
        let mut x1: f32;
        if edi < 0 {
            let m: f32 = (edi.wrapping_neg()) as f32;
            let mut v1: f32 = mul(m, x3s);
            gwr(G1824, ecx);
            x6 = grf(G1824);
            v1 = mul(v1, x7);
            v1 = add(v1, grf(G1504));
            gwf(G1504, v1);
            gwr(G1828, grd(G1504));
            x5 = grf(G1828);
            x1 = v1;
        } else {
            x1 = grf(G1504);
        }
        fr[5] = x1.to_bits();
        if edi > 0 {
            let m: f32 = edi as f32;
            let mut v0: f32 = mul(m, x3s);
            gwr(G1824, ecx);
            x6 = grf(G1824);
            v0 = mul(v0, x7);
            x1 = sub(x1, v0);
            gwf(G1504, x1);
            gwr(G1828, grd(G1504));
            x5 = grf(G1828);
            fr[5] = x1.to_bits();
        }
        // Window clamps. The first branch keys on the comiss above it (SSE
        // moves and pops in between leave integer flags untouched).
        let c6: f32 = x6;
        let c2: f32 = x2;
        let w0: f32 = K_WIN;
        let mut w3: f32 = add(c6, w0);
        let mut w4: f32 = add(c2, w0);
        fr[1] = w3.to_bits();
        w3 = add(x5, w0);
        fr[4] = w3.to_bits();
        w3 = add(x1, w0);
        if !jbe_ss(c6, c2) {
            let mut v0: f32 = f32::from_bits(fr[1]);
            v0 = sub(v0, w4);
            v0 = div(v0, x7);
            v0 = add(v0, x2);
            x2 = v0;
            fr[6] = x2.to_bits();
            gwf(G1500, x2);
        }
        if !jbe_ss(x2, x6) {
            w4 = sub(w4, f32::from_bits(fr[1]));
            w4 = div(w4, x7);
            x2 = sub(x2, w4);
            fr[6] = x2.to_bits();
            gwf(G1500, x2);
        }
        x2 = f32::from_bits(fr[4]);
        if !jbe_ss(x5, x1) {
            let mut v0: f32 = x2;
            v0 = sub(v0, w3);
            v0 = div(v0, x7);
            v0 = add(v0, x1);
            x1 = v0;
            fr[5] = x1.to_bits();
            gwf(G1504, x1);
        }
        if !jbe_ss(x1, x5) {
            w3 = sub(w3, x2);
            w3 = div(w3, x7);
            x1 = sub(x1, w3);
            fr[5] = x1.to_bits();
            gwf(G1504, x1);
        }
        // Final min-folds.
        let f0: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x42u32);
        let mut m0: f32 = rdf(f0);
        if !ja_ss(m0, f32::from_bits(fr[6])) {
            fr[6] = m0.to_bits();
            gwf(G1500, m0);
        }
        let f1: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x42u32);
        m0 = rdf(f1.wrapping_add(4));
        if !ja_ss(m0, f32::from_bits(fr[5])) {
            fr[5] = m0.to_bits();
            gwf(G1504, m0);
        }
        let f2: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x43u32);
        let mut m1: f32 = f32::from_bits(rd32(f2) ^ K_SIGN);
        let c0: f32 = f32::from_bits(fr[6]);
        if !ja_ss(c0, m1) {
            gwf(G1500, m1);
        }
        let f3: u32 = rt::callee_cdecl!(C_PAIR, u32, fr.as_mut_ptr().add(3) as u32, 0x43u32);
        m1 = f32::from_bits(rd32(f3.wrapping_add(4)) ^ K_SIGN);
        let c1: f32 = f32::from_bits(fr[5]);
        if !ja_ss(c1, m1) {
            gwf(G1504, m1);
        }
        let ret: u32;
        if saved_al == 0 {
            ret = f3;
        } else {
            ret = rt::callee_cdecl!(C_FIN0, u32,);
            rt::callee_cdecl!(C_FIN1, u32,);
        }
        rt::callee_cdecl!(C_COOKIE, u32,);
        ret
    }
}

lf_checker_rt::export!(cdecl, rw_008B9880(flag: u32) -> u32 {
    unsafe { rw_008B9880_inner::<false>(flag) }
});
