// original: 0x00ad9130 input_ui_build_tables (proposed)

/// Scale input ranges into global tables, then build per-index output records.
///
/// `a0` points at a float/int array, `a1` and the slot holding `a2` at index
/// arrays; `a2`/`a3` are also read as plain integers for the quotient block.
/// `a4`..`a7` are float parameters (bit patterns in `u32` args), `a8` carries
/// the fill-base transport (see below; the original never reads it), `a9` is
/// unread, `a10` is the inner-loop bound and third divisor, `a11` the outer
/// bound and second divisor. All integer division and all bound comparisons
/// are SIGNED (idiv, jl/js/jge/jle); only the flag bytes and the odd/even
/// test are unsigned. Returns whatever the last executed callee answered on
/// the taken path (see below), 0 never being written explicitly.
///
/// Prologue: four signed quotients `(a1-a0)/a10`, `(a3-a2)/a11`,
/// `(a0-a1)/a10`, `(a2-a3)/a11` and eight scaled floats (`(a5-a4)/a10`-style
/// mixes of the float args against `CONST/a10` and `CONST/a11`, where CONST
/// is the read-only float at `SCALE_CONST`) are stored into the global block
/// `G_OUT` (`0x1550e04`..`0x1550e58`), then callee 1 runs once with
/// `(0, TAB1, TAB2, TAB3, a10, a11)`. When `a11 < 1` (signed) the function
/// returns that answer immediately.
///
/// Otherwise outer index 1..=a11 (signed): callee 1 runs again with the
/// outer index and one of two table sets (odd/even pick which), then callee
/// 2 runs with `(2, 4*a10+2)` when flag `MODE_HI` is set, `(4, 2*a10+2)`
/// when both flags are clear, and is skipped when only `MODE_LO` is set.
/// Inner index 0..=a10 (signed; skipped entirely when `a10` is negative)
/// runs one of three branches selected by the flags: branch A (HI set) makes
/// four 9/8-word callee-3/4 calls of float triples read through the tables
/// and `a0` (its second call reads through `[a0+table]`, which is unmapped,
/// so branch A always raises an access violation on both sides); branch B
/// (both clear) makes one 9-word and one 8-word callee-3/5 call (its second
/// triple base is the word of uninitialised stack at the prologue scratch
/// slot, which the contract fills with the heap base that `a8` transports);
/// branch C (LO only) copies 9-dword records from the tables and `a0`/`a1`
/// into the record buffer behind the object at `REC_OBJ`, advancing the
/// counter at `REC_COUNT`, and calls callee 6 (no stack args) whenever the
/// counter estimate reaches `REC_CAP` (0xfffc, signed). The tail calls
/// callee 7 (no stack args) except on branch-C trials.
///
/// The original reuses two incoming argument slots as scratch (the `a2` slot
/// holds the selected table pointer, the `a3` slot a byte offset or the
/// record counter); the rewrite keeps those in locals, so the stack check
/// stays off. Callee answers are never compared, only returned.
///
/// Original: 0x00ad9130 (cdecl, twelve stack words; `a8`/`a9` unread).
lf_checker_rt::export!(cdecl, rw_00ad9130(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32) -> u32 {
    unsafe {
        const SCALE_CONST: u32 = 0x00fe88e8;
        const MODE_LO: u32 = 0x0154e2b5;
        const MODE_HI: u32 = 0x0154e2b6;
        const G_E04: u32 = 0x01550e04;
        const G_E08: u32 = 0x01550e08;
        const G_E0C: u32 = 0x01550e0c;
        const G_E10: u32 = 0x01550e10;
        const G_E14: u32 = 0x01550e14;
        const G_E18: u32 = 0x01550e18;
        const G_E1C: u32 = 0x01550e1c;
        const G_E20: u32 = 0x01550e20;
        const G_E24: u32 = 0x01550e24;
        const G_E28: u32 = 0x01550e28;
        const G_E2C: u32 = 0x01550e2c;
        const G_E30: u32 = 0x01550e30;
        const G_E34: u32 = 0x01550e34;
        const G_E50: u32 = 0x01550e50;
        const G_E54: u32 = 0x01550e54;
        const G_E58: u32 = 0x01550e58;
        const REC_OBJ: u32 = 0x01550ea4;
        const REC_COUNT: u32 = 0x01550ea8;
        const REC_CAP: i32 = 0xfffc;
        const TAB_S_ODD: u32 = 0x01592ac0;
        const TAB_B_ODD: u32 = 0x0158d600;
        const TAB_A_ODD: u32 = 0x015920c0;
        const TAB_P_ODD: u32 = 0x0158de20;
        const TAB_C_ODD: u32 = 0x015928c0;
        const TAB_SLOT_ODD: u32 = 0x0158e660;
        const C_MAIN: u32 = 1;
        const C_MODE: u32 = 2;
        const C_TRIP9: u32 = 3;
        const C_TRIP8A: u32 = 4;
        const C_TRIP8B: u32 = 5;
        const C_FLUSH: u32 = 6;
        const C_TAIL: u32 = 7;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn f(v: u32) -> f32 {
            f32::from_bits(v)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn g(a: u32) -> u32 {
            lf_checker_rt::relocated(a)
        }

        let _ = a9;
        let fill_base = a8;
        let d10 = a10 as i32;
        let d11 = a11 as i32;
        // Divisors are never zero by contract (the original's divide-error
        // fault has no Rust equivalent); dividends are small differences.
        let q0 = (a1.wrapping_sub(a0) as i32) / d10;
        wr32(g(G_E50), a0);
        let f10 = div(rdf(g(SCALE_CONST)), d10 as f32);
        wr32(g(G_E0C), q0 as u32);
        let q1 = (a3.wrapping_sub(a2) as i32) / d11;
        let f11 = div(rdf(g(SCALE_CONST)), d11 as f32);
        wr32(g(G_E10), q1 as u32);
        let q2 = (a0.wrapping_sub(a1) as i32) / d10;
        wr32(g(G_E54), a2);
        let bsub = a2.wrapping_sub(a3);
        wrf(g(G_E14), mul(sub(f(a5), f(a4)), f10));
        let t1 = mul(sub(f(a6), f(a4)), f11);
        wrf(g(G_E58), f(a4));
        wrf(g(G_E04), f10);
        wrf(g(G_E08), f11);
        wr32(g(G_E1C), a1);
        wr32(g(G_E20), a3);
        wrf(g(G_E18), t1);
        wr32(g(G_E28), q2 as u32);
        let q3 = (bsub as i32) / d11;
        let fa7 = f(a7);
        let t2 = mul(sub(f(a6), fa7), f10);
        let t3 = mul(sub(f(a5), fa7), f11);
        wrf(g(G_E24), fa7);
        wrf(g(G_E30), t2);
        wrf(g(G_E34), t3);
        wr32(g(G_E2C), q3 as u32);
        let mut eax = lf_checker_rt::callee_cdecl!(C_MAIN, u32, 0u32, g(TAB_B_ODD), g(TAB_P_ODD), g(TAB_SLOT_ODD), a10, a11);
        let mut outer: u32 = 1;
        if d11 < 1 {
            return eax;
        }
        loop {
            // Odd and even outer passes swap the two table sets.
            let (s, b, at, p, c, slot2) = if outer & 1 == 1 {
                (g(TAB_S_ODD), g(TAB_B_ODD), g(TAB_A_ODD), g(TAB_P_ODD), g(TAB_C_ODD), g(TAB_SLOT_ODD))
            } else {
                (g(TAB_B_ODD), g(TAB_S_ODD), g(TAB_P_ODD), g(TAB_A_ODD), g(TAB_SLOT_ODD), g(TAB_C_ODD))
            };
            eax = lf_checker_rt::callee_cdecl!(C_MAIN, u32, outer, s, at, c, a10, a11);
            if rd8(g(MODE_HI)) != 0 {
                eax = lf_checker_rt::callee_cdecl!(C_MODE, u32, 2u32, a10.wrapping_mul(4).wrapping_add(2));
            } else if rd8(g(MODE_LO)) == 0 {
                eax = lf_checker_rt::callee_cdecl!(C_MODE, u32, 4u32, a10.wrapping_mul(2).wrapping_add(2));
            }
            if d10 >= 0 {
                let mut inner: u32 = 0;
                loop {
                    if rd8(g(MODE_HI)) != 0 {
                        // Branch A. A2's second triple reads through
                        // [a0 + table], which is unmapped, so this branch
                        // always faults identically on both sides; A2's
                        // pushed word (read through the entry edi, which a
                        // cdecl rewrite cannot observe) is skipped in the
                        // contract. A3/A4 are implemented but unreachable.
                        let p1 = rd32(a1.wrapping_add(inner.wrapping_mul(4)));
                        let off = inner.wrapping_mul(16);
                        eax = lf_checker_rt::callee_cdecl!(C_TRIP9, u32,
                            rd32(s.wrapping_add(off)), rd32(s.wrapping_add(off).wrapping_add(4)), rd32(s.wrapping_add(off).wrapping_add(8)),
                            rd32(a0.wrapping_add(off)), rd32(a0.wrapping_add(off).wrapping_add(4)), rd32(a0.wrapping_add(off).wrapping_add(8)),
                            p1, 0u32, 0u32);
                        // A2 triple order is the original's first-fault order.
                        let q8 = rd32(a0.wrapping_add(p).wrapping_add(8));
                        let q4 = rd32(a0.wrapping_add(p).wrapping_add(4));
                        let q0a = rd32(a0.wrapping_add(p));
                        let r8 = rd32(a0.wrapping_add(b).wrapping_add(8));
                        let r4 = rd32(a0.wrapping_add(b).wrapping_add(4));
                        let r0 = rd32(a0.wrapping_add(b));
                        eax = lf_checker_rt::callee_cdecl!(C_TRIP8A, u32, r0, r4, r8, q0a, q4, q8, 0u32, 0u32);
                        if (inner as i32) >= d10 {
                            // Last inner pass skips A3/A4 (signed compare).
                        } else {
                            let off3 = inner.wrapping_add(1).wrapping_mul(16);
                            let p3 = rd32(slot2.wrapping_add(inner.wrapping_mul(4)).wrapping_add(4));
                            eax = lf_checker_rt::callee_cdecl!(C_TRIP9, u32,
                                rd32(b.wrapping_add(off3)), rd32(b.wrapping_add(off3).wrapping_add(4)), rd32(b.wrapping_add(off3).wrapping_add(8)),
                                rd32(p.wrapping_add(off3)), rd32(p.wrapping_add(off3).wrapping_add(4)), rd32(p.wrapping_add(off3).wrapping_add(8)),
                                p3, 0u32, 0u32);
                            let p4 = rd32(a0.wrapping_add(inner.wrapping_mul(4)));
                            eax = lf_checker_rt::callee_cdecl!(C_TRIP8A, u32,
                                rd32(s.wrapping_add(off)), rd32(s.wrapping_add(off).wrapping_add(4)), rd32(s.wrapping_add(off).wrapping_add(8)),
                                rd32(a0.wrapping_add(off)), rd32(a0.wrapping_add(off).wrapping_add(4)), rd32(a0.wrapping_add(off).wrapping_add(8)),
                                p4, 0u32);
                        }
                    } else if rd8(g(MODE_LO)) != 0 {
                        // Branch C: copy 9-dword records into the record
                        // buffer. `cnt` is the scratched a3 argument slot.
                        let mut cnt: u32;
                        let est = (rd32(g(REC_COUNT)) as i32)
                            .wrapping_add(a10.wrapping_mul(2) as i32)
                            .wrapping_add(2);
                        if est >= REC_CAP {
                            eax = lf_checker_rt::callee_cdecl!(C_FLUSH, u32,);
                        }
                        let obj = rd32(g(REC_OBJ));
                        let base = if rd8(obj.wrapping_add(6)) == 0 {
                            0
                        } else {
                            rd32(obj.wrapping_add(8))
                        };
                        let t = rd32(g(REC_COUNT));
                        let rec = base.wrapping_add(t.wrapping_add(t.wrapping_mul(8)).wrapping_mul(4));
                        let o16 = inner.wrapping_mul(16);
                        wr32(rec, rd32(b.wrapping_add(o16)));
                        wr32(rec.wrapping_add(4), rd32(b.wrapping_add(o16).wrapping_add(4)));
                        wr32(rec.wrapping_add(8), rd32(b.wrapping_add(o16).wrapping_add(8)));
                        wr32(rec.wrapping_add(0x0c), rd32(p.wrapping_add(o16)));
                        wr32(rec.wrapping_add(0x10), rd32(p.wrapping_add(o16).wrapping_add(4)));
                        wr32(rec.wrapping_add(0x14), rd32(p.wrapping_add(o16).wrapping_add(8)));
                        let mut cur = rec.wrapping_add(0x24);
                        wr32(cur.wrapping_sub(0x0c), rd32(slot2.wrapping_add(inner.wrapping_mul(4))));
                        wr32(cur.wrapping_sub(8), 0);
                        wr32(cur.wrapping_sub(4), 0);
                        cnt = 1;
                        if outer == 1 && inner == 0 {
                            wr32(cur, rd32(b));
                            wr32(cur.wrapping_add(4), rd32(b.wrapping_add(4)));
                            wr32(cur.wrapping_add(8), rd32(b.wrapping_add(8)));
                            wr32(cur.wrapping_add(0x0c), rd32(p));
                            wr32(cur.wrapping_add(0x10), rd32(p.wrapping_add(4)));
                            wr32(cur.wrapping_add(0x14), rd32(p.wrapping_add(8)));
                            let second = rd32(slot2);
                            cur = cur.wrapping_add(0x24);
                            wr32(cur.wrapping_sub(0x0c), second);
                            wr32(cur.wrapping_sub(8), inner);
                            wr32(cur.wrapping_sub(4), inner);
                            cnt = 2;
                        }
                        wr32(cur, rd32(s.wrapping_add(o16)));
                        wr32(cur.wrapping_add(4), rd32(s.wrapping_add(o16).wrapping_add(4)));
                        wr32(cur.wrapping_add(8), rd32(s.wrapping_add(o16).wrapping_add(8)));
                        cnt = cnt.wrapping_add(1);
                        wr32(cur.wrapping_add(0x0c), rd32(a0.wrapping_add(o16)));
                        wr32(cur.wrapping_add(0x10), rd32(a0.wrapping_add(o16).wrapping_add(4)));
                        wr32(cur.wrapping_add(0x14), rd32(a0.wrapping_add(o16).wrapping_add(8)));
                        wr32(cur.wrapping_add(0x18), rd32(a1.wrapping_add(inner.wrapping_mul(4))));
                        wr32(cur.wrapping_add(0x1c), 0);
                        wr32(cur.wrapping_add(0x20), 0);
                        if outer == a11.wrapping_sub(1) && inner == (a10.wrapping_sub(1)) {
                            let nxt = cur.wrapping_add(0x24);
                            wr32(nxt, rd32(s.wrapping_add(o16)));
                            wr32(nxt.wrapping_add(4), rd32(s.wrapping_add(o16).wrapping_add(4)));
                            wr32(nxt.wrapping_add(8), rd32(s.wrapping_add(o16).wrapping_add(8)));
                            cnt = cnt.wrapping_add(1);
                            wr32(nxt.wrapping_add(0x0c), rd32(a0.wrapping_add(o16)));
                            wr32(nxt.wrapping_add(0x10), rd32(a0.wrapping_add(o16).wrapping_add(4)));
                            wr32(nxt.wrapping_add(0x14), rd32(a0.wrapping_add(o16).wrapping_add(8)));
                            wr32(nxt.wrapping_add(0x18), rd32(a1.wrapping_add(inner.wrapping_mul(4))));
                            wr32(nxt.wrapping_add(0x1c), 0);
                            wr32(nxt.wrapping_add(0x20), 0);
                        }
                        eax = t.wrapping_add(cnt);
                        wr32(g(REC_COUNT), eax);
                        if (eax as i32) >= REC_CAP {
                            eax = lf_checker_rt::callee_cdecl!(C_FLUSH, u32,);
                        }
                    } else {
                        // Branch B. The second triple base is the
                        // uninitialised prologue scratch word, transported
                        // here as `a8` (the contract fills both with the
                        // heap base).
                        let pb1 = rd32(slot2.wrapping_add(inner.wrapping_mul(4)));
                        let o16 = inner.wrapping_mul(16);
                        eax = lf_checker_rt::callee_cdecl!(C_TRIP9, u32,
                            rd32(b.wrapping_add(o16)), rd32(b.wrapping_add(o16).wrapping_add(4)), rd32(b.wrapping_add(o16).wrapping_add(8)),
                            rd32(p.wrapping_add(o16)), rd32(p.wrapping_add(o16).wrapping_add(4)), rd32(p.wrapping_add(o16).wrapping_add(8)),
                            pb1, 0u32, 0u32);
                        let pb2 = rd32(a1.wrapping_add(inner.wrapping_mul(4)));
                        eax = lf_checker_rt::callee_cdecl!(C_TRIP8B, u32,
                            rd32(fill_base.wrapping_add(o16)), rd32(fill_base.wrapping_add(o16).wrapping_add(4)), rd32(fill_base.wrapping_add(o16).wrapping_add(8)),
                            rd32(a0.wrapping_add(o16)), rd32(a0.wrapping_add(o16).wrapping_add(4)), rd32(a0.wrapping_add(o16).wrapping_add(8)),
                            pb2, 0u32);
                    }
                    inner = inner.wrapping_add(1);
                    if (inner as i32) > d10 {
                        break;
                    }
                }
            }
            if rd8(g(MODE_HI)) == 0 && rd8(g(MODE_LO)) != 0 {
                // Branch-C trials skip the tail call.
            } else {
                eax = lf_checker_rt::callee_cdecl!(C_TAIL, u32,);
            }
            outer = outer.wrapping_add(1);
            if (outer as i32) > d11 {
                break;
            }
        }
        eax
    }
});
