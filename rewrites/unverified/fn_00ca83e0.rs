// original: 0x00CA83E0 CEventHandler::vf11

/// Decide and build this handler's reaction from two facing dots, a mode
/// pair and a slot scan, with a float gauntlet for the close-range modes.
///
/// `this` is the handler (its ped at `+0x04`, the built reaction stored at
/// `+0x0c`). `a1` carries the subject at `+0x14` and a mode pair: two
/// sign-extended 16-bit modes at `+0x40`/`+0x42`. The other two stack words
/// are never read.
///
/// Two facing dots are formed from the subject's position block (at subject
/// `+0x20`) and the handler ped's (at ped `+0x20`): the difference of the
/// words at `+0x30`/`+0x34`/`+0x38`, dotted against the handler block's
/// `+0x10`/`+0x14`/`+0x18` (first dot) and the subject block's (second dot),
/// in the original's exact operation order. Flag A is set when the first dot
/// is ordered greater-or-equal to zero, flag B when zero is ordered
/// greater-or-equal to the second dot (an unordered NaN clears either flag).
/// Flag C is set only when a mode global equals 2 and bit 22 of the word at
/// ped `+0x2A0` is set.
///
/// Five slots at subject-link `+0x44` (the link at subject `+0x224`) are
/// scanned twice for the first nonzero entry (the two scans always agree);
/// when one exists, slot `0x0C` of its object is called and an answer of
/// `0x391` aborts with 0.
///
/// The mode pair then dispatches: (2,2) runs the direct chain; (3 or 4,
/// 2/3/4) checks flag A and then either joins the chain past its flag-A
/// check (flag B clear) or runs the gauntlet (flag B set); (0 or 1, 2/3/4)
/// needs flag B and runs float block 1; (3 or 4, anything else) and the
/// remaining pairs fall through to a check that joins the chain for mode
/// pairs (3 or 4, 0 or 1) and aborts otherwise; (2, not 2) joins that same
/// check.
///
/// The direct chain needs flag A set and flag C clear, resolves an identity
/// (which must differ from the subject), runs a probe whose low byte must be
/// zero and equal the byte at subject `+0x211`, runs an unanswered check,
/// then builds: with a factory object the reaction is made from the subject
/// (or 0 without one), passed with 4 through a gate call, and the gate's
/// answer is stored.
///
/// The gauntlet compares two table floats against 1.5: the words at `+0xF8`
/// of the table entries indexed by the signed words at ped `+0x2E` and
/// subject `+0x2E` (the table is game-filled data, seeded by the contract).
/// A first float at or below 1.5, or two floats above 1.5 with the first not
/// above the second, run float block 2 (which needs mode 4); otherwise float
/// block 3 runs. (A repeated first-float compare on the low-second-float leg
/// always falls through, so that leg always runs block 3.)
///
/// Each float block pushes nine words (handler ped, the subject block's
/// `+0x38` word, the constant 0.25, six zeros) with a frame pointer, runs a
/// setup call whose answer is ignored, then a second call from the subject
/// block's `+0x30` address whose answer becomes the new subject; block 1
/// aborts when the saved second mode is 2. Blocks 1 and 2 then build through
/// one maker, block 3 through another (with an extra 0 word); a missing
/// factory aborts with 0. Every exit stores the answer (or 0) at `+0x0c`,
/// runs the register-preserving cookie check and returns the answer.
///
/// Original: 0x00CA83E0 (thiscall, ecx = handler, three stack words of which
/// only the first is read. A null subject returns the esp-derived cookie, so
/// the contract always provides one.)
lf_checker_rt::export!(thiscall, rw_00ca83e0(this: u32, a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const A1_SUBJ: u32 = 0x14;
        const A1_CX: u32 = 0x40;
        const A1_DX: u32 = 0x42;
        const H_PED: u32 = 0x04;
        const PED_POS: u32 = 0x20;
        const SUBJ_POS: u32 = 0x20;
        const SUBJ_LINK: u32 = 0x224;
        const SLOT_BASE: u32 = 0x44;
        const SLOT_COUNT: u32 = 5;
        const SLOT_VSLOT: u32 = 0x0C;
        const ABORT_ANSWER: u32 = 0x391;
        const G_MODE: u32 = 0x011D_6FD4;
        const MODE_WANT: u32 = 2;
        const PED_BITWORD: u32 = 0x2A0;
        const BIT_SHIFT: u32 = 22;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        let esi = rd32(a1.wrapping_add(A1_SUBJ));
        let hped = rd32(this.wrapping_add(H_PED));
        let sp = rd32(esi.wrapping_add(SUBJ_POS));
        let hp = rd32(hped.wrapping_add(PED_POS));
        let d5 = sub(rdf(sp.wrapping_add(0x30)), rdf(hp.wrapping_add(0x30)));
        let d3 = sub(rdf(sp.wrapping_add(0x34)), rdf(hp.wrapping_add(0x34)));
        let d4 = sub(rdf(sp.wrapping_add(0x38)), rdf(hp.wrapping_add(0x38)));
        let d1 = add(
            add(mul(rdf(hp.wrapping_add(0x14)), d3), mul(rdf(hp.wrapping_add(0x10)), d5)),
            mul(rdf(hp.wrapping_add(0x18)), d4),
        );
        let b16 = d1 >= 0.0;
        let d2 = add(
            add(mul(rdf(sp.wrapping_add(0x14)), d3), mul(rdf(sp.wrapping_add(0x10)), d5)),
            mul(rdf(sp.wrapping_add(0x18)), d4),
        );
        let b17 = 0.0 >= d2;
        let cx = rd16s(a1.wrapping_add(A1_CX));
        let dx = rd16s(a1.wrapping_add(A1_DX));
        let b15 = if rd32(lf_checker_rt::relocated(G_MODE)) == MODE_WANT {
            ((rd32(hped.wrapping_add(PED_BITWORD)) >> BIT_SHIFT) & 1) as u8
        } else {
            0
        };
        let mid = rd32(esi.wrapping_add(SUBJ_LINK));
        let mid2 = rd32(hped.wrapping_add(SUBJ_LINK));
        // First scan: is any slot nonzero?
        let mut i = 0u32;
        let mut hit = false;
        while i < SLOT_COUNT {
            if rd32(mid.wrapping_add(i.wrapping_mul(4)).wrapping_add(SLOT_BASE)) != 0 {
                hit = true;
                break;
            }
            i += 1;
        }
        if hit {
            // Second scan: call the first nonzero slot's object.
            let mut j = 0u32;
            let mut done = false;
            while j < SLOT_COUNT {
                let v = rd32(mid.wrapping_add(j.wrapping_mul(4)).wrapping_add(SLOT_BASE));
                if v != 0 {
                    let vt = rd32(v);
                    let probe: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(vt.wrapping_add(SLOT_VSLOT)) as usize);
                    if probe(v) == ABORT_ANSWER {
                        return ca83e0_epilogue(this, 0);
                    }
                    done = true;
                    break;
                }
                j += 1;
            }
            if !done {
                rd32(0); // faithful: the original reads [0] here; unreachable.
            }
        }
        ca83e0_dispatch(this, esi, hped, sp, mid2, cx, dx, b15, b16, b17)
    }
});

