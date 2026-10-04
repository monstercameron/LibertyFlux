// original: 0x00663b70 rage::snMigrateSessionTask::vf3

/// Run the session-migration task's state machine until it settles.
///
/// `this` is the task, `arg` a tick-sized decrement applied to the delay at
/// `+0x14` (a zero result is stored as -1). The machine switches on the state
/// at `+0x90`: state 0 advances the migration step and, when that reports
/// nothing left and the cursor has reached its count, closes the task through
/// the slot-7 hook; state 1 either enters the matched session, re-arms the
/// hook, or parks at state 0 depending on the flag at `+0x94` and the
/// manager's match; state 2 drains through the slot-7 hook unless already
/// flagged; state 3 retries or counts the delay down to the finish hook.
/// After each step the machine exits once the flag reads 1, otherwise it
/// polls the progress hook and repeats while that reports more work.
/// Original: 0x00663b70 (thiscall, one stack argument, no result).
lf_checker_rt::export!(thiscall, rw_00663B70(this: u32, arg: u32) -> u32 {
    unsafe { sn_migrate_run(this, arg) }
});

unsafe fn sn_migrate_run(this: u32, arg: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 64;
        const FLAG_BASE: u32 = 0x8E0;
        const VT_SLOT7: u32 = 0x1C;
        const C_ADVANCE: u32 = 1;
        const C_VCALL: u32 = 2;
        const C_POLL: u32 = 3;
        const C_ENTER: u32 = 4;
        const C_LOOKUP: u32 = 5;
        const C_DRAIN: u32 = 6;
        const C_RETRY: u32 = 7;
        const C_FINISH: u32 = 8;
        const C_CAS: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall(this: u32, a0: u32, a1: u32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + VT_SLOT7) as usize);
                f(this, a0, a1);
            }
        }
        #[inline(always)]
        unsafe fn row_flag(this: u32, idx: u32) -> u8 {
            unsafe { ((idx.wrapping_add(this).wrapping_add(FLAG_BASE)) as *const u8).read() }
        }

        if rd32(this + 0x14) as i32 > 0 {
            let d = rd32(this + 0x14).wrapping_sub(arg);
            wr32(this + 0x14, if d == 0 { 0xffff_ffff } else { d });
        }
        let mgr = rd32(this + 0x60);
        loop {
            match rd32(this + 0x90) {
                0 => {
                    let adv: u32 = lf_checker_rt::callee_thiscall!(C_ADVANCE, u32, this);
                    if adv & 0xff == 0
                        && rd32(this + 0x908) as i32 >= rd32(this + 0x904) as i32
                    {
                        vcall(this, 0, 0);
                    }
                }
                1 => {
                    if rd32(this + 0x94) == 3 {
                        let st = rd32(mgr + 0x50) as i32;
                        let matched = st >= 2
                            && st <= 3
                            && rd32(mgr + 0xBF0) == rd32(mgr + 0xC30)
                            && rd32(mgr + 0xBF4) == rd32(mgr + 0xC34);
                        if matched {
                            if rd32(mgr + 0x2EA4) != 0 {
                                wr32(this + 0x90, 2);
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(C_ENTER, u32, this);
                            } else {
                                vcall(this, 1, 0);
                            }
                        } else {
                            let b = rd32(this + 0x908);
                            let row = this.wrapping_add(b << 6);
                            let found: u32 = lf_checker_rt::callee_thiscall!(
                                C_LOOKUP, u32, mgr, rd32(row + 0xD8), rd32(row + 0xDC)
                            );
                            if found != 0 && rd32(found) as i32 >= 0 && row_flag(this, b) == 0
                            {
                                vcall(this, 1, 0);
                            } else {
                                wr32(this + 0x90, 0);
                            }
                        }
                    } else if rd32(this + 0x94) != 1 {
                        wr32(this + 0x90, 0);
                    }
                }
                2 => {
                    if rd32(this + 0x94) == 1 {
                        break;
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_DRAIN, u32, this);
                    vcall(this, 1, 0);
                }
                3 => {
                    let f = rd32(this + 0x94);
                    if f == 3 {
                        let b = rd32(this + 0x908);
                        let row = this
                            .wrapping_add(0xA0)
                            .wrapping_add(b << 6);
                        let retry: u32 = lf_checker_rt::callee_thiscall!(
                            C_RETRY, u32, this, row, this + 0x910
                        );
                        if retry & 0xff == 0 {
                            vcall(this, 0, 0);
                        } else {
                            wr32(this + 0x90, 1);
                        }
                    } else if f != 1 {
                        vcall(this, 0, 0);
                    } else {
                        let b = rd32(this + 0x908);
                        let row = this.wrapping_add(b << 6);
                        let found: u32 = lf_checker_rt::callee_thiscall!(
                            C_LOOKUP, u32, mgr, rd32(row + 0xD8), rd32(row + 0xDC)
                        );
                        if found != 0 && rd32(found) as i32 >= 0 && row_flag(this, b) == 0 {
                            wr32(this + 0x1694, rd32(this + 0x1694).wrapping_sub(arg));
                            if rd32(this + 0x1694) as i32 <= 0 {
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    C_FINISH, u32, this + 0x94
                                );
                                wr32(this + 0x90, 0);
                            }
                        } else {
                            let prev: u32 = lf_checker_rt::callee_stdcall!(
                                C_CAS, u32, this + 0x94, 4, 1
                            );
                            if prev == 1 {
                                wr32(this + 0x98, 0xffff_ffff);
                            }
                            wr32(this + 0x90, 0);
                        }
                    }
                }
                _ => {}
            }
            if rd32(this + 0x94) == 1 {
                break;
            }
            let more: u32 = lf_checker_rt::callee_thiscall!(C_POLL, u32, this);
            if more & 0xff == 0 {
                break;
            }
        }
        0
    }
}
