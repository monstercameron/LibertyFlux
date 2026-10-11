// original: 0x009069A0 input_ui_armour_blip_build (proposed)

/// Build one armour-blip element from the current slot: run the setup
/// helper chain, resolve the slot's record and parameters, blend several
/// float stages, and emit through the register or the sink path.
///
/// Arguments: none (cdecl/0, plain `ret`). All entry registers are dead
/// (one is pushed and the slot overwritten with zero before the call).
/// Returns in EAX the final helper's answer. The original returns
/// stack-cookie garbage when the slot index is -1; the contract never feeds
/// -1 (see below), so that exit is not replicated.
///
/// Layout. Two globals hold slot indices (A8, the current slot, and N, the
/// fallback); the 1500-entry slot table holds one pointer per row. A live
/// record has a flag byte at `+8`, a parameter dword at `+0x48` (8 selects
/// the main parameter block), a gate dword at `+0x5C` (zero skips straight
/// to the tail, past the 0-word call), and two float triples at `+0x20`/`+0x24` and
/// `+0x30`/`+0x34`/`+0x38`. A row whose flag byte is clear reads its gate,
/// parameter and floats from row N instead of its own record.
///
/// Algorithm. Two mode bytes pick a setup code (2 or 7) for a four-call
/// helper chain. A third mode byte gates a query whose low byte is kept;
/// that byte selects, through a float-domain minimum against 180.0
/// (comiss+ja: strictly-greatered keeps the constant, else takes the byte;
/// unordered takes the byte) truncated to int, the argument of the next
/// helper (low byte shifted to the top). The slot record is resolved
/// (direct or fallback row); a zero gate jumps to the epilogue; a parameter
/// of 8 runs the main block (a 2-word call, two rounds of a query whose
/// pointed-to word is compared against -1, steering three format calls with
/// relocated constants, then a thiscall pair) while any other value runs a
/// shorter alternate (one call plus the same pair tail). An x87-returning
/// helper's float is saved; the slot floats (or the alternate pair plus
/// zero) become the clamp helper's input and its two outputs are collected.
/// A second setup quartet runs; another query's pointed-to float is
/// compared against 255.0 (unordered-aware: NaN keeps 255.0 itself) and,
/// when the limit strictly exceeds it, a second query's pointed-to float
/// replaces it. Then a float stage: the surviving comparison float is
/// truncated to int (exact cvttss2si semantics: NaN and out-of-range give
/// 0x80000000) and saved; the first clamp output minus a zero scratch
/// word, less the sum of two scratch words, is stored, as is the second
/// clamp output plus a zero scratch word. A second
/// x87 float is added in, the TLS flag (slot from the index global, tested
/// word at `+0x8CC`) is read, and two words are parked. When the flag is
/// set an object is allocated: null registers zero, otherwise a second
/// minimum (same byte against the saved int's low byte) truncated and
/// shifted becomes a phantom argument carried into a thiscall taking the
/// parked frame plus two blended words (a scratch word times 3.0 plus the
/// third x87 float; another scratch word times 2.0 plus the first x87
/// float), whose answer is registered. When clear the same blends go to a
/// five-word sink with the parked pair. A final stage forms two words (the
/// second clamp output plus a scratch word times 2.5; twice 0.5 minus the
/// first clamp output) for a five-word call taking the zero frame, and the
/// epilogue (a 0-word call, the final 1-word call with 0, guard check)
/// returns the final answer.
///
/// Edge cases: null query answers fault on the dereference (same fault both
/// sides); a null allocator answer takes the register-zero path without
/// faulting; float-to-int uses exact truncation semantics including the
/// indefinite value; all float comparisons are unordered-aware. The two
/// dead stores into leftover argument slots before the 0-word call are
/// below-ESP scratch and are omitted; the stack-cookie prologue/epilogue is
/// anti-tamper scratch and is not replicated.
///
/// Original: 0x009069A0 (cdecl, no stack arguments).
/// Shared body: `wrong_min = false` is the faithful rewrite; `true` is the
/// deliberately wrong version (maximum instead of minimum in the first
/// select) used only as the checker's mutant. This function has no signed
/// integer comparison of callee-influenced values (all integer checks are
/// equality; all ordered checks are float), so no signedness mutant applies.
unsafe fn body_9069a0(wrong_min: bool) -> u32 {
    unsafe {
        const A8: u32 = 0x0103_44A8;
        const N: u32 = 0x0103_4494;
        const T1: u32 = 0x0118_F6F8;
        const B250: u32 = 0x0116_C250;
        const B5A8: u32 = 0x0116_15A8;
        const TLSIDX: u32 = 0x017A_BA14;
        const TLS_FLAG: u32 = 0x8CC;
        const F180: u32 = 0x00E8_1218;
        const F255: u32 = 0x00FE_8C08;
        const F3: u32 = 0x00FE_8A94;
        const F2: u32 = 0x00FE_8A24;
        const F2_5: u32 = 0x00FE_8A60;
        const F0_5: u32 = 0x00FE_8830;
        const THIS_Q: u32 = 0x0116_15A8;
        const THIS_MK: u32 = 0x0116_BFF0;
        const FMT0: u32 = 0x00E8_4AE0;
        const FMT1: u32 = 0x00E8_4AEC;
        const FMT2: u32 = 0x00E8_4AF8;
        const MAT: u32 = 0x0119_0E70;
        const C_S0: u32 = 1;
        const C_S1: u32 = 2;
        const C_S2: u32 = 3;
        const C_FIN: u32 = 4;
        const C_QRYB: u32 = 5;
        const C_LK: u32 = 6;
        const C_USE: u32 = 7;
        const C_QF: u32 = 8;
        const C_SET4: u32 = 9;
        const C_NOP0: u32 = 10;
        const C_PAIR0: u32 = 11;
        const C_SETC: u32 = 12;
        const C_PARM: u32 = 13;
        const C_QW: u32 = 14;
        const C_FMT2: u32 = 15;
        const C_FMT3: u32 = 16;
        const C_MK: u32 = 17;
        const C_FIN2: u32 = 18;
        const C_ALT: u32 = 19;
        const C_FST1: u32 = 20;
        const C_CLAMP: u32 = 21;
        const C_SET2: u32 = 22;
        const C_SET4B: u32 = 23;
        const C_FST0: u32 = 24;
        const C_NEW: u32 = 25;
        const C_CTOR: u32 = 26;
        const C_REG: u32 = 27;
        const C_SINK5: u32 = 28;
        const C_FIN5: u32 = 29;
        const C_NOPZ: u32 = 30;
        const C_COOKIE: u32 = 31;

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
        unsafe fn glob(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// comiss+ja select: the second value unless the first is strictly
        /// greater (ordered); unordered takes the first. The wrong version
        /// swaps the arms (maximum instead of minimum).
        #[inline(always)]
        fn select_min(x0: f32, x1: f32, wrong: bool) -> f32 {
            let gt = core::hint::black_box(x0) > core::hint::black_box(x1);
            if !wrong {
                if gt { x1 } else { x0 }
            } else if gt {
                x0
            } else {
                x1
            }
        }
        /// Exact cvttss2si: truncate toward zero; NaN and out-of-range give
        /// 0x80000000 (Rust `as` saturates instead and must not be used).
        #[inline(always)]
        fn cvtt(x: f32) -> u32 {
            if x.is_nan() {
                return 0x80000000;
            }
            let t = x.trunc();
            if t < 2147483648.0 && t >= -2147483648.0 {
                t as i32 as u32
            } else {
                0x80000000
            }
        }

        let mut fr = [0u32; 0x40];
        let base = fr.as_mut_ptr() as u32;
        let at = |w: usize| base.wrapping_add((w as u32).wrapping_mul(4));
        // Tail shared by both exits (the 0-word call before it runs only on
        // the normal path; the gate-zero skip lands past it).
        let tail = || unsafe {
            let fin = lf_checker_rt::callee_cdecl!(C_FIN, u32, 0);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            fin
        };

        let b250 = lf_checker_rt::relocated(B250);
        let code = if rd8(b250) == 0x6a || rd8(b250.wrapping_add(3)) != 0 { 2u32 } else { 7u32 };
        lf_checker_rt::callee_cdecl!(C_S0, u32, code);
        lf_checker_rt::callee_cdecl!(C_S1, u32, 0);
        lf_checker_rt::callee_cdecl!(C_S2, u32, 1);
        lf_checker_rt::callee_cdecl!(C_FIN, u32, 1);
        let mut fb: u8 = 0xff;
        if rd8(lf_checker_rt::relocated(B5A8)) != 0 {
            fb = (lf_checker_rt::callee_thiscall!(C_QRYB, u32, lf_checker_rt::relocated(THIS_Q)) & 0xFF) as u8;
        }
        wr8(base.wrapping_add(0x0C), fb);
        let lk = lf_checker_rt::callee_cdecl!(C_LK, u32, at(8), 0x41, rd32(at(3)));
        let w0 = rd32(lk);
        lf_checker_rt::callee_cdecl!(C_USE, u32, w0);
        lf_checker_rt::callee_cdecl!(C_QF, u32, at(0x0D), 0x46);
        lf_checker_rt::callee_cdecl!(C_QF, u32, at(0x13), 0x47);
        lf_checker_rt::callee_cdecl!(C_SET4, u32, 2, at(0x0D), at(0x13), 0);
        lf_checker_rt::callee_cdecl!(C_NOP0, u32,);
        lf_checker_rt::callee_cdecl!(C_PAIR0, u32, 0, 0);
        let edi = fb as u32;
        let c180 = f32::from_bits(glob(F180));
        let m = select_min((edi as i32) as f32, c180, wrong_min);
        lf_checker_rt::callee_cdecl!(C_SETC, u32, (cvtt(m) & 0xFF) << 24);

        let a8 = glob(A8) as i32;
        let n = glob(N) as i32;
        let table = lf_checker_rt::relocated(T1);
        let row = |i: i32| table.wrapping_add((i as u32).wrapping_mul(4));
        let slot = rd32(row(a8));
        let flag8 = rd8(slot.wrapping_add(8));
        wr8(base.wrapping_add(0x13), flag8);
        let tn = rd32(row(n));
        let v5c = if flag8 != 0 { rd32(slot.wrapping_add(0x5C)) } else { rd32(tn.wrapping_add(0x5C)) };
        if v5c == 0 {
            return tail();
        }
        let v48 = if flag8 != 0 { rd32(slot.wrapping_add(0x48)) } else { rd32(tn.wrapping_add(0x48)) };
        if v48 == 8 {
            wr32(at(3), 0xFFFF_FFFF);
            let pa = lf_checker_rt::callee_cdecl!(C_PARM, u32, a8 as u32, at(3));
            wr32(at(7), pa);
            let q1 = lf_checker_rt::callee_cdecl!(C_QW, u32, at(8), 7);
            let neg1 = rd32(at(3));
            if neg1 != rd32(q1) {
                let q2 = lf_checker_rt::callee_cdecl!(C_QW, u32, at(8), 0x10);
                if neg1 != rd32(q2) {
                    let f1c = rd32(at(7));
                    lf_checker_rt::callee_cdecl!(C_FMT3, u32, at(0x18), lf_checker_rt::relocated(FMT2), f1c);
                } else {
                    lf_checker_rt::callee_cdecl!(C_FMT2, u32, at(0x18), lf_checker_rt::relocated(FMT1));
                }
            } else {
                lf_checker_rt::callee_cdecl!(C_FMT2, u32, at(0x18), lf_checker_rt::relocated(FMT0));
            }
            let mk = lf_checker_rt::callee_thiscall!(C_MK, u32, lf_checker_rt::relocated(THIS_MK), at(0x18));
            lf_checker_rt::callee_cdecl!(C_FIN2, u32, at(0x20), mk);
        } else {
            let pa = lf_checker_rt::callee_cdecl!(C_ALT, u32, a8 as u32);
            lf_checker_rt::callee_cdecl!(C_FIN2, u32, at(0x20), pa);
        }
        let st0a = lf_checker_rt::callee_cdecl!(C_FST1, u32, at(0x20), 1);
        wr32(at(3), st0a);
        let s2 = rd32(row(a8));
        if rd8(s2.wrapping_add(8)) != 0 {
            wr32(at(8), rd32(s2.wrapping_add(0x30)));
            wr32(at(9), rd32(s2.wrapping_add(0x34)));
            wr32(at(0x0A), rd32(s2.wrapping_add(0x38)));
        } else {
            wr32(at(8), rd32(s2.wrapping_add(0x20)));
            wr32(at(9), rd32(s2.wrapping_add(0x24)));
            wr32(at(0x0A), 0);
        }
        lf_checker_rt::callee_cdecl!(C_CLAMP, u32, at(8), at(0x0F), lf_checker_rt::relocated(MAT), 1);
        lf_checker_rt::callee_cdecl!(C_SET2, u32, 0, 0x41200000);
        wr32(at(5), 0x3BE5_6042);
        wr32(at(6), 0x3BA3_D70A);
        lf_checker_rt::callee_cdecl!(C_SET4B, u32, 7, 0, at(5), 0);
        let q3 = lf_checker_rt::callee_cdecl!(C_QF, u32, at(8), 0x4D);
        let lim255 = f32::from_bits(glob(F255));
        // The limit stays in xmm0; the pointed-to float is only compared.
        // Fall through (second query, its answer becomes x0) iff the limit
        // strictly exceeds it; otherwise x0 is the limit itself.
        let mut x0 = lim255;
        if lim255 > f32::from_bits(rd32(q3)) {
            let q4 = lf_checker_rt::callee_cdecl!(C_QF, u32, at(0x16), 0x4D);
            x0 = f32::from_bits(rd32(q4));
        }
        let p0 = f32::from_bits(rd32(at(0x0F)));
        let f34 = f32::from_bits(rd32(at(0x0D)));
        let mut x1 = sub(p0, f34);
        wr32(at(7), cvtt(x0));
        let t14 = f32::from_bits(rd32(at(5)));
        let t0c = f32::from_bits(rd32(at(3)));
        x0 = add(t14, t0c);
        x1 = sub(x1, x0);
        let t38 = f32::from_bits(rd32(at(0x0E)));
        let p1 = f32::from_bits(rd32(at(0x10)));
        x0 = add(t38, p1);
        wr32(at(8), x1.to_bits());
        wr32(at(0x16), x0.to_bits());
        let st0b = lf_checker_rt::callee_cdecl!(C_FST0, u32,);
        wr32(at(0x15), st0b);
        let u0 = f32::from_bits(st0b);
        let u18 = f32::from_bits(rd32(at(6)));
        x0 = add(u0, u18);
        x1 = f32::from_bits(rd32(at(0x16)));
        x1 = sub(x1, x0);
        let ti = glob(TLSIDX);
        let tls_on = rd32(lf_checker_rt::tls_slot(ti as usize).wrapping_add(TLS_FLAG)) != 0;
        wr32(at(0x11), rd32(at(8)));
        wr32(at(0x12), x1.to_bits());
        if tls_on {
            let ao = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x1C, 0);
            if ao != 0 {
                let cw = rd32(at(7));
                let m2 = select_min((edi as i32) as f32, ((cw & 0xFF) as i32) as f32, false);
                let ph = (cvtt(m2) & 0xFF) << 24;
                let st0c = lf_checker_rt::callee_cdecl!(C_FST0, u32,);
                let f1c = f32::from_bits(rd32(at(6))); // [esp+0x1c] at esp=F-4 = F+0x18 (phantom push shifts it)
                let c3 = f32::from_bits(glob(F3));
                let ta = mul(f1c, c3);
                wr32(at(8), st0c);
                let s20 = f32::from_bits(st0c);
                let y1 = add(s20, ta);
                let t14b = f32::from_bits(rd32(at(5)));
                let c2 = f32::from_bits(glob(F2));
                let mut y0 = mul(t14b, c2);
                let t0cb = f32::from_bits(rd32(at(3)));
                y0 = add(y0, t0cb);
                let ra = lf_checker_rt::callee_thiscall!(
                    C_CTOR, u32, ao, at(0x11), y0.to_bits(), y1.to_bits(), ph
                );
                lf_checker_rt::callee_cdecl!(C_REG, u32, ra);
            } else {
                lf_checker_rt::callee_cdecl!(C_REG, u32, 0);
            }
        } else {
            let cw = rd32(at(7));
            let m2 = select_min((edi as i32) as f32, ((cw & 0xFF) as i32) as f32, false);
            let ph = (cvtt(m2) & 0xFF) << 24;
            let st0c = lf_checker_rt::callee_cdecl!(C_FST0, u32,);
            let f1c = f32::from_bits(rd32(at(6))); // [esp+0x1c] at esp=F-4 = F+0x18 (phantom push shifts it)
            let c3 = f32::from_bits(glob(F3));
            let ta = mul(f1c, c3);
            wr32(at(8), st0c);
            let s20 = f32::from_bits(st0c);
            let y1 = add(s20, ta);
            let t14b = f32::from_bits(rd32(at(5)));
            let c2 = f32::from_bits(glob(F2));
            let mut y0 = mul(t14b, c2);
            let t0cb = f32::from_bits(rd32(at(3)));
            y0 = add(y0, t0cb);
            lf_checker_rt::callee_cdecl!(C_SINK5, u32, rd32(at(0x11)), rd32(at(0x12)), y0.to_bits(), y1.to_bits(), ph);
        }
        let m40 = f32::from_bits(rd32(at(0x10)));
        let m38 = f32::from_bits(rd32(at(0x0E)));
        let mut z1 = sub(m40, m38);
        let m18 = f32::from_bits(rd32(at(6)));
        let c25 = f32::from_bits(glob(F2_5));
        z1 = add(z1, mul(m18, c25));
        let b05 = f32::from_bits(glob(F0_5));
        let pout0 = f32::from_bits(rd32(at(0x0F)));
        let f34b = f32::from_bits(rd32(at(0x0D)));
        let mut w1 = b05;
        w1 = sub(w1, pout0);
        w1 = add(w1, b05);
        w1 = add(w1, f34b);
        lf_checker_rt::callee_cdecl!(C_FIN5, u32, w1.to_bits(), z1.to_bits(), at(0x20), 0xFFFF_FFFF, 0xFFFF_FFFF);
        lf_checker_rt::callee_cdecl!(C_NOPZ, u32,);
        tail()
    }
}

lf_checker_rt::export!(cdecl, rw_009069A0() -> u32 {
    unsafe { body_9069a0(false) }
});