/// Store-and-cookie-check exit shared by every path of rw_00ca83e0: the
/// original stores its answer at +0x0c on every exit, even the aborts.
unsafe fn ca83e0_epilogue(this: u32, v: u32) -> u32 {
    unsafe {
        (this.wrapping_add(0x0C) as *mut u32).write_unaligned(v);
        lf_checker_rt::callee_stdcall!(12, u32,);
        v
    }
}

/// The nine-word setup call plus the subject-refresh call of rw_00ca83e0's
/// float blocks. Returns the refreshed subject.
unsafe fn ca83e0_block(hped: u32, sp: u32) -> u32 {
    unsafe {
        const C_QUARTER: u32 = 0x00E9_D494;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let fa = rd32(sp.wrapping_add(0x38));
        let fb = rd32(lf_checker_rt::relocated(C_QUARTER));
        let mut frame = [0u32; 2];
        let _setup: u32 = lf_checker_rt::callee_thiscall!(
            7, u32, frame.as_mut_ptr() as u32, hped, fa, fb, 0, 0, 0, 0, 0, 0
        );
        let mut frame2 = [0u32; 1];
        lf_checker_rt::callee_thiscall!(8, u32, frame2.as_mut_ptr() as u32, sp.wrapping_add(0x30))
    }
}

