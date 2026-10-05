// original: 0x00b76920 anim_state_activate (proposed)

/// Activate the animation state at `this`, unless already active.
///
/// Returns at once when the mode byte at `this+0xbc8` is already set. When
/// the owner at `this+0x964` is null, records mode 1 and returns. Otherwise,
/// unless the initialised flag at `this+0xbcc` is clear, resolves three
/// values through the child at `this+0x98c` (call one answer into
/// `this+0x994`) and passes the last two plus the constant table 0xeb2450,
/// 0xff and `this+0xab8` to the apply callee (cdecl, five stack words);
/// then runs the finish callee (thiscall on the owner) and records mode 3
/// (mode 1 when there was no owner). Returns nothing meaningful.
///
/// Original: 0x00b76920 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b76920(this: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0xbc8;
        const OWNER_OFF: u32 = 0x964;
        const READY_OFF: u32 = 0xbcc;
        const CHILD_OFF: u32 = 0x98c;
        const FIRST_OFF: u32 = 0x994;
        const BLOCK_OFF: u32 = 0xab8;
        const TABLE_FILE_VA: u32 = 0xeb2450;
        const RESOLVE1: u32 = 1;
        const RESOLVE2: u32 = 2;
        const RESOLVE3: u32 = 3;
        const APPLY: u32 = 4;
        const FINISH: u32 = 5;
        if ((this + MODE_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
        if owner == 0 {
            ((this + MODE_OFF) as *mut u8).write(1);
            return 0;
        }
        if ((this + READY_OFF) as *const u8).read() != 0 {
            let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
            let first: u32 = lf_checker_rt::callee_thiscall!(RESOLVE1, u32, child);
            ((this + FIRST_OFF) as *mut u32).write_unaligned(first);
            let second: u32 = lf_checker_rt::callee_thiscall!(RESOLVE2, u32, child);
            let third: u32 = lf_checker_rt::callee_thiscall!(RESOLVE3, u32, child, second);
            let table = lf_checker_rt::relocated(TABLE_FILE_VA);
            // Push order in the original is value, table, 0xff, block, so
            // the callee's first word is the block: C order is
            // (block, 0xff, table, value).
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, this + BLOCK_OFF, 0xff, table, third);
            let _: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, owner);
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, owner);
        }
        ((this + MODE_OFF) as *mut u8).write(3);
        0
    }
});
