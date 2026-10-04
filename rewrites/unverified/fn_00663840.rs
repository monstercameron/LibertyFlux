// original: 0x00663840 rage::snMigrateSessionTask::vf7

/// Close the session-migration task and emit its final report.
///
/// `this` is the task; `a0` selects the report shape and `a1` is forwarded.
/// The close services a bound link slot, scans the peer slots (calling the
/// scan hook for each live one and counting them), and, when the manager
/// state matches twice in a row, sets the task's bit in the manager's bitset.
/// A set flag word then either synchronises through the manager or settles
/// via an atomic compare-exchange. The teardown hook runs with both
/// arguments, the manager link is cleared, and a 92-byte report is built on
/// the scratch area — either the indexed row copied field by field with its
/// tag, or a zeroed report with -1 markers — and handed to the emit hook
/// together with the manager.
/// Original: 0x00663840 (thiscall, two stack arguments, no result).
lf_checker_rt::export!(thiscall, rw_00663840(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe { sn_migrate_close(this, a0, a1, 0) }
});

lf_checker_rt::export!(thiscall, mut_00663840(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe { sn_migrate_close(this, a0, a1, 1) }
});

lf_checker_rt::export!(thiscall, mutstruct_00663840(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe { sn_migrate_close(this, a0, a1, 2) }
});

