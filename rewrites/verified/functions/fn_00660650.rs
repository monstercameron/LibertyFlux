// original: 0x00660650 rage::snAddRemoteGamerTask::vf7

/// Run one step of the add-remote-gamer task, then chain to the base step.
///
/// `this` is the task, `a0` selects the path (`a1` feeds one store). On
/// the `a0 == 1` path the manager at `this+0x60` looks up the key at
/// `this+0xe0` (callee 1); a miss, or a record already marked (byte
/// `+0x80`, bit 0), ends the step. Otherwise the record's 16 bytes at
/// `+0x48` go to the registrar (callee 2, as four words) on the manager,
/// the record is marked, and the step ends.
///
/// On the other path the `+0x544` state decides: 1, or 3 and above, tears
/// down the box at `this+0x538` through its virtual slot `+0x1c`
/// (callee 3) and the detacher (callee 4) on the manager's `+0x32e0`
/// object; below 1, or exactly 2, skips that. When the state at
/// `this+0x94` and the box at `this+0x530` are both 1, an interlocked
/// compare-exchange (callee 5: expect 1, set 2) races the box, and the
/// winner stores `a1` into the box's second word. The session pair
/// `this+0xd8`/`+0xdc` is then resolved (callee 6a): a miss with state 2
/// and a non-negative `this+0x818` value reports it (callee 10) on the
/// manager's `+0x24` object. A hit whose record word `+0x6c` is clear
/// checks readiness (callee 7): ready runs the handshake (callee 8) with
/// the session pair, otherwise the session is resolved again (callee 6b)
/// and a hit there runs the opener (callee 9). The session is resolved a
/// third time (callee 6c) with the pair — except on the handshake path,
/// where the original reads its incoming saved register slot instead of
/// the second word, so the contract fixes that register and the rewrite
/// passes the same constant — and a hit with `this+0x90` set and record
/// word `+0x6c` clear checks readiness once more: ready runs the second
/// handshake (callee 8 again), otherwise the closer (callee 11) runs
/// with the pair. Every path ends by chaining to the base step
/// (callee 12) with `(a0, a1)` and clearing `this+0x60`.
///
/// Callees 6a, 6b and 6c share one target, patched per site so the
/// handshake path's register slot stays visible. The handshake, second
/// session and closer calls take whatever the readiness check left in
/// its object register (never set by the original), so the contract
/// compares their stack words but not that register.
///
/// Original: 0x00660650 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00660650(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x60;
        const KEY: u32 = 0xe0;
        const BOX: u32 = 0x538;
        const STATE544: u32 = 0x544;
        const STATE94: u32 = 0x94;
        const BOX530: u32 = 0x530;
        const SESS0: u32 = 0xd8;
        const SESS1: u32 = 0xdc;
        const FLAG90: u32 = 0x90;
        const VAL818: u32 = 0x818;
        const ESI_TRIAL: u32 = 0x51e5_1e51;
        const C_LOOKUP: u32 = 1;
        const C_REG16: u32 = 2;
        const C_VT1C: u32 = 3;
        const C_DETACH: u32 = 4;
        const C_CMPXCHG: u32 = 5;
        const C_SESS_A: u32 = 6;
        const C_READY: u32 = 7;
        const C_HAND: u32 = 8;
        const C_SESS_B: u32 = 9;
        const C_OPEN: u32 = 10;
        const C_REPORT: u32 = 11;
        const C_SESS_C: u32 = 12;
        const C_CLOSE: u32 = 13;
        const C_BASE: u32 = 14;

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

        let mgr = rd32(this.wrapping_add(MGR));
        if a0 == 1 {
            let rec: u32 =
                lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, this.wrapping_add(KEY));
            if rec != 0 && rd8(rec.wrapping_add(0x80)) & 1 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_REG16,
                    u32,
                    mgr,
                    rd32(rec.wrapping_add(0x48)),
                    rd32(rec.wrapping_add(0x4c)),
                    rd32(rec.wrapping_add(0x50)),
                    rd32(rec.wrapping_add(0x54))
                );
                let mark = rec.wrapping_add(0x80) as *mut u8;
                mark.write(mark.read() | 1);
            }
        } else {
            let st = rd32(this.wrapping_add(STATE544)) as i32;
            if st >= 1 && st != 2 {
                let obj = this.wrapping_add(BOX);
                let slot = rd32(rd32(obj).wrapping_add(0x1c));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let _ = C_VT1C;
                let _: u32 = f(obj, 0, 0);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_DETACH,
                    u32,
                    mgr.wrapping_add(0x32e0),
                    obj
                );
            }
            if rd32(this.wrapping_add(STATE94)) == 1
                && rd32(this.wrapping_add(BOX530)) == 1
            {
                let won: u32 = lf_checker_rt::callee_stdcall!(
                    C_CMPXCHG,
                    u32,
                    this.wrapping_add(BOX530),
                    2,
                    1
                );
                if won == 1 {
                    wr32(this.wrapping_add(BOX530).wrapping_add(4), a1);
                }
            }
            let d0 = rd32(this.wrapping_add(SESS0));
            let d1 = rd32(this.wrapping_add(SESS1));
            let mut d1_for_c = d1;
            let sess: u32 =
                lf_checker_rt::callee_thiscall!(C_SESS_A, u32, mgr, d0, d1);
            if sess == 0 {
                if st == 2 {
                    let v = rd32(this.wrapping_add(VAL818)) as i32;
                    if v >= 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            C_REPORT,
                            u32,
                            rd32(mgr.wrapping_add(0x24)),
                            v as u32,
                            1
                        );
                    }
                }
            } else if rd32(sess.wrapping_add(0x6c)) == 0 {
                let ready: u32 = lf_checker_rt::callee_thiscall!(C_READY, u32, mgr);
                if ready & 0xff != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(C_HAND, u32, mgr, d0, d1);
                    d1_for_c = ESI_TRIAL;
                } else {
                    let again: u32 =
                        lf_checker_rt::callee_thiscall!(C_SESS_B, u32, mgr, d0, d1);
                    if again != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(C_OPEN, u32, mgr, again);
                    }
                }
            }
            let sess2: u32 =
                lf_checker_rt::callee_thiscall!(C_SESS_C, u32, mgr, d0, d1_for_c);
            if sess2 != 0
                && rd8(this.wrapping_add(FLAG90)) != 0
                && rd32(sess2.wrapping_add(0x6c)) == 0
            {
                let ready: u32 = lf_checker_rt::callee_thiscall!(C_READY, u32, mgr);
                if ready & 0xff != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(C_HAND, u32, mgr, d0, d1);
                } else {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(C_CLOSE, u32, mgr, d0, d1);
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_BASE, u32, this, a0, a1);
        wr32(this.wrapping_add(MGR), 0);
        0
    }
});
