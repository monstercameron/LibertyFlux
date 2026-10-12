// original: 0x0089A000 audio_start_child_and_count (proposed)

/// Start a voice's child sound and count the start.
///
/// `this` points to the voice: child slot byte at `+0x48` (0xff means none)
/// and group byte at `+0x40`. The global at 0x0115D988 is the group table
/// base, the one at 0x0115D964 the slot stride. With no child the start count
/// at `+0xd0` is incremented and the untouched entry eax returned (pinned by
/// the proof). Otherwise the child pointer (`stride * slot +
/// group_table[group]`) is resolved; a null pointer increments the count and
/// returns 0, a live one is started through it (patched callee, with `arg`),
/// the count incremented, and the callee's result returned. The re-check of
/// the slot byte after a live pointer is dead (the entry check excluded 0xff)
/// and mirrored as written.
///
/// Original: 0x0089A000 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_0089a000(this: u32, arg: u32) -> u32 {
    unsafe {
        const ENTRY_EAX: u32 = 0xa5a5_a5a5;
        const SLOT_IDX: u32 = 0x48;
        const GROUP_IDX: u32 = 0x40;
        const START_COUNT: u32 = 0xd0;
        const GROUPS_SLOT: u32 = 0x0115_d988;
        const STRIDE_SLOT: u32 = 0x0115_d964;
        const GROUP_STRIDE: u32 = 0x6f40;
        const GROUP_TABLE_OFF: u32 = 0x6f10;
        const NO_SLOT: u32 = 0xff;
        const CHILD_START: u32 = 1;

        let slot = ((this + SLOT_IDX) as *const u8).read() as u32;
        if slot == NO_SLOT {
            let c = ((this + START_COUNT) as *const u32).read_unaligned();
            ((this + START_COUNT) as *mut u32).write_unaligned(c.wrapping_add(1));
            return ENTRY_EAX;
        }
        let group = ((this + GROUP_IDX) as *const u8).read() as u32;
        let groups = lf_checker_rt::global::<u32>(GROUPS_SLOT).read();
        let stride = lf_checker_rt::global::<u32>(STRIDE_SLOT).read();
        let table = ((group
            .wrapping_mul(GROUP_STRIDE)
            .wrapping_add(groups)
            .wrapping_add(GROUP_TABLE_OFF)) as *const u32)
            .read_unaligned();
        let child = stride.wrapping_mul(slot).wrapping_add(table);
        if child == 0 {
            let c = ((this + START_COUNT) as *const u32).read_unaligned();
            ((this + START_COUNT) as *mut u32).write_unaligned(c.wrapping_add(1));
            return 0;
        }
        let target = if slot == NO_SLOT { 0 } else { child };
        let ret = lf_checker_rt::callee_thiscall!(CHILD_START, u32, target, arg);
        let c = ((this + START_COUNT) as *const u32).read_unaligned();
        ((this + START_COUNT) as *mut u32).write_unaligned(c.wrapping_add(1));
        ret
    }
});
