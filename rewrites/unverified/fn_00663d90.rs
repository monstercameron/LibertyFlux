// original: 0x00663d90 sn_migrate_advance (proposed)

/// Advance the session-migration task by one step, returning 1 on success.
///
/// `this` is the task, whose manager object is reached at `+0x60`. The step
/// clears the manager's pending-migration pair, bumps the cursor at
/// `+0x908`, and bails out with 0 unless the manager state at `+0x50` is 2
/// or 3. When the previous cursor was non-negative the row it names (stride
/// 64 from `+0xA0`) is probed, and a row the probe rejects is looked up and
/// handed to the use hook when the lookup finds a live entry.
///
/// When the new cursor has reached the count at `+0x904` the step ends with
/// 0. Otherwise a bound link slot (`+0x166C` naming the manager's slot at
/// `+0xC6C`) is serviced first, then the current row is probed: a row the
/// probe accepts either exchanges the flag word when the manager reports
/// ready or runs the retry hook (0 when that refuses), while a row the
/// probe rejects is looked up and, when live and unflagged, published with
/// the notify hook. Every success path copies the row's key pair into the
/// manager's pending pair and returns 1; every other path returns 0.
/// Original: 0x00663d90 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00663D90(this: u32) -> u32 {
    unsafe { sn_migrate_advance(this) }
});

unsafe fn sn_migrate_advance(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x60;
        const ROW_BASE: u32 = 0xA0;
        const ROW_STRIDE: u32 = 64;
        const FLAG_BASE: u32 = 0x8E0;
        const LINK: u32 = 0x1664;
        const PUBLISH_TAG: u32 = 0x2EE0;
        const C_PROBE: u32 = 1;
        const C_LOOKUP: u32 = 2;
        const C_USE: u32 = 3;
        const C_UNLINK: u32 = 4;
        const C_RELINK: u32 = 5;
        const C_READY: u32 = 6;
        const C_XCHG: u32 = 7;
        const C_TRY: u32 = 8;
        const C_PUBLISH: u32 = 9;
        const C_NOTIFY: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mgr = rd32(this + MGR);
        wr32(mgr + 0xC30, 0xffff_ffff);
        wr32(mgr + 0xC34, 0xffff_ffff);
        let old = rd32(this + 0x908) as i32;
        wr32(this + 0x908, old.wrapping_add(1) as u32);
        let state = rd32(mgr + 0x50) as i32;
        if state < 2 || state > 3 {
            return 0;
        }
        if old >= 0 {
            let prev = this.wrapping_add((old as u32) << 6);
            let probe: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, prev + ROW_BASE);
            if probe & 0xff == 0 {
                let found: u32 = lf_checker_rt::callee_thiscall!(
                    C_LOOKUP, u32, mgr, rd32(prev + 0xD8), rd32(prev + 0xDC)
                );
                if found != 0 && rd32(found) as i32 >= 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_USE, u32, mgr, found);
                }
            }
        }
        let cur = rd32(this + 0x908);
        if cur as i32 >= rd32(this + 0x904) as i32 {
            return 0;
        }
        let link = this + LINK;
        let slot = mgr + 0xC6C;
        if rd32(this + 0x166C) != 0 && slot == rd32(this + 0x166C) && rd32(slot) != 0 {
            wr32(link + 8, 0);
            if rd32(link + 0x14) != 0 {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(C_UNLINK, u32, rd32(slot), link + 0xC);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(C_RELINK, u32, slot + 4, link);
        }
        let row = this.wrapping_add(ROW_BASE).wrapping_add(cur << 6);
        let probe: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, row);
        if probe & 0xff != 0 {
            let ready: u32 = lf_checker_rt::callee_thiscall!(C_READY, u32, mgr);
            if ready & 0xff != 0 {
                let _: u32 = lf_checker_rt::callee_stdcall!(C_XCHG, u32, this + 0x94, 3);
                wr32(this + 0x98, 0);
                wr32(this + 0x90, 1);
            } else {
                let retry: u32 = lf_checker_rt::callee_thiscall!(C_TRY, u32, this, row, 0);
                if retry & 0xff == 0 {
                    return 0;
                }
                wr32(this + 0x90, 1);
            }
        } else {
            let found: u32 = lf_checker_rt::callee_thiscall!(
                C_LOOKUP, u32, mgr, rd32(row + 0x38), rd32(row + 0x3C)
            );
            if found == 0 || (rd32(found) as i32) < 0 {
                return 0;
            }
            if ((cur.wrapping_add(this).wrapping_add(FLAG_BASE)) as *const u8).read() != 0 {
                return 0;
            }
            wr32(this + 0x1694, PUBLISH_TAG);
            wr32(this + 0x90, 3);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_PUBLISH, u32, mgr + 0xC6C, link, rd32(mgr + 0x32F4)
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, this + 0x94);
        }
            wr32(mgr + 0xC30, rd32(row + 0x38));
        wr32(mgr + 0xC34, rd32(row + 0x3C));
        1
    }
}
