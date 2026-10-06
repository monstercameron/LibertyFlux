// original: 0x00903D50 input_ui_cell_build (proposed)

/// Build one indexed UI cell: look up its table entry, construct its backing
/// object, generate four geometry pairs, resolve its colour, and emit an
/// element through the constructor/fold path or the sink path.
///
/// Arguments (cdecl, two stack words): `a` and `b` are cell coordinates.
/// Returns in EAX the id-fold bits on the constructor path or the tail
/// helper's answer on the sink path. All entry registers are dead.
///
/// Layout. The cell index is `stride * b + a` (plus the limit when the mode
/// byte is set), all wrapping 32-bit; the table base comes from a global.
/// Element objects and the id-fold pair work as in the sibling sweeps
/// (vtable at `+0`, id word at `+4`, fold of bits 14..24 from two slot-+8
/// calls with SIGNED `% 16` / `/ 16`).
///
/// Algorithm. A 3-word preparation call takes the coordinates and a scratch
/// pointer. The looked-up entry must differ from -1; the index must either
/// satisfy the SIGNED `0 <= index < limit` window or have the mode byte set
/// (both the negativity test and the bound are SIGNED). A 2-word check gates
/// a setup pair, then a format call (buffer, one of three relocated
/// constants, the index or index-minus-limit) feeds a make call whose answer
/// constructs the object; a null object retries once with the third constant
/// when the mode byte is set. A TLS flag (slot from the index global, tested
/// word at `+0x8CC`) gates a bind-and-register pair. Four geometry rounds
/// follow (counter 0, 8, 16, 24): a pair call fills two words, which become
/// the clamp helper's input; its two outputs land in the pair slots. Each
/// round also forms `ratio = A / B` from a cmov-selected integer pair
/// (converted SIGNED to float, divided in that order) and, when a limit
/// global strictly exceeds the ratio (unordered-aware: NaN skips) and the
/// mode byte is set, blends the slot towards a base global by the ratio.
/// Colour resolution: without the setup flag the colour is one of two
/// constants by the mode byte; otherwise it reuses the constructed object.
/// With the mode byte clear and a gate byte clear, a triple probe call
/// (null-, zero- and SIGNED-`<= 0`-gated, then a thiscall query whose low
/// byte decides) either keeps the colours or resets them to a third
/// constant; two flag bytes then optionally refresh the colour through a
/// lookup call. A second TLS flag picks the exit: set allocates and
/// constructs with the pair blocks and runs the id-fold pair (its bits are
/// the return value); clear runs a tail helper, a 2-word call, then four
/// sink calls walking the pair slots backwards (9 plain words each: three
/// slot words, two zeros, -1.0, the colour, two zero scratch words), and
/// returns the final helper's answer.
///
/// Edge cases: a null probe/lookup/constructor answer faults on the
/// dereference (same fault both sides); a null first-round object skips or
/// retries as above; division by zero yields infinity/NaN through the
/// unordered-aware skip. The stack-cookie prologue/epilogue is anti-tamper
/// scratch and is not replicated.
///
/// Original: 0x00903D50 (cdecl, two stack words).
/// Shared body: `signed_fold = true` is the faithful rewrite; `false` is
/// the deliberately wrong version (UNSIGNED fold) used only as the
/// checker's mutant.
unsafe fn body_903d50(signed_fold: bool, a: u32, b: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x0103_44E4;
        const LIMIT: u32 = 0x0103_44EC;
        const MODEW: u32 = 0x0116_09F4; // mode byte is byte 2 of this word
        const TBASE: u32 = 0x0118_F4E8;
        const G2F58: u32 = 0x0103_2F58;
        const FAC: u32 = 0x01BB_5554;
        const FMT0: u32 = 0x00E8_49B8;
        const FMT1: u32 = 0x00E8_49C4;
        const FMT2: u32 = 0x00E8_49D0;
        const TLSIDX: u32 = 0x017A_BA14;
        const TLS_FLAG: u32 = 0x8CC;
        const MAT: u32 = 0x0119_0E70;
        const PAIR_A0: u32 = 0x0105_C884;
        const PAIR_A1: u32 = 0x0105_C888;
        const PAIR_B0: u32 = 0x0105_C880;
        const PAIR_B1: u32 = 0x0105_C87C;
        const GLIMIT: u32 = 0x00FE_88E8;
        const GBASE: u32 = 0x00FE_8830;
        const GATEB: u32 = 0x011D_B23B;
        const FLGBD: u32 = 0x0118_F4BD;
        const FLGBE: u32 = 0x0118_F4BE;
        const C_PREP: u32 = 1;
        const C_CHK: u32 = 2;
        const C_NOP1: u32 = 3;
        const C_USE: u32 = 4;
        const C_FMT: u32 = 5;
        const C_MK: u32 = 6;
        const C_CTOR1: u32 = 7;
        const C_NOP2: u32 = 8;
        const C_NEW: u32 = 9;
        const C_BIND: u32 = 10;
        const C_REG: u32 = 11;
        const C_PAIR: u32 = 12;
        const C_CLAMP: u32 = 13;
        const C_SEL: u32 = 14;
        const C_PEEK: u32 = 15;
        const C_QRY: u32 = 16;
        const C_LKUP: u32 = 17;
        const C_CTOR2: u32 = 18;
        const C_TAIL1: u32 = 20;
        const C_PAIR2: u32 = 21;
        const C_SINK9: u32 = 22;
        const C_FIN: u32 = 23;
        const C_COOKIE: u32 = 24;

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
        unsafe fn glob(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(core::hint::black_box(obj));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(8)) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn fold_pair(obj: u32, signed: bool) -> u32 {
            unsafe {
                let r1 = vcall(obj);
                let x = if signed {
                    (16i32.wrapping_sub((r1 as i32).wrapping_rem(16))).wrapping_rem(16)
                } else {
                    (16u32.wrapping_sub(r1.wrapping_rem(16)).wrapping_rem(16)) as i32
                };
                let r2 = vcall(obj);
                let q = if signed {
                    (r2 as i32).wrapping_add(x).wrapping_div(16)
                } else {
                    r2.wrapping_add(x as u32).wrapping_div(16) as i32
                };
                let m = rd32(obj.wrapping_add(4));
                let bits = ((q as u32) << 14 ^ m) & 0x01FF_C000;
                wr32(obj.wrapping_add(4), m ^ bits);
                bits
            }
        }

        let mut fr = [0u32; 0x40];
        let base = fr.as_mut_ptr() as u32;
        let at = |w: usize| base.wrapping_add((w as u32).wrapping_mul(4));
        let mode = rd8(lf_checker_rt::relocated(MODEW).wrapping_add(2));

        lf_checker_rt::callee_cdecl!(C_PREP, u32, a, b, at(0x10));
        let stride = glob(STRIDE);
        let w = glob(LIMIT) as i32;
        let mut idx = stride.wrapping_mul(b).wrapping_add(a);
        if mode != 0 {
            idx = idx.wrapping_add(w as u32);
        }
        let idx_i = idx as i32;
        let tbase = glob(TBASE);
        let mut has_setup = false;
        wr32(at(3), 0);
        let tv = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
        let in_window = idx_i >= 0 && idx_i < w;
        if tv != 0xFFFF_FFFF && (in_window || mode != 0) {
            let g = glob(G2F58);
            if (lf_checker_rt::callee_cdecl!(C_CHK, u32, tv, g) & 0xFF) != 0 {
                lf_checker_rt::callee_cdecl!(C_NOP1, u32,);
                let tv2 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
                lf_checker_rt::callee_cdecl!(C_USE, u32, tv2);
                let (fmt, ia) = if mode != 0 {
                    (FMT1, idx.wrapping_sub(w as u32))
                } else {
                    (FMT0, idx)
                };
                lf_checker_rt::callee_cdecl!(C_FMT, u32, at(0x2B), lf_checker_rt::relocated(fmt), ia);
                let fac = glob(FAC);
                let mk = lf_checker_rt::callee_cdecl!(C_MK, u32, at(0x2B), 0);
                let o1 = lf_checker_rt::callee_thiscall!(C_CTOR1, u32, fac, mk);
                wr32(at(3), o1);
                if o1 == 0 && mode != 0 {
                    lf_checker_rt::callee_cdecl!(C_FMT, u32, at(0x2B), lf_checker_rt::relocated(FMT2), idx);
                    let fac2 = glob(FAC);
                    let mk2 = lf_checker_rt::callee_cdecl!(C_MK, u32, at(0x2B), 0);
                    let o2 = lf_checker_rt::callee_thiscall!(C_CTOR1, u32, fac2, mk2);
                    wr32(at(3), o2);
                }
                lf_checker_rt::callee_cdecl!(C_NOP2, u32,);
                has_setup = true;
                let ti = glob(TLSIDX);
                if rd32(lf_checker_rt::tls_slot(ti as usize).wrapping_add(TLS_FLAG)) != 0 {
                    let ao = lf_checker_rt::callee_cdecl!(C_NEW, u32, 8, 0);
                    let reg = if ao != 0 {
                        let tv3 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
                        lf_checker_rt::callee_thiscall!(C_BIND, u32, ao, tv3)
                    } else {
                        0
                    };
                    lf_checker_rt::callee_cdecl!(C_REG, u32, reg);
                }
            }
        }

        // Geometry rounds.
        let mat = lf_checker_rt::relocated(MAT);
        let mut ctr: u32 = 0;
        let mut slot: u32 = at(0x19);
        loop {
            let p1 = at(8).wrapping_add(ctr);
            let p2 = at(0x10).wrapping_add(ctr);
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, p1, p2, a, b);
            let v0 = rd32(p2);
            let dst = slot.wrapping_sub(4);
            wr32(dst, v0);
            let v1 = rd32(at(0x11).wrapping_add(ctr));
            wr32(slot, v1);
            wr32(dst.wrapping_add(8), 0);
            lf_checker_rt::callee_cdecl!(C_CLAMP, u32, dst, at(6), mat, 1);
            let o0 = rd32(at(6));
            let o1 = rd32(at(7));
            wr32(dst, o0);
            wr32(slot, o1);
            let s1 = lf_checker_rt::callee_cdecl!(C_SEL, u32,) & 0xFF;
            let av = if s1 != 0 { glob(PAIR_A1) } else { glob(PAIR_A0) };
            let s2 = lf_checker_rt::callee_cdecl!(C_SEL, u32,) & 0xFF;
            let bv = if s2 != 0 { glob(PAIR_B1) } else { glob(PAIR_B0) };
            let ratio = div((av as i32) as f32, (bv as i32) as f32);
            let limit = f32::from_bits(glob(GLIMIT));
            if limit > ratio && mode != 0 {
                let bf = f32::from_bits(glob(GBASE));
                let t0 = mul(ratio, bf);
                let t1 = sub(bf, t0);
                let cur = f32::from_bits(rd32(slot));
                let t2 = mul(cur, ratio);
                wr32(slot, add(t1, t2).to_bits());
            }
            ctr = ctr.wrapping_add(8);
            slot = slot.wrapping_add(0x10);
            if ctr >= 0x20 {
                break;
            }
        }

        // Colour resolution.
        let mut esi_c = 0xFFFF_FFFFu32;
        let mut edi_c: u32;
        if !has_setup {
            esi_c = if mode == 0 { 0xFF60_7E91 } else { 0xFF00_0000 };
            edi_c = 0;
        } else {
            edi_c = rd32(at(3));
        }
        if mode == 0 {
            let mut reset = false;
            if rd8(lf_checker_rt::relocated(GATEB)) != 0 {
                reset = true;
            } else {
                let p1 = lf_checker_rt::callee_cdecl!(C_PEEK, u32,);
                if p1 != 0 {
                    let p2 = lf_checker_rt::callee_cdecl!(C_PEEK, u32,);
                    if rd32(p2.wrapping_add(0xB0)) != 0
                        && (rd32(p2.wrapping_add(0xB8)) as i32) > 0
                    {
                        let p3 = lf_checker_rt::callee_cdecl!(C_PEEK, u32,);
                        let th = rd32(p3.wrapping_add(0xB0));
                        if th == 0 || (rd32(p3.wrapping_add(0xB8)) as i32) <= 0 {
                            reset = true;
                        } else if (lf_checker_rt::callee_thiscall!(C_QRY, u32, th) & 0xFF) == 0 {
                            reset = true;
                        }
                    }
                }
            }
            if reset {
                esi_c = 0xFF14_1414;
                edi_c = 0;
            }
            if rd8(lf_checker_rt::relocated(FLGBD)) != 0 {
                let ans = lf_checker_rt::callee_cdecl!(C_LKUP, u32, at(3), 4, 0xFF);
                esi_c = rd32(ans);
            }
            if rd8(lf_checker_rt::relocated(FLGBE)) != 0 {
                let ans = lf_checker_rt::callee_cdecl!(C_LKUP, u32, at(3), 0x13, 0xFF);
                esi_c = rd32(ans);
            }
        }

        let ti2 = glob(TLSIDX);
        let exit_eax: u32;
        if rd32(lf_checker_rt::tls_slot(ti2 as usize).wrapping_add(TLS_FLAG)) != 0 {
            let ao = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x80, 0);
            if ao != 0 {
                let e = lf_checker_rt::callee_thiscall!(
                    C_CTOR2, u32, ao, at(0x18), at(8), edi_c, esi_c
                );
                exit_eax = fold_pair(e, signed_fold);
            } else {
                fold_pair(0, signed_fold);
                exit_eax = 0;
            }
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        } else {
            lf_checker_rt::callee_cdecl!(C_TAIL1, u32, edi_c);
            lf_checker_rt::callee_cdecl!(C_PAIR2, u32, 5, 4);
            let mut cur = at(0x25);
            wr32(at(3), cur);
            let mut di: i32 = 3;
            loop {
                let wlo = rd32(base.wrapping_add(0x20).wrapping_add((di as u32).wrapping_mul(8)));
                let whi = rd32(base.wrapping_add(0x24).wrapping_add((di as u32).wrapping_mul(8)));
                let m0 = rd32(cur.wrapping_sub(4));
                let m1 = rd32(cur);
                let m2 = rd32(cur.wrapping_add(4));
                lf_checker_rt::callee_cdecl!(C_SINK9, u32, m0, m1, m2, 0, 0, 0xBF80_0000, esi_c, wlo, whi);
                cur = cur.wrapping_sub(0x10);
                wr32(at(3), cur);
                di -= 1;
                if di < 0 {
                    break;
                }
            }
            exit_eax = lf_checker_rt::callee_cdecl!(C_FIN, u32,);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        }
        exit_eax
    }
}

lf_checker_rt::export!(cdecl, rw_00903D50(a: u32, b: u32) -> u32 {
    unsafe { body_903d50(true, a, b) }
});
