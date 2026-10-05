// original: 0x00be77a0 CTaskSimplePathfindProblem::vf17

/// Age a pathfind-problem task and report whether it just expired.
///
/// `this` points to the task, `ped` to the ped. Copies the four position
/// dwords at `pos+0x30` (where `pos` is the object at `ped+0x20`) to
/// `this+0x20`. When the stamped flag at `+0x40` is clear and the period at
/// `+0x30` is non-negative, stamps the clock (dword at file address
/// `0x11735B4`) at `+0x38`, the period at `+0x3c` and the flag. Notifies the
/// ped of the wait (callee 1 holding 1, then callee 2 holding 0, both
/// thiscall with one word) on every path except a freshly stamped task whose
/// ped flags at `+0x26c` have bit 2 set. Returns 0 when the done flag at
/// `+0x44` is set or the task was never stamped; reloads the stamp from the
/// clock when the re-stamp flag at `+0x41` is set; returns 1 exactly when the
/// signed sum of the period and the stamp no longer exceeds the clock.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be77a0(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_POS: u32 = 0x20;
        const PED_FLAGS: u32 = 0x26c;
        const POS_VEC: u32 = 0x30;
        const OFF_VEC: u32 = 0x20;
        const OFF_PERIOD: u32 = 0x30;
        const OFF_STAMP: u32 = 0x38;
        const OFF_SAVED_PERIOD: u32 = 0x3c;
        const OFF_STAMPED: u32 = 0x40;
        const OFF_RESTAMP: u32 = 0x41;
        const OFF_DONE: u32 = 0x44;
        const CLOCK_GLOBAL: u32 = 0x11735B4;
        const WAIT_BIT: u8 = 4;
        const NOTIFY_SET: u32 = 1;
        const NOTIFY_CLEAR: u32 = 2;

        let pos = ((ped + PED_POS) as *const u32).read_unaligned();
        for i in 0..4u32 {
            let w = ((pos + POS_VEC + i * 4) as *const u32).read_unaligned();
            ((this + OFF_VEC + i * 4) as *mut u32).write_unaligned(w);
        }
        let period = ((this + OFF_PERIOD) as *const u32).read_unaligned();
        let fresh = ((this + OFF_STAMPED) as *const u8).read() == 0 && (period as i32) >= 0;
        if fresh {
            let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
            ((this + OFF_STAMP) as *mut u32).write_unaligned(now);
            ((this + OFF_SAVED_PERIOD) as *mut u32).write_unaligned(period);
            ((this + OFF_STAMPED) as *mut u8).write(1);
        }
        if !fresh || ((ped + PED_FLAGS) as *const u8).read() & WAIT_BIT == 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY_SET, u32, ped, 1);
            lf_checker_rt::callee_thiscall!(NOTIFY_CLEAR, u32, ped, 0);
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
