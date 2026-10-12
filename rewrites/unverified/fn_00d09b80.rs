// original: 0x00D09B80 CTaskComplexCombat::vf19

/// Run one combat-task tick: pose, sub-ticks, pick and fire a subtask.
///
/// Returns zero when no target is locked (`+0x3c`). Otherwise marks the ped
/// in-combat, snapshots the target pose into `+0x80`, runs the three
/// housekeeping steps (patched thiscall callees), and unless the hold bit
/// (bit 2 of `+0xc`) is set, snapshots the ped pose into `+0x20`. Then
/// registers the target (patched thiscall callee), picks a subtask id
/// (patched thiscall callee, retried once with kind 0 on id 200, remapped to
/// 1901 on a repeated 200), builds it (patched thiscall callee) and starts it
/// (patched thiscall callee), clears the ped's alert byte and returns the
/// start answer.
///
/// Original: 0x00D09B80 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d09b80(this: u32, a1: u32) -> u32 {
    unsafe {
        const POSE: u32 = 1;
        const SENSE: u32 = 2;
        const THINK: u32 = 3;
        const REGISTER: u32 = 4;
        const PICK_A: u32 = 5;
        const PICK_B: u32 = 6;
        const BUILD: u32 = 7;
        const START: u32 = 8;
        const LOCK: u32 = 0x3c;
        const HOLD_BIT: u32 = 4;
        const IN_COMBAT: u32 = 0x29c;
        const ALERT: u32 = 0xA6D;
        const RETRY_ID: u32 = 200;
        const REMAP_ID: u32 = 1901;
        let lock = ((this + LOCK) as *const u32).read_unaligned();
        if lock == 0 {
            return 0;
        }
        ((a1 + IN_COMBAT) as *mut u32).write_unaligned(
            ((a1 + IN_COMBAT) as *const u32).read_unaligned() | 0x40000000);
        let anchor = (((lock + 0x20) as *const u32).read_unaligned() + 0x30) as u32;
        let mut o = 0u32;
        while o < 4 {
            ((this + 0x80 + o * 4) as *mut u32).write_unaligned(
                ((anchor + o * 4) as *const u32).read_unaligned());
            o += 1;
        }
        lf_checker_rt::callee_thiscall!(POSE, u32, this, a1);
        lf_checker_rt::callee_thiscall!(SENSE, u32, this, a1);
        lf_checker_rt::callee_thiscall!(THINK, u32, this, a1);
        if ((this + 0x0c) as *const u32).read_unaligned() & HOLD_BIT == 0 {
            let ped = (((a1 + 0x20) as *const u32).read_unaligned() + 0x30) as u32;
            let mut o = 0u32;
            while o < 4 {
                ((this + 0x20 + o * 4) as *mut u32).write_unaligned(
                    ((ped + o * 4) as *const u32).read_unaligned());
                o += 1;
            }
        }
        let reg = ((a1 + 0x224) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(REGISTER, u32, reg, lock, 0);
        let mut id = lf_checker_rt::callee_thiscall!(PICK_A, u32, this, a1, 1);
        if id == RETRY_ID {
            id = lf_checker_rt::callee_thiscall!(PICK_B, u32, this, a1, 0);
            if id == RETRY_ID {
                id = REMAP_ID;
            }
        }
        let built = lf_checker_rt::callee_thiscall!(BUILD, u32, this, a1, id);
        let started = lf_checker_rt::callee_thiscall!(START, u32, this, built, a1);
        ((a1 + ALERT) as *mut u8).write(0);
        started
    }
});
