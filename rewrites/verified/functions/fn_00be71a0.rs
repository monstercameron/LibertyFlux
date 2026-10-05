// original: 0x00be71a0 CTaskSimpleMovePathfindProblem::vf17

/// Age a move-pathfind-problem task and report whether it just expired.
///
/// `this` points to the task, `ped` to the ped. Scans the nav object at
/// `ped+0xa80` with three zero words (callee 1, thiscall) and copies the
/// four position dwords at `pos+0x30` (where `pos` is the object at
/// `ped+0x20`) to `this+0x20`. When the stamped flag at `+0x44` is clear and
/// the period at `+0x30` is non-negative, stamps the clock (dword at file
/// address `0x11735B4`) at `+0x3c`, the period at `+0x40` and the flag, then
/// notifies the ped of a clear wait (callee 2, thiscall, one word holding 0)
/// unless bit 2 of the ped flags at `+0x26c` is set; otherwise notifies of a
/// set wait (callee 3, thiscall, one word holding 1). Returns 0 when the
/// done flag at `+0x38` is set or the task was never stamped; reloads the
/// stamp from the clock when the re-stamp flag at `+0x45` is set; returns 1
/// exactly when the signed sum of the period and the stamp no longer exceeds
/// the clock.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be71a0(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_POS: u32 = 0x20;
        const PED_FLAGS: u32 = 0x26c;
        const PED_NAV: u32 = 0xa80;
        const POS_VEC: u32 = 0x30;
        const OFF_VEC: u32 = 0x20;
        const OFF_PERIOD: u32 = 0x30;
        const OFF_DONE: u32 = 0x38;
        const OFF_STAMP: u32 = 0x3c;
        const OFF_SAVED_PERIOD: u32 = 0x40;
        const OFF_STAMPED: u32 = 0x44;
        const OFF_RESTAMP: u32 = 0x45;
        const CLOCK_GLOBAL: u32 = 0x11735B4;
        const WAIT_BIT: u8 = 4;
        const SCAN: u32 = 1;
        const NOTIFY_CLEAR: u32 = 2;
        const NOTIFY_SET: u32 = 3;

        let nav = ((ped + PED_NAV) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(SCAN, u32, nav, 0, 0, 0);
        let pos = ((ped + PED_POS) as *const u32).read_unaligned();
        for i in 0..4u32 {
            let w = ((pos + POS_VEC + i * 4) as *const u32).read_unaligned();
            ((this + OFF_VEC + i * 4) as *mut u32).write_unaligned(w);
        }
        let period = ((this + OFF_PERIOD) as *const u32).read_unaligned();
        if ((this + OFF_STAMPED) as *const u8).read() == 0 && (period as i32) >= 0 {
            let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
            ((this + OFF_STAMP) as *mut u32).write_unaligned(now);
            ((this + OFF_SAVED_PERIOD) as *mut u32).write_unaligned(period);
            ((this + OFF_STAMPED) as *mut u8).write(1);
            if ((ped + PED_FLAGS) as *const u8).read() & WAIT_BIT == 0 {
                lf_checker_rt::callee_thiscall!(NOTIFY_CLEAR, u32, ped, 0);
            }
        } else {
            lf_checker_rt::callee_thiscall!(NOTIFY_SET, u32, ped, 1);
        }
        if ((this + OFF_DONE) as *const u8).read() != 0 {
            return 0;
        }
        if ((this + OFF_STAMPED) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + OFF_RESTAMP) as *const u8).read() != 0 {
            let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
            ((this + OFF_STAMP) as *mut u32).write_unaligned(now);
            ((this + OFF_RESTAMP) as *mut u8).write(0);
        }
        let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
        let saved = ((this + OFF_SAVED_PERIOD) as *const u32).read_unaligned();
        let stamp = ((this + OFF_STAMP) as *const u32).read_unaligned();
        if (saved.wrapping_add(stamp) as i32) <= (now as i32) {
            1
        } else {
            0
        }
    }
});
