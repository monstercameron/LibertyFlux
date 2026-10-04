// original: 0x00900ab0 ui_pending_action_step (proposed)

/// Advance one step of a UI slot's pending-action state machine.
///
/// `this` points to the slot object: flag bytes at `+0x10` (down/teardown
/// pending), `+0x11`/`+0x12` (actions A/B take the `arg` value),
/// `+0x13`/`+0x14` (actions C/D), a callee-managed pair at `+0x18`, a handle
/// at `+0x20` and a counter at `+0x24`. `arg` is an opaque value forwarded to
/// the A/B actions; only its low byte gates the tail retry. The global dword
/// at `0x1173594` limits the D action against `counter + 0x493e0` (unsigned).
///
/// Behaviour: acquire a session (callee 1). When one is returned, validate it
/// (callees 2, 3 with the pair); on success snapshot two words through the
/// info pointer into the pair, refresh the handle (callee 4) and notify
/// (callee 5 with 0). With no session the handle is set to -1, the pair is
/// reset (callee 6), a pending teardown runs (callee 7) and flags C/D are
/// cleared. A readiness check (callee 8 on the pair) returning zero clears
/// flags C/D and returns. Otherwise exactly one pending action runs, in
/// A, B, C, D priority order: A/B call their handler with `arg` and clear on
/// a nonzero answer; C calls its handler and on a nonzero answer sets flag A
/// before clearing itself; D is skipped while the counter is nonzero and the
/// biased counter reaches the global limit, otherwise calls its handler,
/// sets flag B on a nonzero answer, and clears itself. When `arg`'s low byte
/// is set, a still-pending A action is retried (a zero answer returns, a
/// nonzero answer clears A and returns), else a still-pending B action is
/// retried and cleared on a nonzero answer.
///
/// Original: 0x00900ab0 (thiscall, one stack word; no return value; all
/// twelve callees are thiscall and take zero or one stack words).
lf_checker_rt::export!(thiscall, rw_00900ab0(this: u32, arg: u32) -> u32 {
    unsafe {
        const PAIR: u32 = 0x18;
        const HANDLE: u32 = 0x20;
        const COUNT: u32 = 0x24;
        const FLAG_DOWN: u32 = 0x10;
        const FLAG_A: u32 = 0x11;
        const FLAG_B: u32 = 0x12;
        const FLAG_C: u32 = 0x13;
        const FLAG_D: u32 = 0x14;
        const COUNT_BIAS: u32 = 0x0004_93e0;
        const LIMIT_GV: u32 = 0x0117_3594;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let session = lf_checker_rt::callee_thiscall!(1, u32, this);
        let pair = this.wrapping_add(PAIR);
        if session != 0 {
            let info = lf_checker_rt::callee_thiscall!(2, u32, session);
            let ok = lf_checker_rt::callee_thiscall!(3, u32, info, pair) as u8;
            if ok != 0 {
                let p = lf_checker_rt::callee_thiscall!(2, u32, session);
                wr32(pair, rd32(p));
                wr32(pair.wrapping_add(4), rd32(p.wrapping_add(4)));
                let h = lf_checker_rt::callee_thiscall!(4, u32, session);
                wr32(this.wrapping_add(HANDLE), h);
                lf_checker_rt::callee_thiscall!(5, u32, this, 0);
            }
        } else {
            wr32(this.wrapping_add(HANDLE), 0xffff_ffff);
            lf_checker_rt::callee_thiscall!(6, u32, pair);
            if rd8(this.wrapping_add(FLAG_DOWN)) != 0 {
                lf_checker_rt::callee_thiscall!(7, u32, this);
                wr8(this.wrapping_add(FLAG_DOWN), 0);
            }
            wr8(this.wrapping_add(FLAG_C), 0);
            wr8(this.wrapping_add(FLAG_D), 0);
        }
        let ready = lf_checker_rt::callee_thiscall!(8, u32, pair) as u8;
        if ready == 0 {
            wr8(this.wrapping_add(FLAG_C), 0);
            wr8(this.wrapping_add(FLAG_D), 0);
            return 0;
        }
        if rd8(this.wrapping_add(FLAG_A)) != 0 {
            let ok = lf_checker_rt::callee_thiscall!(9, u32, this, arg) as u8;
            if ok != 0 {
                wr8(this.wrapping_add(FLAG_A), 0);
            }
        } else if rd8(this.wrapping_add(FLAG_B)) != 0 {
            let ok = lf_checker_rt::callee_thiscall!(10, u32, this, arg) as u8;
            if ok != 0 {
                wr8(this.wrapping_add(FLAG_B), 0);
            }
        } else if rd8(this.wrapping_add(FLAG_C)) != 0 {
            let ok = lf_checker_rt::callee_thiscall!(11, u32, this) as u8;
            if ok != 0 {
                wr8(this.wrapping_add(FLAG_A), 1);
            }
            wr8(this.wrapping_add(FLAG_C), 0);
        } else if rd8(this.wrapping_add(FLAG_D)) != 0 {
            let n = rd32(this.wrapping_add(COUNT));
            let limit = (lf_checker_rt::relocated(LIMIT_GV) as *const u32).read_unaligned();
            let skip = limit <= n.wrapping_add(COUNT_BIAS) && n != 0;
            if !skip {
                let ok = lf_checker_rt::callee_thiscall!(12, u32, this) as u8;
                if ok != 0 {
                    wr8(this.wrapping_add(FLAG_B), 1);
                }
                wr8(this.wrapping_add(FLAG_D), 0);
            }
        }
        if arg & 0xff == 0 {
            return 0;
        }
        if rd8(this.wrapping_add(FLAG_A)) != 0 {
            let ok = lf_checker_rt::callee_thiscall!(9, u32, this, arg) as u8;
            if ok == 0 {
                return 0;
            }
            wr8(this.wrapping_add(FLAG_A), 0);
            return 0;
        }
        if rd8(this.wrapping_add(FLAG_B)) != 0 {
            let ok = lf_checker_rt::callee_thiscall!(10, u32, this, arg) as u8;
            if ok != 0 {
                wr8(this.wrapping_add(FLAG_B), 0);
            }
        }
        0
    }
});
