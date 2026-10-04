// original: 0x00661360 rage::snLeaveGamersFromRlineTask::vf2

/// Leave the task's gamers from their line, then report.
///
/// `this` is the task. The manager at `this+0x60` must be in state 2 or
/// 3 (signed); otherwise the task completes with (0, 0) through its own
/// virtual slot `+0x1c` (callee 4). On the main path a scratch buffer of
/// `0x1f0` zeroed bytes collects up to `this+0x2a0` (signed) gamer
/// records from `this+0xa0` (16 bytes each): each is ranked (callee 1)
/// on the manager's `+0x48` object, negatives are skipped, the rest are
/// copied in order. With none collected, or when the removals call
/// below answers zero, an interlocked exchange (callee 3) sets
/// `this+0x94` to 3 and its second word is cleared. Otherwise the
/// removals call (callee 2) runs on the `+0x48` object with the buffer,
/// the collected count and `this+0x94`. Either way `this+0x90` becomes 1.
///
/// The original guards its frame with the CRT security cookie (callee 5,
/// register-preserving); the rewrite issues the same intercepted call so
/// the call sequences match, with no register comparison.
///
/// Original: 0x00661360 (thiscall, no stack words). The main path lives
/// in an out-of-line tail past two neighbouring functions; the entry
/// chunk jumps there directly.
lf_checker_rt::export!(thiscall, rw_00661360(this: u32) -> u32 {
    unsafe {
        const STARTED: u32 = 0x0c;
        const MGR: u32 = 0x60;
        const STATE: u32 = 0x94;
        const FLAG90: u32 = 0x90;
        const GAMERS: u32 = 0xa0;
        const COUNT: u32 = 0x2a0;
        const C_RANK: u32 = 1;
        const C_REMOVE: u32 = 2;
        const C_XCHG: u32 = 3;
        const C_DONE: u32 = 4;
        const C_COOKIE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(this.wrapping_add(STARTED)) == 0 {
            wr32(this.wrapping_add(STARTED), 1);
        }
        let mgr = rd32(this.wrapping_add(MGR));
        let st = rd32(mgr.wrapping_add(0x50)) as i32;
        if st < 2 || st > 3 {
            let slot = rd32(rd32(this).wrapping_add(0x1c));
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let _ = C_DONE;
            let _: u32 = f(this, 0, 0);
            return 0;
        }
        let left = rd32(this.wrapping_add(COUNT)) as i32;
        let mut buf = [0u32; 124];
        let mut accepted = 0u32;
        if left > 0 {
            let mut i = 0i32;
            let mut src = this.wrapping_add(GAMERS);
            let obj = mgr.wrapping_add(0x48);
            while i < left {
                let rank: u32 =
                    lf_checker_rt::callee_thiscall!(C_RANK, u32, obj, src);
                if rank as i32 >= 0 {
                    let dst = accepted.wrapping_mul(4);
                    buf[dst as usize] = rd32(src);
                    buf[dst as usize + 1] = rd32(src.wrapping_add(4));
                    buf[dst as usize + 2] = rd32(src.wrapping_add(8));
                    buf[dst as usize + 3] = rd32(src.wrapping_add(12));
                    accepted += 1;
                }
                src = src.wrapping_add(16);
                i += 1;
            }
            if accepted > 0 {
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    C_REMOVE,
                    u32,
                    obj,
                    buf.as_mut_ptr() as u32,
                    accepted,
                    this.wrapping_add(STATE)
                );
                if ok & 0xff != 0 {
                    wr32(this.wrapping_add(FLAG90), 1);
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_COOKIE, u32, 0);
                    return 0;
                }
            }
        }
        let stw = this.wrapping_add(STATE);
        let _: u32 = lf_checker_rt::callee_stdcall!(C_XCHG, u32, stw, 3);
        wr32(stw.wrapping_add(4), 0);
        wr32(this.wrapping_add(FLAG90), 1);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_COOKIE, u32, 0);
        0
    }
});