unsafe fn sn_migrate_close(this: u32, a0: u32, a1: u32, mode: u32) -> u32 {
    unsafe {
        const REPORT_TAG: u32 = 0x00FE_32B4;
        const C_COOKIE: u32 = 8;
        const SLOT_STRIDE: u32 = 0x68;
        const C_UNLINK: u32 = 1;
        const C_RELINK: u32 = 2;
        const C_SCAN: u32 = 3;
        const C_SYNC: u32 = 4;
        const C_CAS: u32 = 5;
        const C_TEARDOWN: u32 = 6;
        const C_EMIT: u32 = 7;

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
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mgr = rd32(this + 0x60);
        let link = this + 0x1664;
        let slot = mgr + 0xC6C;
        if rd32(this + 0x166C) != 0 && slot == rd32(this + 0x166C) && rd32(slot) != 0 {
            wr32(link + 8, 0);
            if rd32(link + 0x14) != 0 {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(C_UNLINK, u32, rd32(slot), link + 0xC);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(C_RELINK, u32, slot + 4, link);
        }
        let total = rd32(this + 0x1658) as i32;
        if total > 0 {
            let mut s = 0u32;
            while (s as i32) < rd32(this + 0x1658) as i32 {
                let entry = this + 0x958 + s * SLOT_STRIDE;
                if rd32(entry + 8) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_SCAN, u32, entry);
                }
                s += 1;
            }
            let _scan_count = s;
        }
        // The match is checked twice with identical reads; the second
        // failure arm is unreachable but kept as the original orders it.
        let st = rd32(mgr + 0x50) as i32;
        let matched = st >= 2
            && st <= 3
            && rd32(mgr + 0xBF0) == rd32(mgr + 0xC30)
            && rd32(mgr + 0xBF4) == rd32(mgr + 0xC34);
        if matched {
            let st2 = rd32(mgr + 0x50) as i32;
            let matched2 = st2 >= 2
                && st2 <= 3
                && rd32(mgr + 0xBF0) == rd32(mgr + 0xC30)
                && rd32(mgr + 0xBF4) == rd32(mgr + 0xC34);
            if matched2 {
                let n = rd32(mgr + 0x32F4) as i32;
                let cell = rd32(mgr + 0x24)
                    .wrapping_add(0x68C)
                    .wrapping_add(((n >> 5) as u32).wrapping_mul(4));
                wr32(cell, rd32(cell) | (1u32 << ((n as u32) & 0x1F)));
                let flag = (mgr + 0x32F8) as *mut u8;
                flag.write(flag.read() | 2);
            } else {
                let flag = (mgr + 0x32F8) as *mut u8;
                flag.write(flag.read() & 0xFD);
            }
        }
        if rd32(this + 0x94) == 1 {
            if a0 == 2 {
                if rd32(this + 0x90) == 1 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_SYNC, u32, mgr + 0x48, this + 0x94
                    );
                } else {
                    let prev: u32 =
                        lf_checker_rt::callee_stdcall!(C_CAS, u32, this + 0x94, 4, 1);
                    if prev == 1 {
                        wr32(this + 0x98, 0xffff_ffff);
                    }
                }
            } else {
                let prev: u32 =
                    lf_checker_rt::callee_stdcall!(C_CAS, u32, this + 0x94, 2, 1);
                if prev == 1 {
                    wr32(this + 0x98, 0xffff_ffff);
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_TEARDOWN, u32, this, a0, a1);
        let clear = (mgr + 0x32F8) as *mut u8;
        clear.write(clear.read() & 0xF7);
        if mode != 1 {
            wr32(this + 0x60, 0);
        }
        // 92-byte report. Byte pairs at +0x2A, +0x32, +0x3A and +0x51 are
        // padding the original never writes; they keep scratch values here.
        let mut rep = [0u8; 92];
        let base = rep.as_mut_ptr() as u32;
        wr32(base, lf_checker_rt::relocated(REPORT_TAG));
        wr32(base + 0x08, 0);
        wr32(base + 0x0C, 0);
        if a0 == 1 {
            let row = this
                .wrapping_add(0xA0)
                .wrapping_add(rd32(this + 0x908) << 6);
            wr32(base + 0x10, rd32(row));
            wr32(base + 0x14, rd32(row + 4));
            wr32(base + 0x18, rd32(row + 8));
            wr32(base + 0x1C, rd32(row + 12));
            wr32(base + 0x20, rd32(row + 0x10));
            wr32(base + 0x24, rd32(row + 0x14));
            wr16(base + 0x28, rd16(row + 0x18));
            wr32(base + 0x2C, rd32(row + 0x1C));
            wr16(base + 0x30, rd16(row + 0x20));
            wr32(base + 0x34, rd32(row + 0x24));
            wr16(base + 0x38, rd16(row + 0x28));
            wr32(base + 0x3C, rd32(row + 0x2C));
            wr32(base + 0x40, rd32(row + 0x30));
            wr32(base + 0x44, rd32(row + 0x34));
            wr32(base + 0x48, rd32(row + 0x38));
            wr32(base + 0x4C, rd32(row + 0x3C));
            wr8(base + 0x50, 1);
            if mode == 2 {
                wr32(base + 0x14, rd32(base + 0x14).wrapping_add(1));
            }
        } else {
            wr32(base + 0x10, 0);
            wr32(base + 0x14, 0);
            wr32(base + 0x18, 0);
            wr32(base + 0x1C, 0);
            wr16(base + 0x28, 0);
            wr16(base + 0x30, 0);
            wr16(base + 0x38, 0);
            wr32(base + 0x3C, 0);
            wr8(base + 0x50, 0);
            wr32(base + 0x20, 0);
            wr32(base + 0x24, 0xffff_ffff);
            wr32(base + 0x2C, 0xffff_ffff);
            wr32(base + 0x34, 0xffff_ffff);
            wr32(base + 0x40, 0);
            wr32(base + 0x44, 0);
            wr32(base + 0x48, 0xffff_ffff);
            wr32(base + 0x4C, 0xffff_ffff);
            if mode == 2 {
                wr8(base + 0x50, 1);
            }
        }
        wr32(base + 0x04, base);
        wr32(base + 0x54, rd32(this + 0x904));
        wr32(base + 0x58, rd32(this + 0x908));
        // The emit hook's register argument is the manager, not the scan
        // count: the scratch store ahead of the teardown call lands on the
        // loop counter's slot and overwrites it.
        let _: u32 = lf_checker_rt::callee_thiscall!(C_EMIT, u32, mgr, base);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
}
