// original: 0x006616B0 rage::snDropGamersTask::vf2

/// Drop the task's collected gamers from the session, then report completion.
///
/// `this` is the drop-gamers task. The manager at `this+0x60` must be in
/// state 2 or 3 (signed) with a changed gamer pair (`+0xbf0`/`+0xbf4`
/// against `+0xc30`/`+0xc34`); otherwise the task completes with code 0.
/// On the main path the pending count at `+0x398` is taken and cleared,
/// and for each pending entry the gamer lookup (callee 1) runs over the
/// next 8-byte key starting at `this+0x98`: a hit copies the record's 16
/// bytes at `+0x48` into
/// the drop list at `this+0x198` (16 bytes per hit) and bumps the stored
/// count. With no hits the task completes with code 1. Otherwise the
/// session check (callee 2) runs over `this+0x90`/`+0x94`; a null answer
/// completes with code 1, else the drop call (callee 4, seven words:
/// the `0x90`/`0x94` pair, the list, the hit count, zero, the manager,
/// `+0x39c`) runs and its answer feeds the finish call (callee 3): a
/// nonzero low byte completes with code 1, zero with code 0. Completion
/// is the task's own virtual slot `+0x1c` (callee 5), whose answer is the
/// return value.
///
/// Callees 2 and 4 share one target, called with two words and with seven;
/// the target pops two words, the frame restore absorbs the rest, and the
/// contract patches each site separately. The drop call's sixth word is
/// the manager: the original passes whatever the session check left in
/// its object register, and that callee's hit path (the only one that
/// reaches the drop call) preserves it, so the contract restores it; the
/// miss path clobbers it but always leaves through an earlier branch.
///
/// Original: 0x006616B0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_006616b0(this: u32) -> u32 {
    unsafe {
        const STARTED: u32 = 0x0c;
        const MGR: u32 = 0x60;
        const MGR_STATE: u32 = 0x50;
        const PAIR_A0: u32 = 0xbf0;
        const PAIR_A1: u32 = 0xbf4;
        const PAIR_B0: u32 = 0xc30;
        const PAIR_B1: u32 = 0xc34;
        const KEY: u32 = 0x98;
        const DROP_LIST: u32 = 0x198;
        const COUNT: u32 = 0x398;
        const EXTRA: u32 = 0x39c;
        const SESS0: u32 = 0x90;
        const SESS1: u32 = 0x94;
        const VT_DONE: u32 = 0x1c;
        const C_LOOKUP: u32 = 1;
        const C_SESS: u32 = 2;
        const C_FINISH: u32 = 3;
        const C_DROP: u32 = 4;
        const C_DONE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn done(this: u32, code: u32, flag: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(this).wrapping_add(VT_DONE));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let _ = C_DONE;
                f(this, code, flag)
            }
        }

        if rd32(this.wrapping_add(STARTED)) == 0 {
            wr32(this.wrapping_add(STARTED), 1);
        }
        let mgr = rd32(this.wrapping_add(MGR));
        let state = rd32(mgr.wrapping_add(MGR_STATE)) as i32;
        if state < 2 || state > 3 {
            return done(this, 0, 0);
        }
        if rd32(mgr.wrapping_add(PAIR_A0)) == rd32(mgr.wrapping_add(PAIR_B0))
            && rd32(mgr.wrapping_add(PAIR_A1)) == rd32(mgr.wrapping_add(PAIR_B1))
        {
            return done(this, 0, 0);
        }
        let mut left = rd32(this.wrapping_add(COUNT)) as i32;
        wr32(this.wrapping_add(COUNT), 0);
        if left > 0 {
            let mut key = this.wrapping_add(KEY);
            loop {
                let rec: u32 =
                    lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, key);
                if rec != 0 {
                    let k = rd32(this.wrapping_add(COUNT));
                    let dst = this.wrapping_add(DROP_LIST).wrapping_add(k.wrapping_mul(16));
                    let lo0 = rd32(rec.wrapping_add(0x48));
                    let lo1 = rd32(rec.wrapping_add(0x4c));
                    wr32(dst, lo0);
                    wr32(dst.wrapping_add(4), lo1);
                    let hi0 = rd32(rec.wrapping_add(0x50));
                    let hi1 = rd32(rec.wrapping_add(0x54));
                    wr32(dst.wrapping_add(8), hi0);
                    wr32(dst.wrapping_add(12), hi1);
                    wr32(this.wrapping_add(COUNT), k.wrapping_add(1));
                }
                key = key.wrapping_add(8);
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        let hits = rd32(this.wrapping_add(COUNT));
        if hits == 0 {
            return done(this, 1, 0);
        }
        let s0 = rd32(this.wrapping_add(SESS0));
        let s1 = rd32(this.wrapping_add(SESS1));
        let found: u32 = lf_checker_rt::callee_thiscall!(C_SESS, u32, mgr, s0, s1);
        if found == 0 {
            return done(this, 1, 0);
        }
        let dropped: u32 = lf_checker_rt::callee_thiscall!(
            C_DROP,
            u32,
            mgr,
            s0,
            s1,
            this.wrapping_add(DROP_LIST),
            hits,
            0,
            mgr,
            rd32(this.wrapping_add(EXTRA))
        );
        let finished: u32 = lf_checker_rt::callee_thiscall!(C_FINISH, u32, mgr, dropped);
        if finished & 0xff != 0 {
            done(this, 1, 0)
        } else {
            done(this, 0, 0)
        }
    }
});
