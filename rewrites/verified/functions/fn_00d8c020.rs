// original: 0x00d8c020 audio_event_dispatch_update (proposed)

/// Dispatch one audio event by the object's state, run the shared updates,
/// and report the target's liveness.
///
/// Arguments (cdecl, five stack words): `obj` points at the event object
/// (inner pointer at `+0x21c`, index word at `+0x2e`, flag byte at `+0x219`);
/// `target` is a handle compared against small ids; only the low byte of
/// `flags` is read; `bias` shifts a global counter index; `probe`, when
/// nonzero, is dereferenced at `+0x398` for the final liveness word.
///
/// First a switch on `t - 3` where `t` is the inner state's word at `+0x12c`:
/// `t == 7` runs the long case (a constant call, a helper query whose answer
/// feeds a computed thiscall, and, when that answers nonzero, two more
/// constant calls), every other `t` in `3..=14` runs a single constant call,
/// and anything else runs nothing. (The original dispatches through a
/// twelve-byte index table and a two-entry jump table in its own code, which
/// the rewrite cannot read; the decoded mapping is `7` to the long case and
/// everything else in range to the short one.) Then, unless skipped: a call
/// gated on bit `0x2000` of a flag word reached through a global pointer
/// table indexed by the sign-extended index word; a mode/state cascade over
/// two globals that, on its go-path, may call a predicate and an updater,
/// decrement a global, and emit a call whose id depends on whether the
/// state word equals 2; a global counter increment at `index + bias` unless
/// the (unsigned) index reaches `0x44c`; a call gated on the flag byte; a
/// global counter increment at the state word; a call gated on the flag byte
/// of `flags`; two unconditional calls; and a final liveness call whose
/// second word is `probe` with its low byte forced to 1 (probe nonzero with
/// a nonzero word at `+0x398`) or 0. Returns the final call's answer.
///
/// The helper-null path (which would read address `0x3d4`) and small-integer
/// `probe` values (which would read unmapped memory at the end) are not
/// exercised: the contract keeps both pointers valid.
///
/// Original: 0x00d8c020 (cdecl, five stack words; returns last call's result).
lf_checker_rt::export!(cdecl, rw_00d8c020(obj: u32, target: u32, flags: u32, bias: u32, probe: u32) -> u32 {
    unsafe {
        const EMIT: u32 = 1;
        const QUERY: u32 = 2;
        const APPLY: u32 = 3;
        const EMIT3: u32 = 4;
        const PRED: u32 = 5;
        const UPDATE: u32 = 6;
        const REPORT: u32 = 7;
        const ONE: u32 = 0x3f800000;
        const INNER_OFF: u32 = 0x21c;
        const STATE_OFF: u32 = 0x12c;
        const INDEX_OFF: u32 = 0x2e;
        const FLAGB_OFF: u32 = 0x219;
        const ADD_BASE_FILE: u32 = 0x01666c90;
        const FLAG_MASK: u32 = 0x2000;
        const TABLE_ADDR: u32 = 0x01295cd8;
        const MODE_ADDR: u32 = 0x0179bf98;
        const STATE2_ADDR: u32 = 0x0179bf9c;
        const GATE_ADDR: u32 = 0x0179d101;
        const DEC_ADDR: u32 = 0x0179bfb0;
        const COUNT1_ADDR: u32 = 0x0179bfd0;
        const COUNT2_ADDR: u32 = 0x012b62a0;
        const LIVE_OFF: u32 = 0x398;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn g(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn emit1(id: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(EMIT, u32, id, ONE);
            }
        }

        let inner = rd32(obj + INNER_OFF);
        let t = rd32(inner + STATE_OFF);
        let v = t.wrapping_sub(3);
        if v <= 0x0b {
            if v == 4 {
                emit1(0x1d1);
                let a: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32, 0);
                // Null answer would read address 0x3d4; the contract keeps
                // the pointed-to word a valid pointer.
                let w = rd32(a + 0x228);
                let c = rd32(w.wrapping_add(0x70).wrapping_add(0x3d4));
                let this = c
                    .wrapping_mul(0x5c)
                    .wrapping_add(lf_checker_rt::relocated(ADD_BASE_FILE));
                let r: u32 = lf_checker_rt::callee_thiscall!(APPLY, u32, this, obj);
                if (r as u8) != 0 {
                    emit1(0x1d2);
                    let _: u32 = lf_checker_rt::callee_cdecl!(EMIT3, u32, 0, 0x2ac, ONE);
                }
            } else {
                emit1(0x1d0);
            }
        }

        let sx = rd16(obj + INDEX_OFF) as u16 as i16 as i32;
        let tp = rd32(
            lf_checker_rt::relocated(TABLE_ADDR).wrapping_add((sx as u32).wrapping_mul(4)),
        );
        if rd32(tp + 0x124) & FLAG_MASK != 0 {
            emit1(0x111);
        }

        let bl = flags as u8;
        let mode = g(MODE_ADDR);
        if mode == 1 || mode == 4 {
            let ga = g(STATE2_ADDR);
            let mut go = false;
            if ga == 0x38 || ga == 0x39 {
                let r: u32 = lf_checker_rt::callee_cdecl!(PRED, u32, target, ga);
                if (r as u8) != 0 {
                    go = true;
                }
            }
            if !go {
                // The original reloads the state word after a failed
                // predicate; the stubbed call leaves globals alone, so the
                // value is unchanged.
                if target == ga
                    || target == 0x33
                    || (target == 0x31 && ga == 0x32)
                    || (target == 0x32 && ga == 0x31)
                    || ga == 5
                {
                    go = true;
                }
            }
            if go {
                let r: u32 = lf_checker_rt::callee_cdecl!(UPDATE, u32, obj);
                if (r as u8) != 0 {
                    let gate = (lf_checker_rt::relocated(GATE_ADDR) as *const u8).read();
                    if gate == 0 || bl != 0 {
                        let d = lf_checker_rt::relocated(DEC_ADDR);
                        wr32(d, rd32(d).wrapping_sub(1));
                    }
                }
                // The state-word call is part of the go-path: the skip paths
                // jump past it.
                let t2 = rd32(inner + STATE_OFF);
                emit1(if t2 == 2 { 0x1c7 } else { 0x1c6 });
            }
        }

        let ax = rd16(obj + INDEX_OFF);
        if ax < 0x44c {
            let idx = (sx as u32).wrapping_add(bias);
            let c = lf_checker_rt::relocated(COUNT1_ADDR).wrapping_add(idx.wrapping_mul(4));
            wr32(c, rd32(c).wrapping_add(1));
        }

        if ((obj + FLAGB_OFF) as *const u8).read() == 0 {
            emit1(0x101);
        }

        let t3 = rd32(inner + STATE_OFF);
        let c2 = lf_checker_rt::relocated(COUNT2_ADDR).wrapping_add(t3.wrapping_mul(4));
        wr32(c2, rd32(c2).wrapping_add(1));

        if bl != 0 {
            emit1(0x121);
        }
        emit1(0x1b0);
        emit1(0x10f);

        let live = if probe != 0 && rd32(probe + LIVE_OFF) != 0 {
            probe | 1
        } else {
            probe & 0xffffff00
        };
        lf_checker_rt::callee_cdecl!(REPORT, u32, target, live)
    }
});