/// Factory fetch of rw_00ca83e0.
unsafe fn ca83e0_factory() -> u32 {
    unsafe {
        const G_FACTORY: u32 = 0x0167_E2A0;
        lf_checker_rt::callee_thiscall!(
            5, u32, (lf_checker_rt::relocated(G_FACTORY) as *const u32).read()
        )
    }
}

/// Direct chain of rw_00ca83e0 past its flag-A check.
unsafe fn ca83e0_chain_tail(this: u32, esi: u32, hped: u32, mid2: u32, b15: u8) -> u32 {
    unsafe {
        const SUBJ_MARK: u32 = 0x211;
        const GATE_OFF: u32 = 0x44;
        if b15 != 0 {
            return ca83e0_epilogue(this, 0);
        }
        let ident: u32 = lf_checker_rt::callee_thiscall!(2, u32, hped);
        if ident == esi {
            return ca83e0_epilogue(this, 0);
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(3, u32, mid2, esi);
        if probe & 0xFF != 0 {
            return ca83e0_epilogue(this, 0);
        }
        if (esi.wrapping_add(SUBJ_MARK) as *const u8).read() != probe as u8 {
            return ca83e0_epilogue(this, 0);
        }
        let _check: u32 = lf_checker_rt::callee_thiscall!(4, u32, hped, esi);
        let fac = ca83e0_factory();
        // Both legs converge on the gate call: the made reaction, or 0 when
        // the factory is missing.
        let made = if fac == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(6, u32, fac, esi)
        };
        let gated: u32 =
            lf_checker_rt::callee_thiscall!(11, u32, mid2.wrapping_add(GATE_OFF), made, 4);
        ca83e0_epilogue(this, gated)
    }
}

/// Direct chain of rw_00ca83e0 with its flag-A check.
unsafe fn ca83e0_chain_full(
    this: u32, esi: u32, hped: u32, mid2: u32, b15: u8, b16: bool,
) -> u32 {
    unsafe {
        if !b16 {
            return ca83e0_epilogue(this, 0);
        }
        ca83e0_chain_tail(this, esi, hped, mid2, b15)
    }
}

/// Float block 1 follow-through of rw_00ca83e0 (needs flag B; aborts when
/// the saved second mode is 2).
unsafe fn ca83e0_block1(this: u32, hped: u32, sp: u32, dx: i32, b17: bool) -> u32 {
    unsafe {
        if !b17 {
            return ca83e0_epilogue(this, 0);
        }
        let subj = ca83e0_block(hped, sp);
        if dx == 2 {
            return ca83e0_epilogue(this, 0);
        }
        let fac = ca83e0_factory();
        if fac == 0 {
            return ca83e0_epilogue(this, 0);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(9, u32, fac, subj);
        ca83e0_epilogue(this, made)
    }
}

/// Float block 2 follow-through of rw_00ca83e0.
unsafe fn ca83e0_block2(this: u32, hped: u32, sp: u32) -> u32 {
    unsafe {
        let subj = ca83e0_block(hped, sp);
        let fac = ca83e0_factory();
        if fac == 0 {
            return ca83e0_epilogue(this, 0);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(9, u32, fac, subj);
        ca83e0_epilogue(this, made)
    }
}

/// Float block 3 follow-through of rw_00ca83e0.
unsafe fn ca83e0_block3(this: u32, hped: u32, sp: u32) -> u32 {
    unsafe {
        let subj = ca83e0_block(hped, sp);
        let fac = ca83e0_factory();
        if fac == 0 {
            return ca83e0_epilogue(this, 0);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(10, u32, fac, subj, 0);
        ca83e0_epilogue(this, made)
    }
}

/// Gauntlet exit of rw_00ca83e0: block 2 needs mode 4.
unsafe fn ca83e0_l86cd(this: u32, hped: u32, sp: u32, cx: i32) -> u32 {
    unsafe {
        if cx != 4 {
            return ca83e0_epilogue(this, 0);
        }
        ca83e0_block2(this, hped, sp)
    }
}

/// Float gauntlet of rw_00ca83e0.
unsafe fn ca83e0_gauntlet(this: u32, esi: u32, hped: u32, sp: u32, cx: i32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5CD8;
        const K_LIMIT: u32 = 0x00EA_D644;
        const ENTRY_SCORE: u32 = 0xF8;
        const H_INDEX: u32 = 0x2E;
        const E_INDEX: u32 = 0x2E;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        let base = lf_checker_rt::relocated(TABLE);
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(K_LIMIT)));
        let ih = rd16s(hped.wrapping_add(H_INDEX)) as u32;
        let t0 = f32::from_bits(rd32(rd32(base.wrapping_add(ih.wrapping_mul(4))).wrapping_add(ENTRY_SCORE)));
        if !(t0 > k) {
            return ca83e0_l86cd(this, hped, sp, cx);
        }
        let ie = rd16s(esi.wrapping_add(E_INDEX)) as u32;
        let t1 = f32::from_bits(rd32(rd32(base.wrapping_add(ie.wrapping_mul(4))).wrapping_add(ENTRY_SCORE)));
        // The low-second-float leg re-checks the first float, which is still
        // above the limit, so it always reaches block 3; written directly.
        if !(t1 > k) {
            return ca83e0_block3(this, hped, sp);
        }
        if t0 > t1 {
            return ca83e0_block3(this, hped, sp);
        }
        ca83e0_l86cd(this, hped, sp, cx)
    }
}

