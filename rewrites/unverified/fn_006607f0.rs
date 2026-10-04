// original: 0x006607F0 rage::snAddRemoteGamerTask::vf3

/// Tick the add-remote-gamer task's state machine until it settles.
///
/// `this` is the task and `a0` the elapsed time, first subtracted from
/// the countdown at `this+0x14` (a countdown already at or below zero is
/// left alone; one landing exactly on zero becomes -1). Then the state
/// at `this+0x94` dispatches, looping back until the box at `this+0x530`
/// reads 1 or the settle check (callee 8) on the task answers zero:
///
/// * State 2 scans the manager's `+0x32d4` list through virtual slot
///   `+0x28` (callees 1 and 9, one stub per list node) for the wanted
///   marker: on a match the state
///   becomes 1, the box is set to 1 with an interlocked exchange
///   (callee 4) and its second word cleared. A miss with `this+0x90`
///   set completes with (0, 0); with it clear, the filler (callee 2)
///   runs on the manager with `this+0xa0`,
///   `this+0x120`, the `this+0x124` value (or zero, with `this+0x128`
///   then also zeroed as its object) and the value itself: a nonzero
///   answer completes through virtual slot `+0x1c` (callee 3) with
///   (1, 0), a zero one with (0, 0).
/// * State 1 scans the same list: a match ends the tick, otherwise the
///   state becomes 2 and an interlocked compare-exchange (callee 5:
///   expect 1, set 3) races the box, the winner clearing its second
///   word.
/// * State 0 with the box at 1 ends the tick when `this+0x90` is clear
///   and completes with (0, 0) otherwise. With the box changed, the
///   session pair `this+0xd8`/`+0xdc` is resolved (callee 6): a hit, or
///   a miss followed by a nonzero attach (callee 7) on the manager with
///   (`this+0xa0`, `this+0x7f8`, `this+0x110`, `this+0x818`, 0, 0 and the
///   checker's register residue), lands with the state at 2 when
///   `this+0x90` is clear and completes with (0, 0) otherwise; a miss
///   with `this+0x90` set, or a zero attach, completes with (0, 0).
/// * Any other state ends the tick at once.
///
/// The attach call's last word is whatever the session check left in
/// its object register: the checker's stub residue on both sides, with
/// no game meaning, so the contract skips it.
///
/// Original: 0x006607F0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_006607f0(this: u32, a0: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x14;
        const MGR: u32 = 0x60;
        const STATE: u32 = 0x94;
        const FLAG90: u32 = 0x90;
        const BOX: u32 = 0x530;
        const SESS0: u32 = 0xd8;
        const SESS1: u32 = 0xdc;
        const MARKER: u32 = 0x01bb_66b0;
        const C_SCAN: u32 = 1;
        const C_FILL: u32 = 2;
        const C_VT1C: u32 = 3;
        const C_XCHG: u32 = 4;
        const C_CMPXCHG: u32 = 5;
        const C_SESS: u32 = 6;
        const C_ATTACH: u32 = 7;
        const C_SETTLE: u32 = 8;

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
        unsafe fn scan_hit(mgr: u32) -> bool {
            unsafe {
                let _ = C_SCAN;
                let mut node = rd32(mgr.wrapping_add(0x32d4));
                while node != 0 {
                    let slot = rd32(rd32(node).wrapping_add(0x28));
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    if f(node) == MARKER {
                        return true;
                    }
                    node = rd32(node.wrapping_add(0x64));
                }
                false
            }
        }
        #[inline(always)]
        unsafe fn complete(this: u32, a: u32, b: u32) {
            unsafe {
                let slot = rd32(rd32(this).wrapping_add(0x1c));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let _ = C_VT1C;
                let _: u32 = f(this, a, b);
            }
        }

        let t = rd32(this.wrapping_add(COUNT)) as i32;
        if t > 0 {
            let r = (t as u32).wrapping_sub(a0);
            wr32(this.wrapping_add(COUNT), r);
            wr32(this.wrapping_add(COUNT), if r == 0 { 0xffff_ffff } else { r });
        }
        let mgr = rd32(this.wrapping_add(MGR));
        let obj = this.wrapping_add(BOX);
        loop {
            let st = rd32(this.wrapping_add(STATE));
            if st == 2 {
                if scan_hit(mgr) {
                    wr32(this.wrapping_add(STATE), 1);
                    let _: u32 =
                        lf_checker_rt::callee_stdcall!(C_XCHG, u32, obj, 1);
                    wr32(obj.wrapping_add(4), 0);
                } else if rd8(this.wrapping_add(FLAG90)) != 0 {
                    complete(this, 0, 0);
                } else {
                    let v = rd32(this.wrapping_add(0x124));
                    let (arg_eax, arg_ecx) = if v != 0 {
                        (v, this.wrapping_add(0x128))
                    } else {
                        (0, 0)
                    };
                    let filled: u32 = lf_checker_rt::callee_thiscall!(
                        C_FILL,
                        u32,
                        mgr,
                        this.wrapping_add(0xa0),
                        rd32(this.wrapping_add(0x120)),
                        arg_ecx,
                        arg_eax
                    );
                    if filled & 0xff != 0 {
                        complete(this, 1, 0);
                    } else {
                        complete(this, 0, 0);
                    }
                }
            } else if st == 1 {
                if !scan_hit(mgr) {
                    wr32(this.wrapping_add(STATE), 2);
                    let won: u32 = lf_checker_rt::callee_stdcall!(
                        C_CMPXCHG,
                        u32,
                        obj,
                        3,
                        1
                    );
                    if won == 1 {
                        wr32(obj.wrapping_add(4), 0);
                    }
                }
            } else if st == 0 {
                if rd32(obj) == 1 {
                    if rd8(this.wrapping_add(FLAG90)) != 0 {
                        complete(this, 0, 0);
                    }
                } else {
                    let d0 = rd32(this.wrapping_add(SESS0));
                    let d1 = rd32(this.wrapping_add(SESS1));
                    let sess: u32 =
                        lf_checker_rt::callee_thiscall!(C_SESS, u32, mgr, d0, d1);
                    let mut landed = sess != 0;
                    if !landed && rd8(this.wrapping_add(FLAG90)) == 0 {
                        let attached: u32 = lf_checker_rt::callee_thiscall!(
                            C_ATTACH,
                            u32,
                            mgr,
                            this.wrapping_add(0xa0),
                            this.wrapping_add(0x7f8),
                            this.wrapping_add(0x110),
                            rd32(this.wrapping_add(0x818)),
                            0,
                            0,
                            d1
                        );
                        landed = attached != 0;
                    }
                    if landed {
                        if rd8(this.wrapping_add(FLAG90)) != 0 {
                            complete(this, 0, 0);
                        } else {
                            wr32(this.wrapping_add(STATE), 2);
                        }
                    } else if sess == 0 {
                        // Miss with the flag set, or a zero attach.
                        complete(this, 0, 0);
                    }
                }
            }
            if rd32(obj) == 1 {
                break;
            }
            let settle: u32 = lf_checker_rt::callee_thiscall!(C_SETTLE, u32, this);
            if settle & 0xff == 0 {
                break;
            }
        }
        0
    }
});
