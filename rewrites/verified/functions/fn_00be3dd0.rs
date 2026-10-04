// original: 0x00be3dd0 task_advance_indexed_child_hi (proposed)

/// Advance the child cursor, then dispatch to the selected child.
///
/// `holder_a` (second stack word; the first is unread) is bumped first and
/// its new value drives everything. When the repeat count at
/// `this + REPEAT_OFF` (0x68) is zero this is single-shot: a bumped value of
/// `WRAP_AT` (0x10) or an empty child slot returns zero, otherwise the child
/// at `this + TABLE_OFF + value * 4` (0x24) runs through its second virtual
/// and its answer is returned.
///
/// In repeat mode (`holder_b`, the third stack word, counts the passes) a
/// bumped value of `WRAP_AT` or an empty slot resets `*holder_a` to zero and
/// bumps `*holder_b` instead; then, unless the repeat count is 1, a pass
/// count that has reached the repeat count returns zero. Otherwise the (maybe
/// reset) cursor dispatches with no empty check: a null child faults, exactly
/// as the original does.
///
/// Original: 0x00be3dd0 (thiscall, three stack words: unread, holder_a,
/// holder_b).
lf_checker_rt::export!(thiscall, rw_00be3dd0(this: u32, _a0: u32, holder_a: u32, holder_b: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x24;
        const REPEAT_OFF: u32 = 0x68;
        const VF1_SLOT: u32 = 0x04;
        const WRAP_AT: u32 = 0x10;
        let bumped = (holder_a as *const u32)
            .read_unaligned()
            .wrapping_add(1);
        (holder_a as *mut u32).write_unaligned(bumped);
        let repeat = (this.wrapping_add(REPEAT_OFF) as *const u32).read_unaligned();
        if repeat == 0 {
            if bumped == WRAP_AT {
                return 0;
            }
            let child = (this
                .wrapping_add(TABLE_OFF)
                .wrapping_add(bumped.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if child == 0 {
                return 0;
            }
            let vtable = (child as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(VF1_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            return f(child);
        }
        if bumped == WRAP_AT
            || (this
                .wrapping_add(TABLE_OFF)
                .wrapping_add(bumped.wrapping_mul(4)) as *const u32)
                .read_unaligned()
                == 0
        {
            (holder_a as *mut u32).write_unaligned(0);
            let passes = (holder_b as *const u32).read_unaligned().wrapping_add(1);
            (holder_b as *mut u32).write_unaligned(passes);
        }
        if repeat != 1 {
            let passes = (holder_b as *const u32).read_unaligned();
            if passes == repeat {
                return 0;
            }
        }
        let cursor = (holder_a as *const u32).read_unaligned();
        let child = (this
            .wrapping_add(TABLE_OFF)
            .wrapping_add(cursor.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let vtable = (child as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(VF1_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        f(child)
    }
});
