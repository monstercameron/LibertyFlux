// original: 0x006600B0 snJoinRequest_setupGamer (proposed)

/// Look up a joining gamer in the session manager and copy its record.
///
/// `this` is the join-request task. `gamer` points at the candidate gamer
/// descriptor, `rec` at a 14-byte record whose words are copied into the
/// task, and `mode` selects the join path: the flag passed on is 1 unless
/// `mode` is 2. `a0` is never read.
///
/// Behaviour: notify the object at `this+0x98` about `gamer` (callee 1),
/// stamp the task's join slots (`+0x2c0`/`+0x2c8`/`+0x2e0` to all-ones,
/// `+0x2c4`/`+0x2cc` to zero), copy the record into `+0x2d0..+0x2de`,
/// compare the gamer's identity words (`+0x24`, `+0x18`, `+0x14`) against
/// the manager's (`+0x104`, `+0xf8`, `+0xf4`) — the result is dropped, only
/// the reads can fault — then call the join worker (callee 2) with the
/// manager's `+0x24` object, the task pointers, the record, the flag and
/// the manager's `+0xd8`/`+0xdc` words. On a nonzero answer clear
/// `this+0x90` and return 1, else return 0.
///
/// The original also overwrites its incoming `gamer` stack slot with the
/// flag and stores scratch words (manager `+0xd8`/`+0xd0`/`+0xd4`, the
/// match bit) that are never read back; none of that is observable except
/// through the stack check, which is off for this function. The third
/// argument of the join call points at the original's own saved-register
/// slot and carries nothing; the rewrite passes its own scratch word and
/// the contract skips that argument.
///
/// Original: 0x006600B0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_006600b0(this: u32, _a0: u32, gamer: u32, rec: u32, mode: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x60;
        const NOTIFY_OBJ: u32 = 0x98;
        const TASK_D0: u32 = 0xd0;
        const CLEARED: u32 = 0x90;
        const SLOT_A: u32 = 0x2c0;
        const SLOT_B: u32 = 0x2c8;
        const SLOT_C: u32 = 0x2e0;
        const REC_DST: u32 = 0x2d0;
        const TAIL_PTR: u32 = 0x2e4;
        const C_NOTIFY: u32 = 1;
        const C_JOIN: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let _ = lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, this.wrapping_add(NOTIFY_OBJ), gamer);
        wr32(this.wrapping_add(SLOT_A), 0xffff_ffff);
        wr16(this.wrapping_add(0x2c4), 0);
        wr16(this.wrapping_add(0x2cc), 0);
        wr32(this.wrapping_add(SLOT_B), 0xffff_ffff);
        wr32(this.wrapping_add(REC_DST), rd32(rec));
        wr16(this.wrapping_add(0x2d4), rd16(rec.wrapping_add(4)));
        wr32(this.wrapping_add(0x2d8), rd32(rec.wrapping_add(8)));
        wr16(this.wrapping_add(0x2dc), rd16(rec.wrapping_add(0x0c)));
        wr32(this.wrapping_add(SLOT_C), 0xffff_ffff);

        let mgr = rd32(this.wrapping_add(MGR));
        let flag: u32 = (mode != 2) as u32;
        // Identity comparison in the original's order; the outcome is
        // dropped (the original stores it to dead scratch), but the reads
        // must happen for identical fault behaviour.
        let matched = rd32(mgr.wrapping_add(0x104)) == rd32(gamer.wrapping_add(0x24))
            && rd16(mgr.wrapping_add(0xf8)) == rd16(gamer.wrapping_add(0x18))
            && rd32(mgr.wrapping_add(0xf4)) == rd32(gamer.wrapping_add(0x14));
        core::hint::black_box(matched);

        let mut scratch = [0u32; 2];
        let answer: u32 = lf_checker_rt::callee_thiscall!(
            C_JOIN,
            u32,
            rd32(mgr.wrapping_add(0x24)),
            this.wrapping_add(NOTIFY_OBJ),
            rec,
            scratch.as_mut_ptr() as u32,
            flag,
            5,
            rd32(mgr.wrapping_add(0xd8)),
            rd32(mgr.wrapping_add(0xdc)),
            1,
            this.wrapping_add(TASK_D0),
            this.wrapping_add(TAIL_PTR)
        );
        if answer & 0xff != 0 {
            wr32(this.wrapping_add(CLEARED), 0);
            1
        } else {
            0
        }
    }
});
