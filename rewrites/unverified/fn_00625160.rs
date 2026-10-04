// original: 0x00625160 net_session_announce (proposed)

/// Announce session state: refresh per-channel publishers, then report the
/// session tag through the host object.
///
/// `this` is the session object: `STATE` at `+0x50`, flag bytes for five
/// channels at `+CH0`..`+CH4` (each a small record, `CH2` doubling as a
/// NUL-terminated tag string), the session id pair in the scratch area, and
/// argument words `a0` (selector, only 0 runs the main path), `a1`/`a2`
/// (flag bytes). No return value.
///
/// With selector 0 and the enable bit (`FEAT` bit 1) set: compute a mode word
/// (1 when the state reads 2 or 3, `MODE_GATE` reads 1 and `MODE_ZERO` reads
/// 0), initialise two scratch records, and refresh the `+0xd0` record unless
/// suppressed by `a1`, the mode word, or the state. When the state reads 2
/// or 3, `JOIN_GATE` reads 1, the id pair is nonzero and the accumulator
/// pair is nonzero, each present channel is published through its fast
/// hook; otherwise each present channel is withdrawn through the host
/// object's slot 1, with `CH2` getting a scratch re-initialisation and its
/// fast hook first. The host object comes from `HOST_PTR` (a null host
/// skips the withdrawal).
///
/// Then, unless `HOST_GATE` is clear: when the tag is empty or equals the
/// well-known info tag, take the cold path, otherwise compare it against the
/// configured tag (`CFG_TAG`, which must be nonempty); a match takes the
/// warm path and a mismatch ends the call. Both paths clear a scratch
/// buffer with the runtime fill, decide a ready byte from the scratch
/// validity hook and the state predicate, optionally encode the buffer, and
/// report through two host slots (warm: slots 21/20; cold: slots 12/13)
/// with the ready byte and the buffer.
///
/// Any other selector, or a clear enable bit, takes the tail: when `CH4` is
/// present and the state is outside 2..3, query the host's slot 4 with the
/// channel address; a nonzero answer followed by a nonzero id pair in the
/// scratch makes the slot-1 withdrawal for `CH4`. Original: 0x00625160
/// (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00625160(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x50;
        const MODE_ZERO: u32 = 0x120;
        const MODE_GATE: u32 = 0x124;
        const JOIN_GATE: u32 = 0x118;
        const REFRESH_REC: u32 = 0xd0;
        const CH0: u32 = 0x3300;
        const CH1: u32 = 0x3340;
        const CH2: u32 = 0x3380;
        const CH3: u32 = 0x33c0;
        const CH4: u32 = 0x3400;
        const FEAT: u32 = 0x19f32ac;
        const HOST_PTR: u32 = 0x19f2640;
        const HOST_GATE: u32 = 0x19f262c;
        const CFG_TAG: u32 = 0x19f2eb0;
        const INFO_TAG: u32 = 0xfaac3c;
        const F_MODE: u32 = 0x10;
        const F_ID0: u32 = 0x28;
        const F_ID1: u32 = 0x2c;
        const F_ACC0: u32 = 0x60;
        const F_ACC1: u32 = 0x64;
        const F_REP: u32 = 0x68;
        const F_REC_A: u32 = 0x20;
        const F_REC_B: u32 = 0x30;
        const F_REC2_A: u32 = 0x68;
        const F_REC2_B: u32 = 0x78;
        const C_CLR: u32 = 0x44;
        const C_CLR_AT: u32 = 0x69;
        const C_ENC_LEN: u32 = 0x45;
        const INIT_B: u32 = 1;
        const INIT_A: u32 = 2;
        const REFRESH: u32 = 3;
        const PUB0: u32 = 4;
        const PUB1: u32 = 5;
        const PUB2A: u32 = 6;
        const PUB2B: u32 = 7;
        const PUB3: u32 = 8;
        const VALID: u32 = 9;
        const PRED_A: u32 = 10;
        const PRED_B: u32 = 11;
        const ENCODE: u32 = 12;
        const V_WITHDRAW: u32 = 13;
        const V_QUERY: u32 = 14;
        const V_COLD0: u32 = 15;
        const V_COLD1: u32 = 16;
        const V_WARM0: u32 = 17;
        const V_WARM1: u32 = 18;
        const SLOT_WITHDRAW: u32 = 0x04;
        const SLOT_QUERY: u32 = 0x10;
        const SLOT_COLD0: u32 = 0x30;
        const SLOT_COLD1: u32 = 0x34;
        const SLOT_WARM0: u32 = 0x54;
        const SLOT_WARM1: u32 = 0x50;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Unsigned bytewise compare returning -1, 0 or 1 like the original.
        unsafe fn tag_cmp(mut p: u32, mut q: u32) -> i32 {
            unsafe {
                loop {
                    let a = rd8(p);
                    let b = rd8(q);
                    if a != b {
                        return if a < b { -1 } else { 1 };
                    }
                    if a == 0 {
                        return 0;
                    }
                    let a1 = rd8(p.wrapping_add(1));
                    let b1 = rd8(q.wrapping_add(1));
                    if a1 != b1 {
                        return if a1 < b1 { -1 } else { 1 };
                    }
                    if a1 == 0 {
                        return 0;
                    }
                    p = p.wrapping_add(2);
                    q = q.wrapping_add(2);
                }
            }
        }
        #[inline(always)]
        unsafe fn v_withdraw(host: u32, a1v: u32, a2v: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(host) + SLOT_WITHDRAW) as usize);
                slot(host, host, a1v, a2v, 0, 0);
            }
        }
        #[inline(always)]
        unsafe fn v_pair(host: u32, slot_off: u32, flag: u32) {
            unsafe {
                let vptr = rd32(host);
                let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vptr + slot_off) as usize);
                slot(vptr, host, flag);
            }
        }
        #[inline(always)]
        unsafe fn v_pair_buf(host: u32, slot_off: u32, obj: u32, buf: u32) {
            unsafe {
                let vptr = rd32(host);
                let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vptr + slot_off) as usize);
                slot(vptr, obj, buf);
            }
        }

        // The original's scratch frame as one byte array; stubbed callees
        // fill the record areas through pointers into it.
        let mut fr = [0u8; 0xB0];
        let fb = (&mut fr[0] as *mut u8) as u32;
        #[inline(always)]
        unsafe fn frd(fb: u32, off: u32) -> u32 {
            unsafe { ((fb.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn frw(fb: u32, off: u32, v: u32) {
            unsafe { ((fb.wrapping_add(off)) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn frb(fb: u32, off: u32) -> u8 {
            unsafe { ((fb.wrapping_add(off)) as *const u8).read() }
        }

        let sel_ok = (a0 as i32) >= 0 && a0 < 1;
        let feat_on = if sel_ok { (rd8(lf_checker_rt::relocated(FEAT)) >> 1) & 1 != 0 } else { false };
        if !sel_ok || !feat_on {
            // Tail: withdraw CH4 through a host query.
            if rd8(this.wrapping_add(CH4)) == 0 {
                return 0;
            }
            let st = rd32(this.wrapping_add(STATE));
            if st >= 2 && st <= 3 {
                return 0;
            }
            if (rd8(lf_checker_rt::relocated(FEAT)) >> 1) & 1 == 0 {
                return 0;
            }
            let host = rd32(lf_checker_rt::relocated(HOST_PTR));
            if host == 0 {
                return 0;
            }
            let slot: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(host) + SLOT_QUERY) as usize);
            let ans: u32 = slot(host, host, 0, this.wrapping_add(CH4), fb.wrapping_add(F_MODE));
            if ans & 0xff == 0 {
                return 0;
            }
            if frd(fb, F_MODE) == 0 && frd(fb, F_MODE.wrapping_add(4)) == 0 {
                return 0;
            }
            let host2 = rd32(lf_checker_rt::relocated(HOST_PTR));
            if host2 == 0 {
                return 0;
            }
            v_withdraw(host2, 0, this.wrapping_add(CH4));
            return 0;
        }
        // Main path.
        let state = rd32(this.wrapping_add(STATE));
        let mut mode = 0u32;
        if state >= 2 && state <= 3 && rd32(this.wrapping_add(MODE_GATE)) == 1 {
            mode = if rd32(this.wrapping_add(MODE_ZERO)) == 0 { 1 } else { 0 };
        }
        frw(fb, F_MODE, mode);
        frw(fb, F_ID0, 0);
        frw(fb, F_ID1, 0);
        lf_checker_rt::callee_thiscall!(INIT_B, u32, fb.wrapping_add(F_REC_B));
        lf_checker_rt::callee_thiscall!(INIT_A, u32, fb.wrapping_add(F_REC_A));
        if (a1 & 0xff) == 0 && frb(fb, F_MODE) == 0 {
            let st2 = rd32(this.wrapping_add(STATE));
            if st2 >= 2 && st2 <= 3 {
                lf_checker_rt::callee_stdcall!(REFRESH, u32, this.wrapping_add(REFRESH_REC));
            }
        }
        let st3 = rd32(this.wrapping_add(STATE));
        let mut ebp = frd(fb, F_ACC0);
        let joined = st3 >= 2
            && st3 <= 3
            && rd32(this.wrapping_add(JOIN_GATE)) == 1
            && (frd(fb, F_ID0) != 0 || frd(fb, F_ID1) != 0)
            && (ebp | frd(fb, F_ACC1)) != 0;
        if joined {
            if rd8(this.wrapping_add(CH0)) != 0 {
                frw(fb, F_MODE, frd(fb, F_ID0));
                frw(fb, F_MODE.wrapping_add(4), frd(fb, F_ID1));
                lf_checker_rt::callee_fastcall!(
                    PUB0, u32, a0, fb.wrapping_add(F_MODE), this.wrapping_add(CH0));
            }
            if rd8(this.wrapping_add(CH1)) != 0 {
                lf_checker_rt::callee_fastcall!(
                    PUB1, u32, a0, this.wrapping_add(CH1),
                    rd32(this.wrapping_add(0x540)), rd32(this.wrapping_add(0x544)));
            }
            if rd8(this.wrapping_add(CH2)) != 0 {
                lf_checker_rt::callee_fastcall!(
                    PUB2A, u32, a0, fb.wrapping_add(F_REC_A), this.wrapping_add(CH2));
            }
            if rd8(this.wrapping_add(CH3)) != 0 {
                lf_checker_rt::callee_fastcall!(
                    PUB3, u32, a0, (a2 & 0xff), this.wrapping_add(CH3));
            }
            if rd8(this.wrapping_add(CH4)) != 0 {
                let va: u32 =
                    lf_checker_rt::callee_thiscall!(VALID, u32, fb.wrapping_add(F_REC_A));
                if va & 0xff != 0 {
                    let pa: u32 = lf_checker_rt::callee_thiscall!(PRED_A, u32, this);
                    if pa & 0xff != 0 {
                        lf_checker_rt::callee_fastcall!(
                            PUB3, u32, a0, 1, this.wrapping_add(CH4));
                        ebp = frd(fb, F_ACC1);
                    } else {
                        lf_checker_rt::callee_fastcall!(
                            PUB3, u32, a0, 0, this.wrapping_add(CH4));
                        ebp = frd(fb, F_ACC0);
                    }
                } else {
                    lf_checker_rt::callee_fastcall!(PUB3, u32, a0, 0, this.wrapping_add(CH4));
                    ebp = frd(fb, F_ACC0);
                }
            } else {
                ebp = frd(fb, F_ACC0);
            }
        } else {
            let host_of = || rd32(lf_checker_rt::relocated(HOST_PTR));
            if rd8(this.wrapping_add(CH0)) != 0 {
                let h = host_of();
                if h != 0 {
                    v_withdraw(h, a0, this.wrapping_add(CH0));
                }
            }
            if rd8(this.wrapping_add(CH1)) != 0 {
                let h = host_of();
                if h != 0 {
                    v_withdraw(h, a0, this.wrapping_add(CH1));
                }
            }
            if rd8(this.wrapping_add(CH2)) != 0 {
                frw(fb, F_REP.wrapping_add(8), 0);
                frw(fb, F_REP.wrapping_add(12), 0);
                lf_checker_rt::callee_thiscall!(INIT_B, u32, fb.wrapping_add(F_REC2_B));
                lf_checker_rt::callee_thiscall!(INIT_A, u32, fb.wrapping_add(F_REC2_A));
                lf_checker_rt::callee_fastcall!(
                    PUB2B, u32, a0, 0, this.wrapping_add(CH2));
            }
            if rd8(this.wrapping_add(CH3)) != 0 {
                let h = host_of();
                if h != 0 {
                    v_withdraw(h, a0, this.wrapping_add(CH3));
                }
            }
            if rd8(this.wrapping_add(CH4)) != 0 {
                let h = host_of();
                if h != 0 {
                    v_withdraw(h, a0, this.wrapping_add(CH4));
                }
            }
        }
        if rd32(lf_checker_rt::relocated(HOST_GATE)) == 0 {
            return 0;
        }
        let tag = this.wrapping_add(CH2);
        let take_cold = if rd8(tag) == 0 {
            true
        } else if tag_cmp(tag, lf_checker_rt::relocated(INFO_TAG)) == 0 {
            true
        } else {
            if rd8(lf_checker_rt::relocated(CFG_TAG)) == 0 {
                return 0;
            }
            if tag_cmp(tag, lf_checker_rt::relocated(CFG_TAG)) != 0 {
                return 0;
            }
            false
        };
        let mut ready: u8;
        if !take_cold {
            let va: u32 = lf_checker_rt::callee_thiscall!(VALID, u32, fb.wrapping_add(F_REC_A));
            ready = 0;
            if va & 0xff != 0 {
                let pb: u32 = lf_checker_rt::callee_thiscall!(PRED_B, u32, this);
                if pb & 0xff != 0 {
                    ready = 1;
                }
            }
            frw(fb, F_MODE, ready as u32);
            ((fb.wrapping_add(F_REP)) as *mut u8).write(0);
            core::ptr::write_bytes((fb.wrapping_add(C_CLR_AT)) as *mut u8, 0, C_CLR as usize);
            if ready != 0 {
                lf_checker_rt::callee_thiscall!(
                    ENCODE, u32, fb.wrapping_add(F_REC_A), fb.wrapping_add(F_REP),
                    C_ENC_LEN);
            }
            let h = rd32(lf_checker_rt::relocated(HOST_PTR));
            v_pair(h, SLOT_WARM0, frd(fb, F_MODE));
            v_pair_buf(h, SLOT_WARM1, h, fb.wrapping_add(F_REP));
            return 0;
        }
        // Cold path.
        ready = 0;
        if frd(fb, F_ID0) != 0 || frd(fb, F_ID1) != 0 {
            if (ebp | frd(fb, F_ACC1)) != 0 {
                let pb: u32 = lf_checker_rt::callee_thiscall!(PRED_B, u32, this);
                if pb & 0xff != 0 {
                    ready = 1;
                }
            }
        }
        frw(fb, F_MODE, ready as u32);
        ((fb.wrapping_add(F_REP)) as *mut u8).write(0);
        core::ptr::write_bytes((fb.wrapping_add(C_CLR_AT)) as *mut u8, 0, C_CLR as usize);
        if ready != 0 {
            lf_checker_rt::callee_thiscall!(
                ENCODE, u32, fb.wrapping_add(F_REC_A), fb.wrapping_add(F_REP), C_ENC_LEN);
        }
        let h = rd32(lf_checker_rt::relocated(HOST_PTR));
        v_pair(h, SLOT_COLD0, frd(fb, F_MODE));
        v_pair_buf(h, SLOT_COLD1, h, fb.wrapping_add(F_REP));
        0
    }
});