/// Close-range entry of rw_00ca83e0.
unsafe fn ca83e0_l866d(
    this: u32, esi: u32, hped: u32, sp: u32, mid2: u32, cx: i32, dx: i32, b15: u8,
    b16: bool, b17: bool,
) -> u32 {
    unsafe {
        if !b16 {
            return ca83e0_epilogue(this, 0);
        }
        if !b17 {
            return ca83e0_chain_tail(this, esi, hped, mid2, b15);
        }
        if dx == 2 {
            return ca83e0_l86cd(this, hped, sp, cx);
        }
        ca83e0_gauntlet(this, esi, hped, sp, cx)
    }
}

/// (3-or-4, 0-or-1) join of rw_00ca83e0.
unsafe fn ca83e0_l8657(
    this: u32, esi: u32, hped: u32, mid2: u32, dx: i32, b15: u8, b16: bool,
) -> u32 {
    unsafe {
        if dx == 1 || dx == 0 {
            return ca83e0_chain_full(this, esi, hped, mid2, b15, b16);
        }
        ca83e0_epilogue(this, 0)
    }
}

/// Fall-through join of rw_00ca83e0.
unsafe fn ca83e0_l8649(
    this: u32, esi: u32, hped: u32, mid2: u32, cx: i32, dx: i32, b15: u8, b16: bool,
) -> u32 {
    unsafe {
        if cx == 3 || cx == 4 {
            return ca83e0_l8657(this, esi, hped, mid2, dx, b15, b16);
        }
        ca83e0_epilogue(this, 0)
    }
}

/// Mode-pair dispatch of rw_00ca83e0.
unsafe fn ca83e0_dispatch(
    this: u32, esi: u32, hped: u32, sp: u32, mid2: u32, cx: i32, dx: i32, b15: u8,
    b16: bool, b17: bool,
) -> u32 {
    unsafe {
        if cx == 2 {
            if dx == 2 {
                return ca83e0_chain_full(this, esi, hped, mid2, b15, b16);
            }
            return ca83e0_l8657(this, esi, hped, mid2, dx, b15, b16);
        }
        if cx == 3 || cx == 4 {
            if dx == 2 || dx == 3 || dx == 4 {
                return ca83e0_l866d(this, esi, hped, sp, mid2, cx, dx, b15, b16, b17);
            }
            return ca83e0_l8649(this, esi, hped, mid2, cx, dx, b15, b16);
        }
        if cx == 1 || cx == 0 {
            if dx == 2 || dx == 3 || dx == 4 {
                return ca83e0_block1(this, hped, sp, dx, b17);
            }
            return ca83e0_l8649(this, esi, hped, mid2, cx, dx, b15, b16);
        }
        ca83e0_l8649(this, esi, hped, mid2, cx, dx, b15, b16)
    }
}
